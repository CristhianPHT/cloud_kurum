use actix_web::{get, post, put, HttpRequest, web, HttpResponse, Responder};
use serde_json::json;

// use crate::schema::usuario::username;
use crate::web::auth::extractor::get_user_id_from_token;
use crate::infrastructure::db::establish_connection;
use crate::{select_usuario_por_id, select_usuario_por_nickname, select_me_header};

#[get("/me")] // Select perfil,nickname from Account = select_id_usuario = sacamos data minima por id
pub async fn get_user(req: HttpRequest) -> impl Responder {
  let user_id :i32 = match get_user_id_from_token(&req) {
    Ok(id) => id,
    Err(response) => return response,
  };

  let mut conn = establish_connection();

  match select_usuario_por_id(&mut conn, user_id) {
    Ok(usuario) => HttpResponse::Ok().json(usuario),
    Err(_) => HttpResponse::InternalServerError()
      .json(json!({ "error": "No se pudo obtener el usuario" })),
  }
}

#[get("/users/{nickname_link}")]
pub async fn get_user_page(nickname_link: web::Path<String>) -> impl Responder {
  let usuariox = nickname_link.into_inner();
  let mut conn = establish_connection();
  match select_usuario_por_nickname(&mut conn, &usuariox) {
    Ok(datos) => HttpResponse::Ok().json(json!({ "data": datos })), // Vec<HeaderAccount>
    Err(diesel::result::Error::NotFound) => {
      HttpResponse::NotFound().json(json!({
          "error": "Usuario no encontrado"
      }))
  }
  
  Err(_) => {
      HttpResponse::InternalServerError().json(json!({
          "error": "Error al obtener los datos del usuario"
      }))
  }
  }
}

#[get("/me/header")]
pub async fn get_header(req: HttpRequest) -> impl Responder {
  let user_id :i32 = match get_user_id_from_token(&req) {
    Ok(id) => id,
    Err(response) => return response,
  };

  let mut conn = establish_connection();
  match select_me_header(&mut conn, user_id) {
    Ok(usuario) => HttpResponse::Ok().json(usuario),
    Err(_) => HttpResponse::InternalServerError()
      .json(json!({ "error": "No se pudo obtener el usuario" }))
  }
}

use crate::web::dto::account::RegisterAccount;
use crate::services::account::{register_user};
use crate::web::errors::map_user_error;
#[post("/users/register")]
pub async fn insert_login(user: web::Json<RegisterAccount>) -> impl Responder {
  let mut conn = establish_connection();

  match register_user(&mut conn, user.into_inner()) {
    Ok(token) => HttpResponse::Ok().json(json!({ "token": token })),
    Err(e) => map_user_error(e),
  }
}

use crate::models::LoginAccount;
use crate::services::login_user_service;
#[post("/users/login")]
pub async fn login_usuario(user: web::Json<LoginAccount>) -> impl Responder {
  let mut conn = establish_connection();
  let login_data = user.into_inner();

  match login_user_service(&mut conn, login_data) {
    Ok(token) => HttpResponse::Ok().json(json!({ "token": token })),
    Err(e) => map_user_error(e),
  }
}

use crate::web::dto::account::ChangePassword;
use crate::services::update_password_service;
#[put("/me/password")]
pub async fn new_password(req: HttpRequest, user: web::Json<ChangePassword>) -> impl Responder {
  let user_id :i32 = match get_user_id_from_token(&req) {
    Ok(id) => id,
    Err(response) => return response,
  };
  let mut conn = establish_connection();
  let login_data = user.into_inner();
  match update_password_service(&mut conn, user_id, login_data) {
    Ok(()) => HttpResponse::Ok().json(json!({ "message": "Actualizado" })),
    Err(e) => map_user_error(e),
  }
}

use crate::web::auth::extractor::get_token_from_http;
use crate::repositories::delete_token;
#[put("/me/logout")]
pub async fn logout_sesion(req: HttpRequest) -> impl Responder {
  let token = match get_token_from_http(&req) {
    Ok(t) => t,
    Err(e) => return e,
  };
  let mut conn = establish_connection();
  match delete_token(&mut conn, &token) {
    Ok(_) => HttpResponse::Ok().json(json!({
      "message": "Token eliminado"
    })),
    Err(_) => HttpResponse::InternalServerError().json(json!({
      "error": "Error al eliminar el token del usuario"
    })),
  }
}


use crate::web::dto::account::UpdateNickname;
use crate::repositories::update_nickname;
#[put("/me/nickname")] // actualizar usuario
pub async fn actualizar_user(req: HttpRequest, new_users: web::Json<UpdateNickname>) -> impl Responder {
  let user_id :i32 = match get_user_id_from_token(&req) {
    Ok(id) => id,
    Err(response) => return response,
  };
  // falta ingresar esto al service por para la validación nickname...
  let mut conn = establish_connection();
  match update_nickname(&mut conn, user_id, new_users.into_inner()) {
    Ok(()) => HttpResponse::Ok().json(json!({
      "message": "Token eliminado"
    })),
    Err(_) => HttpResponse::InternalServerError().json(json!({
      "error": "Error al eliminar el token del usuario"
    })),
  }
}