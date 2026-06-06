use actix_web::{get, web, HttpResponse, Responder};
use crate::select_libro_main;
use serde_json::json;

use crate::infrastructure::db::establish_connection;

#[get("/users/{username}/books")] // malisimo en username, path, arreglarlo
pub async fn get_user_books(path: web::Path<String>) -> impl Responder {
  let mut conn = establish_connection();
  let libro_id = path.into_inner().parse::<i32>().unwrap();  // parse::<i32>() para convertir el String a i32
  match select_libro_main(&mut conn, libro_id) {
    Ok(libro) => HttpResponse::Ok().json(json!({ "data": libro })),
    Err(_) => HttpResponse::InternalServerError()
      .json(json!({ "error": "No se pudo obtener el libro" })),
  }
}
