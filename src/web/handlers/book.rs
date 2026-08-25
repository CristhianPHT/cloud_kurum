use actix_web::{get, post, web, HttpResponse, Responder};
use crate::select_libro_main;
use serde_json::json;

use crate::infrastructure::db::establish_connection;

// -----------------------------------------libros----------------------------------------------
use crate::{select_nombre_libros};
#[get("/books/{page}")]   // últimos 10 libros publicados (/api/books/{page})
pub async fn get_libro_all(pagina: web::Path<i64>) -> impl Responder {
  let mut conn = establish_connection();
  let pagina: i64 = pagina.into_inner();
  match select_nombre_libros(&mut conn, pagina) {
    Ok(libros) => HttpResponse::Ok().json(json!({ "libros": libros })), // Vec<LibroDashboard>
    Err(_) => HttpResponse::InternalServerError().json(json!({ "error": "Error al obtener los libros" })), // Result<diesel::result::Error>
  }
}


// select_libro_main
#[get("/book/{slug}")]   // quizá debería ser /books/{nickname}/{slug}?
pub async fn get_libro_slug(slug: web::Path<String>) -> impl Responder {  // falta dto
  let mut conn = establish_connection();
  let libro_slug = slug.into_inner();
  match select_libro_main(&mut conn, libro_slug) {
    Ok(libro) => HttpResponse::Ok().json(json!({ "data": libro })), // Vec<LibroDashboard>
    Err(_) => HttpResponse::InternalServerError()
      .json(json!({ "error": "Error al obtener los libros" })), // Result<diesel::result::Error>
  }
}

use crate::web::dto::book::NewBook;
use crate::services::book::create_book_service;
// use crate::services::book::create_book_service;

#[post{"/books"}]
pub async fn post_nuevo_libro(param: web::Json<NewBook>) -> impl Responder {
  let mut conn = establish_connection();
  let nuevo_librito = param.into_inner();
  match create_book_service(&mut conn, nuevo_librito) {    // falta verificar si ya hay para no duplicados
    Ok(slug) => HttpResponse::Ok().json(json!({ "slug": slug })), // QueryResult<i32>
    Err(_) => HttpResponse::InternalServerError().json(json!({ "error": "Error generar nuevos libros" })), // QueryResult<Error>
  }
}

use crate::repositories::select_libro_detalle;
#[get("/bookid/{id}")]
pub async fn get_libro_unique(id: web::Path<i32>) -> impl Responder {  // falta dto
  let mut conn = establish_connection();
  let libro_id = id.into_inner();
  match select_libro_detalle(&mut conn, libro_id) {
    Ok(libro) => HttpResponse::Ok().json(json!({ "data": libro })), // Vec<LibroDashboard>
    Err(_) => HttpResponse::InternalServerError()
      .json(json!({ "error": "Error al obtener los libros" })), // Result<diesel::result::Error>
  }
}