use crate::models::NuevoLibroUsuario;
use crate::repositories::{insert_libro_usuario, select_all_books_by_user, select_public_books_by_username};
use crate::web::auth::extractor::get_user_id_from_token;
use actix_web::{get, post, HttpResponse, web, HttpRequest, Responder};
use diesel::PgConnection;
// HttpRequest
use serde_json::json;

// use crate::schema::usuario::username;
// use crate::web::auth::extractor::get_user_id_from_token;
use crate::infrastructure::db::establish_connection;
use crate::repositories::select_books_by_user_images;

#[get("/me/libros")]
pub async fn get_all_books_user(req: HttpRequest) -> impl Responder {
  let user_id = match get_user_id_from_token(&req) {
    Ok(id) => id,
    Err(response) => return response,
  };
  let mut conn: PgConnection = establish_connection();
  match select_all_books_by_user(&mut conn, user_id) {
    Ok(vecto_libro) => HttpResponse::Ok().json(vecto_libro),
    Err(_) => HttpResponse::InternalServerError()
    .json(json!({"no hay relaciones...": "Error InternalServerError"})),
  }
}

#[get("/me/sufle_libro")] // Select username,perfil,nickname from Account = select_id_usuario = sacamos data minima por id
pub async fn get_books_x_user(req: HttpRequest) -> impl Responder { //  id_usuario: web::Path<i32>
  let user_id = match get_user_id_from_token(&req) {
    Ok(id) => id,
    Err(response) => return response,
  };
  // let user_id = id_usuario.into_inner(); // era para cosas sin tokens

  let mut conn = establish_connection();

  match select_books_by_user_images(&mut conn, user_id) {
    Ok(usuario) => HttpResponse::Ok().json(usuario),
    Err(_) => HttpResponse::InternalServerError()
      .json(json!({ "error": "No se pudo obtener el usuario" })),
  }
}

#[post("/me/libros")]
pub async fn post_books_x_user(param: web::Json<NuevoLibroUsuario>) -> impl Responder {  // req: HttpRequest
  let mut conn = establish_connection();
  let nuevo_librito = param.into_inner();
  match insert_libro_usuario(&mut conn, nuevo_librito) {
    Ok(id) => HttpResponse::Ok().json(json!({ "libro_id": id })), // QueryResult<i32>
    Err(_) => HttpResponse::InternalServerError().json(json!({ "error": "Error generar nuevos libros" })), // QueryResult<Error>
  }
}

#[get("/user/{username_link}/libros")]
pub async fn get_libros_publicos_x_user(username_link: web::Path<String> ) -> impl Responder {
  let usuariox = username_link.into_inner();
  let mut conn = establish_connection();

  match select_public_books_by_username(&mut conn, &usuariox) {
    Ok(libros) => HttpResponse::Ok().json(libros),
    // Err(_) => HttpResponse::InternalServerError().json(json!({ "error": "Error al obtener los datos del usuario" })), // Result<diesel::result::Error>
    Err(_) => HttpResponse::NotFound()
      .json(json!({ "error": "Usuario no encontrado o sin libros públicos" })),
  }
}

// link: "/me" = usuario
// link: "/libros", "/me/libros" = libros 
// link: "/" = insert new libro_usuario, tabla relacional
