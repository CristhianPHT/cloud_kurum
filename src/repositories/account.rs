use crate::models::{NuevoAccount};
use crate::web::dto::account::{HeaderAccount, Account};
use crate::schema::usuario::dsl::{id as account_id, password_hash, username, usuario, nickname, updated_at, email};
use crate::schema::imagen_perfil::dsl::{imagen_perfil as perfil_acc, user_id as fk_image, url_image};
use diesel::dsl::{insert_into, update};
use diesel::prelude::*;

pub fn insert_usuario( conn: &mut PgConnection, nuevo: NuevoAccount ) -> QueryResult<i32> {
  insert_into(usuario)
    .values(&nuevo)
    .returning(account_id)
    .get_result(conn)
}

pub fn select_usuario_por_nickname(conn: &mut PgConnection, nick_name: &str) -> QueryResult<HeaderAccount> {  // versión publica 
  usuario
    .left_join(
      perfil_acc.on(
        fk_image.eq(account_id)
      )
    )
    .filter(nickname.eq(nick_name))
    .select((
      nickname,
      // username,
      url_image.nullable(),
    ))
    .first::<HeaderAccount>(conn)
}

pub fn select_usuario_por_id( conn: &mut PgConnection, usuario_id: i32 ) -> QueryResult<Account> { // select_usuario_por_id, futura versión name?
  usuario
    .left_join(
      perfil_acc.on(
        fk_image.eq(account_id)
      )
    )
    .filter(account_id.eq(usuario_id))
    .select((
      nickname,
      username,
      email,
      url_image.nullable(),
    ))
    .first::<Account>(conn)
}
// pub fn select_id_usuario(conn: &mut PgConnection, usuario_id: i32) -> QueryResult<Account> {
//     diesel::query_dsl::methods::FindDsl::find(usuario, usuario_id)
//         .select(Account::as_select())
//         .first::<Account>(conn)
// }
pub fn select_me_header ( conn: &mut PgConnection, usuario_id: i32 ) -> QueryResult<HeaderAccount> { // select_usuario_por_id, futura versión name?
  usuario
    .left_join(
      perfil_acc.on(
        fk_image.eq(account_id)
      )
    )
    .filter(account_id.eq(usuario_id))
    .select((
      nickname,
      url_image.nullable(),
    ))
    .first::<HeaderAccount>(conn)
}

pub fn select_login_credentials(conn: &mut PgConnection, user_name: &str) -> QueryResult<(i32, String)> {  // no se usa... por ahora?
  usuario
    .filter(username.eq(user_name))
    .select((account_id, password_hash))
    .first(conn)
}
// de momento es algo no funcional... update_login
pub fn update_login(conn: &mut PgConnection, usuario_id: i32, nuevo_username: String, nuevo_password_hash: String) -> QueryResult<usize> {  // no se usa por ahora
  update(usuario.filter(account_id.eq(usuario_id)))
    .set((username.eq(nuevo_username), password_hash.eq(nuevo_password_hash)))
    .execute(conn)
}
pub fn username_existe( conn: &mut PgConnection, user_name: &str ) -> QueryResult<bool> { // También funciona para nickname...
  diesel::select(
    diesel::dsl::exists(
      usuario.filter(username.eq(user_name))
    )
  )
  .get_result(conn)
}

pub fn select_password_by_id( conn: &mut PgConnection, user_id: i32 ) -> QueryResult<String> {
  usuario
    .filter(account_id.eq(user_id))
    .select(password_hash)
    .first(conn)
}

pub fn update_contrasenia (conn: &mut PgConnection, user_id: i32, new_password_hash: &str) -> QueryResult<()> {
  update(usuario.filter(account_id.eq(user_id)))
    .set(password_hash.eq(new_password_hash))
    .execute(conn)
    .map(|_| ())   // "espero el unit type, o sea nada"
}

use crate::schema::auth_tokens::dsl::{is_active, token as token_auth, auth_tokens};
pub fn delete_token (conn: &mut PgConnection, token_input: &str) -> QueryResult<()> { //soft delete
  let affected = update(auth_tokens
    .filter(token_auth.eq(token_input)))
    .set(is_active.eq(false))
    .execute(conn)?;
  if affected == 0 {
    return Err(diesel::result::Error::NotFound);
  }
  Ok(())
}

use crate::web::dto::account::UpdateNickname;
pub fn update_nickname( conn: &mut PgConnection, user_id: i32, data: UpdateNickname ) -> QueryResult<()> {
  update(usuario.filter(account_id.eq(user_id)))
    .set((
      nickname.eq(data.nickname),
      updated_at.eq(diesel::dsl::now),
    ))
    .execute(conn)?;
  Ok(())
}
// pub fn username_existe(conn: &mut PgConnection, user_name: &String) -> Result<bool, diesel::result::Error> {
//     let count: i64 = usuario
//         .filter(username.eq(user_name))
//         .select(count_star())
//         .first::<i64>(conn)?;
//     Ok(count > 0)
// }

pub fn login_usuario_hashed_old(conn: &mut PgConnection, user_name: &str, hashed_password: &str) -> QueryResult<i32> {
  usuario
    .filter(username.eq(user_name))
    .filter(password_hash.eq(hashed_password))
    .select(account_id)
    .first::<i32>(conn)
}
