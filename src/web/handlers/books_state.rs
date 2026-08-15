use actix_web::{get, post, put, delete, web, HttpResponse, Responder};
use serde_json::json;

use crate::infrastructure::db::establish_connection;

// ------------------------------------------- Libro - Estado -------------------------------------------
use crate::model::libro_estados::{NewLibroTipo, LibroTipo};
use crate::repositories::libro_estados::{insert_lib_tip_new, select_lib_tip, select_lib_tip_all, update_lib_tip, delete_lib_tip};

#[post("/books/tipe")]
pub async fn post_nuevo_lib_tip(param: web::Json<NewLibroTipo>) -> impl Responder {
  let mut conn = establish_connection();
  let libro_tipo = param.into_inner();
  match insert_lib_tip_new(&mut conn, libro_tipo) {
    Ok(id) => HttpResponse::Ok().json(json!({ "libro tipo id": id })),
    Err(_) => HttpResponse::InternalServerError().json(json!({ "error": "Error generar nuevos libros" }))
  }
}

#[get("/books/tipe/{id}")]
pub async fn get_lib_tip_unique(id: web::Path<i32>) -> impl Responder {  // falta dto
  let mut conn = establish_connection();
  let libro_tipo_id = id.into_inner();
  match select_lib_tip(&mut conn, libro_tipo_id) {
    Ok(tipo_id) => HttpResponse::Ok().json(json!({ "data id": tipo_id })), // Vec<LibroDashboard>
    Err(_) => HttpResponse::InternalServerError()
      .json(json!({ "error": "Error al obtener los libros" })), // Result<diesel::result::Error>
  }
}

#[get("/books/tipe/all/{pagina}")]   // 
pub async fn get_lib_tip_all(pagina: web::Path<i64>) -> impl Responder {
  let mut conn = establish_connection();
  let pagina: i64 = pagina.into_inner();
  match select_lib_tip_all(&mut conn, pagina) {
    Ok(libros) => HttpResponse::Ok().json(json!({ "Libro tipos": libros })), // Vec<LibroDashboard>
    Err(_) => HttpResponse::InternalServerError().json(json!({ "error": "Error al obtener los libros" }))
  }
}

#[put("/books/tipe/{id}")]
pub async fn put_lib_tip( id: web::Path<i32>, name_tipo: web::Json<LibroTipo>) -> impl Responder {
  let mut conn = establish_connection();
  let identificacion = id.into_inner();
  let name_tipo_new = name_tipo.into_inner();
  match update_lib_tip(&mut conn, identificacion, name_tipo_new) {
    Ok(nombre) => HttpResponse::Ok().json(json!({ "Nombre actual": nombre})),
    Err(_) => HttpResponse::InternalServerError().json(json!({ "error": "Error al actualizar el nombre" }))
  }
}

#[delete("/books/tipe/{id}")]
pub async fn delete_lib_tip_web( id: web::Path<i32>) -> impl Responder {
  let mut conn = establish_connection();
  let identificacion = id.into_inner();
  match delete_lib_tip(&mut conn, identificacion) {
    Ok(_) => HttpResponse::Ok().finish(),
    Err(_) => HttpResponse::InternalServerError().finish()
  }
}

// ------------------------------------------- Libro - Tipo -------------------------------------------
use crate::repositories::libro_estados::{insert_lib_est_new, select_lib_est, select_lib_est_all, update_lib_est, delete_lib_est};
use crate::model::libro_estados::{LibroEstado, NewLibroEstado};

#[post("/books/state")]
pub async fn post_nuevo_lib_est( param: web::Json<NewLibroEstado> ) -> impl Responder {
  let mut conn = establish_connection();
  let libro_estado = param.into_inner();
  match insert_lib_est_new(&mut conn, libro_estado) {
    Ok(id) => HttpResponse::Ok()
      .json(json!({ "libro estado id": id })),
    Err(_) => HttpResponse::InternalServerError()
      .json(json!({ "error": "Error al generar nuevo estado" })),
  }
}

#[get("/books/state/{id}")]
pub async fn get_lib_est_unique( id: web::Path<i32> ) -> impl Responder {
  let mut conn = establish_connection();
  let libro_estado_id = id.into_inner();
  match select_lib_est(&mut conn, libro_estado_id) {
    Ok(estado) => HttpResponse::Ok()
      .json(json!({ "data": estado })),
    Err(_) => HttpResponse::InternalServerError()
      .json(json!({ "error": "Error al obtener el estado" })),
  }
}

#[get("/books/state/all/{pagina}")]
pub async fn get_lib_est_all(
  pagina: web::Path<i64> ) -> impl Responder {
  let mut conn = establish_connection();
  let pagina = pagina.into_inner();
  match select_lib_est_all(&mut conn, pagina) {
    Ok(estados) => HttpResponse::Ok()
      .json(json!({ "Libro estados": estados })),
    Err(_) => HttpResponse::InternalServerError()
      .json(json!({ "error": "Error al obtener los estados" })),
  } 
}

#[put("/books/state/{id}")]
pub async fn put_lib_est( id: web::Path<i32>, name_estado: web::Json<LibroEstado> ) -> impl Responder {
  let mut conn = establish_connection();
  let identificacion = id.into_inner();
  let name_estado_new = name_estado.into_inner();
  match update_lib_est(&mut conn, identificacion, name_estado_new) {
    Ok(estado) => HttpResponse::Ok()
      .json(json!({ "estado actual": estado })),
    Err(_) => HttpResponse::InternalServerError()
      .json(json!({ "error": "Error al actualizar el estado" })),
  }
}

#[delete("/books/state/{id}")]
pub async fn delete_lib_est_web( id: web::Path<i32> ) -> impl Responder {
  let mut conn = establish_connection();
  let identificacion = id.into_inner();

  match delete_lib_est(&mut conn, identificacion) {
    Ok(_) => HttpResponse::Ok().finish(),
    Err(_) => HttpResponse::InternalServerError().finish(),
  }
}