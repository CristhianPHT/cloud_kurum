use crate::models::{Libro, NuevoLibroGenero};
use crate::schema::libro_genero::dsl::{id as libro_genero_id, libro_genero};
use diesel::dsl::insert_into;
use diesel::prelude::*;

// INSERT - Relacionar libro con género
pub fn insert_libro_genero(
    conn: &mut PgConnection,
    lib_gen_data: NuevoLibroGenero,
) -> QueryResult<i32> {
    insert_into(libro_genero)
        .values(&lib_gen_data)
        .returning(libro_genero_id)
        .get_result(conn)
}

#[derive(Debug)]
pub enum OrdenamientoLibro {
    TituloAsc,
    TituloDesc,
    FechaAsc,
    FechaDesc,
}

pub fn buscar_libros_por_genero(
    conn: &mut PgConnection,
    id_genero: i32,
    pagina: i64,
    orden: OrdenamientoLibro,
) -> QueryResult<Vec<Libro>> {
    use crate::schema::{libro, libro_genero};
    use diesel::prelude::*;

    let resultados_por_pagina = 10;
    let offset = (pagina - 1) * resultados_por_pagina;

    let mut query = libro::table
        .inner_join(libro_genero::table.on(libro_genero::libro_id.eq(libro::id)))
        .filter(libro_genero::genero_id.eq(id_genero))
        .select(libro::all_columns)
        .into_boxed();

    query = match orden {
        OrdenamientoLibro::TituloAsc => query.order(libro::titulo.asc()),
        OrdenamientoLibro::TituloDesc => query.order(libro::titulo.desc()),
        OrdenamientoLibro::FechaAsc => query.order(libro::publicacion.asc()),
        OrdenamientoLibro::FechaDesc => query.order(libro::publicacion.desc()),
    };

    let query = diesel::QueryDsl::limit(query, resultados_por_pagina);
    diesel::QueryDsl::offset(query, offset).load::<Libro>(conn)
}
