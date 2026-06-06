use crate::web::dto::account::Account;
use crate::models::{NuevoAccount, HeaderAccount};
use crate::schema::usuario::dsl::{id as account_id, password_hash, username, usuario};
use diesel::dsl::{count_star, insert_into, update};
use diesel::prelude::*;

pub fn insert_usuario( conn: &mut PgConnection, nuevo: NuevoAccount ) -> QueryResult<i32> {

    insert_into(usuario)
        .values(&nuevo)
        .returning(account_id)
        .get_result(conn)
}

pub fn select_usuario_por_username(conn: &mut PgConnection, user_name: &str) -> QueryResult<Account> {
    usuario
        .filter(username.eq(user_name))
        .select(Account::as_select())
        .first::<Account>(conn)
}

pub fn select_id_usuario(conn: &mut PgConnection, usuario_id: i32) -> QueryResult<Account> {
    diesel::query_dsl::methods::FindDsl::find(usuario, usuario_id)
        .select(Account::as_select())
        .first::<Account>(conn)
}

pub fn select_header_user(conn: &mut PgConnection, usuario_id: i32) -> Result<HeaderAccount, diesel::result::Error> {
    use crate::schema::usuario::dsl::{nickname, perfil, usuario, id as accoun_id};
    usuario
    .filter(accoun_id.eq(usuario_id))
    .select((nickname, perfil))
    .first::<HeaderAccount>(conn)
}

pub fn select_login_credentials(conn: &mut PgConnection, user_email: &str) -> QueryResult<(i32, String)> {
    usuario
        .filter(username.eq(user_email))
        .select((account_id, password_hash))
        .first(conn)
}

pub fn update_login(conn: &mut PgConnection, usuario_id: i32, nuevo_username: String, nuevo_password_hash: String) -> QueryResult<usize> {
    update(usuario.filter(account_id.eq(usuario_id)))
        .set((username.eq(nuevo_username), password_hash.eq(nuevo_password_hash)))
        .execute(conn)
}

pub fn username_existe(conn: &mut PgConnection, user_name: &String) -> Result<bool, diesel::result::Error> {
    let count: i64 = usuario
        .filter(username.eq(user_name))
        .select(count_star())
        .first::<i64>(conn)?;
    Ok(count > 0)
}

pub fn login_usuario_hashed_old(conn: &mut PgConnection, user_email: &str, hashed_password: &str) -> QueryResult<i32> {
    usuario
        .filter(username.eq(user_email))
        .filter(password_hash.eq(hashed_password))
        .select(account_id)
        .first::<i32>(conn)
}
