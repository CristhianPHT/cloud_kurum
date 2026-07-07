// --------------------------------------------------------------------------------------------
// insert_auth_token Se crea un nuevo token... solo eso...
use crate::{calculate_expiration, generate_jwt, insert_auth_token};
use crate::models::NuevoAuthToken;
#[post("/auth")]
pub async fn auth_user(user: web::Json<NuevoAuthToken>) -> impl Responder {
  let mut conn = establish_connection();

  let auth_token = user.into_inner();
  let expira = calculate_expiration();
  let token = match generate_jwt(auth_token.user_id, expira){ 
  // let token = match generate_jwt(identidad, expira) {
    Ok(token) => token,
    Err(_) => return HttpResponse::InternalServerError().json(json!({ "error": "Error al generar token" })),
  };
  let _data_base = insert_auth_token(&mut conn, auth_token.user_id, &token, expira);  // token : &str
  HttpResponse::Ok().json(json!({
      "auth_token": token
  }))
}