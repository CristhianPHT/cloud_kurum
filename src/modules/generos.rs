use crate::models::{Genero, NuevoGenero};
use crate::schema::genero::dsl::{genero, id as gen_id};
use diesel::dsl::insert_into;
use diesel::prelude::*;

pub fn insert_gen_new(conn: &mut PgConnection, gen_new: NuevoGenero) -> QueryResult<i32> {
    // Ingresar un nuevo género
    let query = insert_into(genero)
        .values(gen_new)
        .returning(gen_id)
        .get_result(conn);
    query
}

pub fn select_gen_unico(conn: &mut PgConnection, identificar: i32) -> QueryResult<i32> {
    // Seleccionamos sólo un género
    diesel::QueryDsl::find(genero, identificar)
        .select(gen_id)
        .first::<i32>(conn)
}

pub fn select_gen_all(conn: &mut PgConnection) -> Result<Vec<Genero>, diesel::result::Error> {
    // Seleccionamos todos los géneros
    genero.load::<Genero>(conn)
}
