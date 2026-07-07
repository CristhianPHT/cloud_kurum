use actix_web::{HttpRequest, HttpResponse};
use serde_json::json;

use crate::auth::select_id_token;
use crate::establish_connection;

pub fn get_token_from_http(req: &HttpRequest) -> std::result::Result<String, HttpResponse> {
  let header = req.headers().get("Authorization")
    .ok_or_else(|| {
      HttpResponse::Unauthorized().json(json!({
        "error": "Token no proporcionado"
      }))
    })?;
  let header_str = header.to_str().map_err(|_| {
    HttpResponse::BadRequest().json(json!({
      "error": "Encabezado inválido"
    }))
  })?;
  if !header_str.starts_with("Bearer ") {
    return Err(HttpResponse::BadRequest().json(json!({
      "error": "Formato inválido"
    })));
  }
  let token = header_str
    .strip_prefix("Bearer ")
    .unwrap()
    .trim();
  if token.is_empty() {
    return Err(HttpResponse::Unauthorized().json(json!({
      "error": "Token vacío"
    })));
  }
  Ok(token.to_string())
}


pub fn get_user_id_from_token(req: &HttpRequest) -> std::result::Result<i32, HttpResponse> {
  let token = match req.headers().get("Authorization") {
    Some(header_value) => {
      let token_str = match header_value.to_str() {
        Ok(s) => s.trim(),
        Err(_) => {
          return Err(HttpResponse::BadRequest()
            .json(json!({ "error": "Encabezado inválido" })))
        }
      };

      if !token_str.starts_with("Bearer ") {
        return Err(HttpResponse::BadRequest()
          .json(json!({ "error": "Formato inválido" })));
      }

      match token_str.strip_prefix("Bearer ") {
        Some(t) if !t.is_empty() => t.to_string(),
          _ => {
            return Err(HttpResponse::Unauthorized()
              .json(json!({ "error": "Token no proporcionado" })))
          }
      }
    },
    _none => {
      return Err(HttpResponse::Unauthorized()
          .json(json!({ "error": "Token requerido" })))
    }
  };

  let mut conn = establish_connection();

  match select_id_token(&mut conn, token) {
    Ok(id) => Ok(id),
    Err(_) => Err(HttpResponse::Unauthorized()
      .json(json!({ "error": "Token inválido o expirado" }))),
  }
}