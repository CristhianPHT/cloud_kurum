use actix_web::{get, post, web, HttpResponse, Responder};
use crate::select_libro_main;
use serde_json::json;

use crate::infrastructure::db::establish_connection;

// -----------------------------------------libros----------------------------------------------
use crate::{select_nombre_libros};
#[get("/books/{page}")]   // últimos 10 libros publicados (/api/books/{page})
pub async fn get_libro_all() -> impl Responder {
  let mut conn = establish_connection();
  // let user_id = id.into_inner();
  match select_nombre_libros(&mut conn) {
    Ok(libros) => HttpResponse::Ok().json(json!({ "libros": libros })), // Vec<LibroDashboard>
    Err(_) => HttpResponse::InternalServerError().json(json!({ "error": "Error al obtener los libros" })), // Result<diesel::result::Error>
  }
}
#[get("/books/{slug}")]   // quizá debería ser /books/{nickname}/{slug}?
pub async fn get_libro_unique(slug: web::Path<String>) -> impl Responder {  // falta dto
  let mut conn = establish_connection();
  let libro_slug = slug.into_inner();
  match select_libro_main(&mut conn, libro_slug) {
    Ok(libro) => HttpResponse::Ok().json(json!({ "data": libro })), // Vec<LibroDashboard>
    Err(_) => HttpResponse::InternalServerError()
      .json(json!({ "error": "Error al obtener los libros" })), // Result<diesel::result::Error>
  }
}
use crate::{insert_libro_nuevo};
use crate::models::NuevoLibro;
#[post{"/books"}]
pub async fn post_nuevo_libro(param: web::Json<NuevoLibro>) -> impl Responder {
  let mut conn = establish_connection();
  let nuevo_librito = param.into_inner();
  match insert_libro_nuevo(&mut conn, nuevo_librito) {    // falta verificar si ya hay para no duplicados
    Ok(id) => HttpResponse::Ok().json(json!({ "libro_id": id })), // QueryResult<i32>
    Err(_) => HttpResponse::InternalServerError().json(json!({ "error": "Error generar nuevos libros" })), // QueryResult<Error>
  }
}
