use crate::models::{Libro, LibroDashboard, InsertLibro};
// use crate::schema::imagen_libro::dsl::{url_image, imagen_libro, libro_id};
use crate::schema::libro::dsl::{id as id_libro, libro, titulo, slug};
use diesel::dsl::insert_into;
use diesel::prelude::*;

pub fn select_nombre_libros( conn: &mut PgConnection, pagina: i64 ) -> QueryResult<Vec<LibroDashboard>> {
  use crate::schema::imagen_libro::dsl::{imagen_libro, libro_id, tipo, is_active, url_image};
  let por_pagina: i64 = 10;
  let offset: i64 = (pagina - 1) * por_pagina;  // e ingresar 'pagina' cómo input
  // let offset = 1;
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
      slug,
      url_image.nullable(),
    ))
    .order(id_libro.desc()) // importante (últimos libros subidos db), en futuro a fecha
    .limit(por_pagina)
    .offset(offset)
    .load::<LibroDashboard>(conn)
}

pub fn insert_libro_nuevo(conn: &mut PgConnection, nuevo_libro: InsertLibro) -> QueryResult<String> {
  insert_into(libro)
    .values(nuevo_libro)
    .returning(slug)
    .get_result(conn)
}

pub fn select_libro_main(conn: &mut PgConnection, slug_as_id: String) -> QueryResult<Libro> {
  libro
    .filter(slug.eq(slug_as_id))
    // .select(Libro::as_select())
    .first(conn)
}
