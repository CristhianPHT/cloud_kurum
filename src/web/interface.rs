use crate::establish_connection;

use actix_web::{get,post,web, HttpResponse, Responder};

use crate::models::{NuevoGenero, NuevoLibroGenero}; // Libro, NuevoLibro Eliminados por no usarlos (warning)
use serde_json::json;


// ---------------------------------------------------------------------------------------------
// Generica
// #[post("/generica")]
// pub async fn insert_generica(user: web::Json<NuevoUsuario>) -> impl Responder {
//   let mut conn = establish_connection();
//   let nuevo_usuario = user.into_inner();
//   let _identidad = generic_insert(&mut conn, nuevo_usuario.clone());
//   HttpResponse::Ok().json(json!({
//       "usuario": nuevo_usuario
//   }))
// }

// ---------------------------------------------------------------------------------------------
// -----------------------------------------genero----------------------------------------------
use crate::insert_gen_new;
#[post("/nuevogenero")]
pub async fn post_nuevo_genero(param: web::Json<NuevoGenero>) -> impl Responder {
  let mut conn = establish_connection();
  let nuevo_genero = param.into_inner();
  match insert_gen_new(&mut conn, nuevo_genero) {
    Ok(id) => HttpResponse::Ok().json(json!({ "genero_id": id })), // QueryResult<i32>
    Err(_) => HttpResponse::InternalServerError().json(json!({ "error": "Error al ingresar un nuevo genero" })), // QueryResult<Error>
  }
}

use crate::{insert_libro_genero,buscar_libros_por_genero, OrdenamientoLibro};
#[post("/nuevolibroxgenero")]
pub async fn post_nuevo_libro_genero(param: web::Json<NuevoLibroGenero>) -> impl Responder {
  let mut conn = establish_connection();
  let data = param.into_inner();
  match insert_libro_genero(&mut conn, data) {
    Ok(diccionario) => HttpResponse::Ok().json(json!({"id": diccionario})),
    Err(_) => HttpResponse:: InternalServerError().json(json!({ "Error": "Error al ingresar los datos"}))
  }
}

#[get("/busqueda_libro_genero/{pagina}")]
pub async fn get_buscar_lib_gen( pagina: web::Path<i64>,  // Cambiado a i64 para coincidir con tu función get_buscar_lib_gen,post_nuevo_genero
) -> impl Responder {
    let mut conn = establish_connection();
    match buscar_libros_por_genero(&mut conn, 1, pagina.into_inner(), OrdenamientoLibro::TituloAsc) {
        Ok(busqueda) => HttpResponse::Ok().json(json!({ "libros": busqueda })), 
        Err(e) => {
            eprintln!("Error en búsqueda: {:?}", e); // Log simple en consola
            HttpResponse::InternalServerError().json(json!({ "error": "Falló la búsqueda" }))
        }
    }
}


// insert_libro_genero

// #[get("/generica/{id}")]
// pub async fn get_libro_data(id: web::Path<i32>) -> impl Responder {
//   let mut conn = establish_connection();
//   let user_id = id.into_inner();
//   let libro: Libro = select_by_id(libro, &mut conn, user_id);
//   HttpResponse::Ok().json(json!({
//     "libros": libro
//   }))
// }

// #[post("/generica")]
// pub async fn insert_libro(user: web::Json<NuevoLibro>) -> impl Responder {
//   let mut conn = establish_connection();
//   let nuevo_usuario = user.into_inner();
//   let _identidad = insert_user(&mut conn, nuevo_usuario.clone());
//   HttpResponse::Ok().json(json!({
//       "usuario": nuevo_usuario
//   }))
// }

