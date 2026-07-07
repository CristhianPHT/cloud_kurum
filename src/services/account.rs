use crate::models::{LoginAccount, NuevoAccount};
use crate::repositories::{account as account_repository, select_login_credentials};
// use crate::schema::usuario::password_hash;
use bcrypt::{hash, verify, DEFAULT_COST};
use diesel::prelude::*;
use crate::web::dto::account::RegisterAccount;
use diesel::result::{Error, DatabaseErrorKind}; // register_user
use crate::services::auth::{generate_jwt, calculate_expiration}; //register_user
use crate::insert_auth_token; // (src/repositories), register_user
pub enum UserError {  // AuthError posible nombre futuro
  Validation(String), // 400
  InvalidCredentials, // 401
  EmailTaken,   // 409
  NicknameTaken,    // 409
  TokenError,   // 500
  DatabaseError,    // 500
  Internal,   // 500
}
// pub enum UserError {   // más profesional (evolutivo)
//   Validation(String),
//   Auth(AuthError),
//   Register(RegisterError),
//   Infra(InfraError),
// }
// ------------------- Moverlo a src/validation/*.rs todas las validaciones ----------------------
fn validate_credentials(username: &str, password: &str) -> Result<(), UserError> {
  if username.trim().is_empty() {
    return Err(UserError::Validation("username vacío".into()));
  }
  if password.trim().is_empty() {
    return Err(UserError::Validation("password vacío".into()));
  }
  if password.len() > 128 {
    return Err(UserError::Validation("contraseña demasiado larga".into()));
  }
  Ok(())
}
pub fn validate_password(password: &str) -> Result<(), UserError> {
  if password.len() < 8 {
    return Err(UserError::Validation(
      "La contraseña debe tener al menos 8 caracteres".into()
    ));
  }
  Ok(())
}

// insert_auth_token con username y password para obtener el authtoken  ?
pub fn register_user( conn: &mut PgConnection, json_dto: RegisterAccount ) -> Result<String, UserError> {
  // 1. VALIDACIÓN
  validate_credentials(&json_dto.username, &json_dto.password)?;
  let hashed_password = hash(&json_dto.password, DEFAULT_COST)
  .map_err(|_| UserError::Internal)?;
  let now = chrono::Utc::now().naive_utc();
  let nuevo = NuevoAccount {
    nickname: json_dto.nickname,
    username: json_dto.username,
    password_hash: hashed_password,
    email: json_dto.email,
    is_active: true,
    updated_at: now,
    created_at: now,
  };
  // 2. INSERT USER
  let user_id = match account_repository::insert_usuario(conn, nuevo) {
    Ok(id) => id,
    Err(Error::DatabaseError(DatabaseErrorKind::UniqueViolation, info)) => {
      match info.constraint_name() {
        Some("account_email_key") => return Err(UserError::EmailTaken),
        Some("account_nickname_key") => return Err(UserError::NicknameTaken),
        _ => return Err(UserError::Internal),
      }
    }
    Err(_) => return Err(UserError::Internal),
  };
  // 3. JWT
  let expira = calculate_expiration();
  let token = generate_jwt(user_id, expira)
    .map_err(|_| UserError::TokenError)?;
  // 4. Guardar token 
  insert_auth_token(conn, user_id, &token, expira)
    .map_err(|_| UserError::Internal)?;
  Ok(token)
}

pub fn login_user_service ( conn: &mut PgConnection, login: LoginAccount ) -> Result<String, UserError> {
  // 1. Validación
  validate_credentials(&login.username, &login.password)?;
  // 2. Extraemos id, password (database)
  let (user_id, password_hashed) = select_login_credentials(conn, &login.username)
    .map_err(|_| UserError::InvalidCredentials)?;
  // 3. Verificación password
  verify(&login.password, &password_hashed)
    .map_err(|_| UserError::Internal)?;
  // 4. JWT
  let expira = calculate_expiration();
  let token = generate_jwt(user_id, expira)
    .map_err(|_| UserError::TokenError)?;
  // 5. Guardar token
  insert_auth_token(conn, user_id, &token, expira)
    .map_err(|_| UserError::Internal)?;
  Ok(token)
}


// fn validación_password ???
use crate::web::dto::account::ChangePassword;
use crate::repositories::account::{update_contrasenia, select_password_by_id};
pub fn update_password_service(conn: &mut PgConnection,usuario_id: i32,change_pass: ChangePassword)
-> Result<(), UserError> {
  // 1. Validaciones
  validate_password(&change_pass.new_password)?;
  // 2. Obtener hash actual
  let current_hash = select_password_by_id(conn, usuario_id)
    .map_err(|_| UserError::InvalidCredentials)?;
  // 3. Verificar contraseña actual
  let is_valid = verify( &change_pass.old_password, &current_hash )
    .map_err(|_| UserError::Internal)?;
  if !is_valid {
    return Err(UserError::InvalidCredentials);
  }
  // 4. Hashear nueva contraseña
  let new_hash = hash(&change_pass.new_password, DEFAULT_COST)
    .map_err(|_| UserError::Internal)?;
  // 5. Actualizar
  update_contrasenia(conn, usuario_id, &new_hash)
    .map_err(|_| UserError::DatabaseError)?;
  Ok(())
}

pub fn update_login(conn: &mut PgConnection, usuario_id: i32, nuevo: NuevoAccount) -> QueryResult<usize> {
    account_repository::update_login(conn, usuario_id, nuevo.username, nuevo.password_hash)
}
