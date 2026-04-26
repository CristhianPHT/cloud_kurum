use actix_web::{get, HttpRequest, web, HttpResponse, Responder};
use serde_json::json;

// use crate::schema::usuario::username;
use crate::web::auth::extractor::get_user_id_from_token;
use crate::infrastructure::db::establish_connection;
use crate::modules::account::select_id_usuario;
use crate::modules::select_usuario_por_username;

#[get("/user")]
pub async fn get_user(req: HttpRequest) -> impl Responder {
  let user_id = match get_user_id_from_token(&req) {
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

#[get("/usuario/{username_link}")]
pub async fn get_user_page(username_link: web::Path<String>) -> impl Responder {
  let usuariox = username_link.into_inner();
  let mut conn = establish_connection();
  match select_usuario_por_username(&mut conn, &usuariox) {
    Ok(datos) => HttpResponse::Ok().json(json!({ "data": datos })), // Vec<Account>
    Err(_) => HttpResponse::InternalServerError().json(json!({ "error": "Error al obtener los datos del usuario" })), // Result<diesel::result::Error>
  }
}