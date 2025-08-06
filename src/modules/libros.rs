use crate::models::{Libro, LibroDashboard, NuevoLibro};
use crate::schema::libro::dsl::{id as libro_id_all, libro, perfil, titulo};
use diesel::dsl::insert_into;
use diesel::prelude::*;

pub fn select_nombre_libros(
    conn: &mut PgConnection,
) -> Result<Vec<LibroDashboard>, diesel::result::Error> {
    libro
        .select((libro_id_all, titulo, perfil))
        .load::<LibroDashboard>(conn)
}

pub fn insert_libro_nuevo(conn: &mut PgConnection, nuevo_libro: NuevoLibro) -> QueryResult<i32> {
    let salida = insert_into(libro)
        .values(nuevo_libro)
        .returning(libro_id_all)
        .get_result(conn);
    salida
}

pub fn select_libro_main(conn: &mut PgConnection, identificador: i32) -> QueryResult<Libro> {
    let test = libro
        .filter(libro_id_all.eq(identificador))
        .first::<Libro>(conn);
    test
}
