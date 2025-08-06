use diesel::pg::Pg;
use diesel::prelude::*;
use diesel::query_dsl::methods::{FindDsl, LimitDsl};
use diesel::query_dsl::{LoadQuery, RunQueryDsl};
use diesel::Table;
use dotenv::dotenv;
use std::env;

// Establece la conexión a la base de datos
pub fn establish_connection() -> PgConnection {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("Se debe configurar DATABASE_URL");
    PgConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}

// Función genérica para seleccionar por ID
pub fn select_by_id<T, U>(
    table: T,
    conn: &mut PgConnection,
    id_clave: i32,
) -> Result<U, diesel::result::Error>
where
    T: Table + FindDsl<i32>,
    <T as FindDsl<i32>>::Output: LimitDsl + RunQueryDsl<PgConnection>,
    <<T as FindDsl<i32>>::Output as LimitDsl>::Output: LoadQuery<'static, PgConnection, U>,
    U: Queryable<<T as Table>::AllColumns, Pg>,
    <T as Table>::AllColumns: diesel::Expression<SqlType = diesel::sql_types::Untyped>,
{
    table.find(id_clave).first(conn)
}

// Función genérica para insertar
use diesel::insertable::CanInsertInSingleQuery;
use diesel::query_builder::{InsertStatement, QueryFragment, QueryId};
use diesel::query_dsl::methods::ExecuteDsl;

pub fn generic_insert<T, U>(
    table: T,
    conn: &mut PgConnection,
    data: U,
) -> Result<usize, diesel::result::Error>
where
    T: Table + QuerySource + QueryId,
    U: Insertable<T>,
    InsertStatement<T, U::Values>: ExecuteDsl<PgConnection>,
    U::Values: QueryFragment<Pg> + QueryId + CanInsertInSingleQuery<Pg>,
{
    diesel::insert_into(table).values(data).execute(conn)
}
