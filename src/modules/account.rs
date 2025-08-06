use crate::models::{Account, NuevoAccount};
use crate::schema::usuario::dsl::{id as account_id, password_hash, username, usuario};
use bcrypt::{hash, verify, DEFAULT_COST};
use diesel::dsl::{count_star, insert_into, update};
use diesel::prelude::*;

pub fn insert_usuario(conn: &mut PgConnection, mut nuevo: NuevoAccount) -> QueryResult<i32> {
    // Hashear la contraseña antes de insertar
    let hashed_password = hash(&nuevo.password_hash, DEFAULT_COST)
        .map_err(|e| diesel::result::Error::DeserializationError(Box::new(e)))?;

    nuevo.password_hash = hashed_password;
    let inserted_id = insert_into(usuario)
        .values(nuevo)
        .returning(account_id)
        .get_result(conn);
    inserted_id
}

pub fn select_id_usuario(conn: &mut PgConnection, usuario_id: i32) -> QueryResult<Account> {
    let cuentas = diesel::query_dsl::methods::FindDsl::find(usuario, usuario_id)
        .select(Account::as_select())
        .first::<Account>(conn);
    cuentas
}

pub fn login_usuario_hashed(
    conn: &mut PgConnection,
    user_email: &str,
    password_plano: &str,
) -> QueryResult<i32> {
    let (identificador, contrasenia): (i32, String) = usuario
        .filter(username.eq(user_email))
        .select((account_id, password_hash))
        .first(conn)?;

    // Verificar la contraseña en texto plano contra el hash almacenado
    let is_valid = verify(password_plano, &contrasenia)
        .map_err(|e| diesel::result::Error::DeserializationError(Box::new(e)))?;

    if is_valid {
        Ok(identificador)
    } else {
        Err(diesel::result::Error::NotFound)
    }
}

pub fn update_login(
    conn: &mut PgConnection,
    usuario_id: i32,
    nuevo: NuevoAccount,
) -> QueryResult<usize> {
    let query = update(usuario.filter(account_id.eq(usuario_id)));
    query.set(nuevo).execute(conn)
}

pub fn username_existe(
    conn: &mut PgConnection,
    user_name: &String,
) -> Result<bool, diesel::result::Error> {
    let count: i64 = usuario
        .filter(username.eq(user_name))
        .select(count_star())
        .first::<i64>(conn)?;
    Ok(count > 0)
}

pub fn login_usuario_hashed_old(
    conn: &mut PgConnection,
    user_email: &str,
    hashed_password: &str,
) -> QueryResult<i32> {
    let cuentas = usuario
        .filter(username.eq(user_email))
        .filter(password_hash.eq(hashed_password))
        .select(account_id)
        .first::<i32>(conn);

    cuentas
}
