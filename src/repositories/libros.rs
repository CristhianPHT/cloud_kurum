use crate::models::{Libro, LibroDashboard, NuevoLibro};
// use crate::schema::imagen_libro::dsl::{url_image, imagen_libro, libro_id};
use crate::schema::libro::dsl::{id as id_libro, libro, titulo};
use diesel::dsl::insert_into;
use diesel::prelude::*;

pub fn select_nombre_libros( conn: &mut PgConnection ) -> QueryResult<Vec<LibroDashboard>> {
  use crate::schema::imagen_libro::dsl::{imagen_libro, libro_id, tipo, is_active, url_image};
  libro
    .left_join(
      imagen_libro.on(
        libro_id
          .eq(id_libro)
          .and(tipo.eq("portada")).and(is_active.eq(true))
      )
      )
    .select((
      id_libro,
      titulo,
      url_image.nullable(),
    ))
    .load::<LibroDashboard>(conn)
}
pub fn insert_libro_nuevo(conn: &mut PgConnection, nuevo_libro: NuevoLibro) -> QueryResult<i32> {
  insert_into(libro)
    .values(nuevo_libro)
    .returning(id_libro)
    .get_result(conn)
}

pub fn select_libro_main(conn: &mut PgConnection, identificador: i32) -> QueryResult<Libro> {
  libro
    .filter(id_libro.eq(identificador))
    // .select(Libro::as_select())
    .first(conn)
}
