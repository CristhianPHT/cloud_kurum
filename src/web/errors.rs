use actix_web::{HttpResponse};
use crate::services::UserError;
use serde_json::json;
pub fn map_user_error(err: UserError) -> HttpResponse {
  match err {
    UserError::Validation(msg) => {
      HttpResponse::BadRequest().json(json!({ "error": msg }))
    }
    UserError::InvalidCredentials => {
      HttpResponse::Unauthorized().json(json!({
        "error": "Usuario o contraseña incorrectos"
      }))
    }
    UserError::EmailTaken => {
      HttpResponse::Conflict().json(json!({
        "error": "El email ya está en uso"
      }))
    }
    UserError::NicknameTaken => {
      HttpResponse::Conflict().json(json!({
        "error": "El nickname ya está en uso"
      }))
    }
    UserError::TokenError => {
      HttpResponse::InternalServerError().json(json!({
        "error": "Error al generar token"
      }))
    }
    UserError::DatabaseError | UserError::Internal => {   // falta mejoras para mí (programador)
      HttpResponse::InternalServerError().json(json!({
        "error": "Error interno del servidor"
      }))
    }
  }
}