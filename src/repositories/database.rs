use diesel::insertable::CanInsertInSingleQuery;
use diesel::pg::Pg;
use diesel::prelude::*;
use diesel::query_builder::{InsertStatement, QueryFragment, QueryId};
use diesel::query_dsl::methods::{ExecuteDsl, FindDsl, LimitDsl};
use diesel::query_dsl::{LoadQuery, RunQueryDsl};
use diesel::Table;

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
