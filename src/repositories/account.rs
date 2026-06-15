use crate::models::{NuevoAccount, HeaderAccount};
use crate::schema::usuario::dsl::{id as account_id, password_hash, username, usuario, nickname};
use crate::schema::imagen_perfil::dsl::{imagen_perfil as perfil_acc, user_id as fk_image, url_image};
use diesel::dsl::{insert_into, update};
use diesel::prelude::*;

pub fn insert_usuario( conn: &mut PgConnection, nuevo: NuevoAccount ) -> QueryResult<i32> {
  insert_into(usuario)
    .values(&nuevo)
    .returning(account_id)
    .get_result(conn)
}

pub fn select_usuario_por_nickname(conn: &mut PgConnection, user_name: &str) -> QueryResult<HeaderAccount> {  // versión publica 
  usuario
    .left_join(
      perfil_acc.on(
        fk_image.eq(account_id)
      )
    )
    .filter(username.eq(user_name))
    .select((
      nickname,
      // username,
      url_image.nullable(),
    ))
    .first::<HeaderAccount>(conn)
}

pub fn select_usuario_por_id( conn: &mut PgConnection, usuario_id: i32 ) -> QueryResult<HeaderAccount> { // select_usuario_por_id, futura versión name?
  usuario
    .left_join(
      perfil_acc.on(
        fk_image.eq(account_id)
      )
    )
    .filter(account_id.eq(usuario_id))
    .select((
      nickname,
      // username,
      url_image.nullable(),
    ))
    .first::<HeaderAccount>(conn)
}
// pub fn select_id_usuario(conn: &mut PgConnection, usuario_id: i32) -> QueryResult<Account> {
//     diesel::query_dsl::methods::FindDsl::find(usuario, usuario_id)
//         .select(Account::as_select())
//         .first::<Account>(conn)
// }
pub fn select_header_by_id( conn: &mut PgConnection, usuario_id: i32 ) -> QueryResult<HeaderAccount> { // select_usuario_por_id, futura versión name?
  usuario
    .left_join(
      perfil_acc.on(
        fk_image.eq(account_id)
      )
    )
    .filter(account_id.eq(usuario_id))
    .select((
      nickname,
      // username,
      url_image.nullable(),
    ))
    .first::<HeaderAccount>(conn)
}

pub fn select_login_credentials(conn: &mut PgConnection, user_email: &str) -> QueryResult<(i32, String)> {  // no se usa... por ahora?
  usuario
    .filter(username.eq(user_email))
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
