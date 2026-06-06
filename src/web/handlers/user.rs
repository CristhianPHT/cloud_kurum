use actix_web::{get, HttpRequest, web, HttpResponse, Responder};
use serde_json::json;

// use crate::schema::usuario::username;
use crate::web::auth::extractor::get_user_id_from_token;
use crate::infrastructure::db::establish_connection;
use crate::{select_id_usuario, select_usuario_por_username, select_header_user};

#[get("/me")] // Select username,perfil,nickname from Account = select_id_usuario = sacamos data minima por id
pub async fn get_user(req: HttpRequest) -> impl Responder {
  let user_id :i32 = match get_user_id_from_token(&req) {
    Ok(id) => id,
    Err(response) => return response,
  };

  let mut conn = establish_connection();

  match select_id_usuario(&mut conn, user_id) {
    Ok(usuario) => HttpResponse::Ok().json(usuario),
    Err(_) => HttpResponse::InternalServerError()
      .json(json!({ "error": "No se pudo obtener el usuario" })),
  }
}

#[get("/user/{username_link}")]
pub async fn get_user_page(username_link: web::Path<String>) -> impl Responder {
  let usuariox = username_link.into_inner();
  let mut conn = establish_connection();
  match select_usuario_por_username(&mut conn, &usuariox) {
    Ok(datos) => HttpResponse::Ok().json(json!({ "data": datos })), // Vec<Account>
    Err(_) => HttpResponse::InternalServerError().json(json!({ "error": "Error al obtener los datos del usuario" })), // Result<diesel::result::Error>
  }
}

#[get("/me/header")]
pub async fn get_header(req: HttpRequest) -> impl Responder {
  let user_id :i32 = match get_user_id_from_token(&req) {
    Ok(id) => id,
    Err(response) => return response,
  };

  let mut conn = establish_connection();
  match select_header_user(&mut conn, user_id) {
    Ok(usuario) => HttpResponse::Ok().json(usuario),
    Err(_) => HttpResponse::InternalServerError()
      .json(json!({ "error": "No se pudo obtener el usuario" }))
  }
}