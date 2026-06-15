use crate::models::NuevoAccount;
use crate::repositories::account as account_repository;
use bcrypt::{hash, verify, DEFAULT_COST};
use diesel::prelude::*;
use crate::web::dto::account::RegisterAccount;

pub fn register_user( conn: &mut PgConnection, json_dto: RegisterAccount ) -> QueryResult<i32> {
  let hashed_password =
    hash(&json_dto.password, DEFAULT_COST)
    .map_err(|e|
      diesel::result::Error::DeserializationError(
        Box::new(e)
      )
    )?;
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

  account_repository::insert_usuario(
    conn,
    nuevo
  )
}

pub fn login_usuario_hashed(conn: &mut PgConnection, user_email: &str, password_plano: &str) -> QueryResult<i32> {
    let (identificador, contrasenia) = account_repository::select_login_credentials(conn, user_email)?;
    let is_valid = verify(password_plano, &contrasenia)
        .map_err(|e| diesel::result::Error::DeserializationError(Box::new(e)))?;
    if is_valid {
        Ok(identificador)
    } else {
        Err(diesel::result::Error::NotFound)
    }
}

pub fn update_login(conn: &mut PgConnection, usuario_id: i32, nuevo: NuevoAccount) -> QueryResult<usize> {
    account_repository::update_login(conn, usuario_id, nuevo.username, nuevo.password_hash)
}
