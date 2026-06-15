

use crate::models::{RelationLibroUsuario, NuevoLibroUsuario, NiceAccount, ImagenLibro, Libro};   // AllLibroxUsuario
use crate::schema::usuario::dsl::{usuario, username};
use crate::schema::libro::dsl::{libro, visibilidad};
use crate::schema::usuario_libro::dsl::{usuario_id, id as id_usuario_libro, usuario_libro};
use diesel::dsl::insert_into;
use diesel::prelude::*;

pub fn insert_libro_usuario(conn: &mut PgConnection, modelo: NuevoLibroUsuario) -> QueryResult<i32> {   // siempre returning id, para poder retornar errores
  insert_into(usuario_libro)
    .values(modelo)
    .returning(id_usuario_libro)
    .get_result(conn)
}

pub fn select_public_books_by_username(conn: &mut PgConnection, nombre_usuario: &str) -> QueryResult<Vec<Libro>> {    // Seleccionar los libros públicos para mostrarlos, para todos
  let user = usuario  // user = select * from where username=? limit 1;
    .filter(username.eq(nombre_usuario))
    .first::<NiceAccount>(conn)?;
  RelationLibroUsuario::belonging_to(&user) //select * from libro ...
    .inner_join(libro)
    .filter(visibilidad.eq(Some(true)))
    .select(Libro::as_select())
    .load::<Libro>(conn)
  // user equivale 1 consulta, y belonging_to equivale a otra consulta, terminando en 2 consultas
}

pub fn select_all_books_by_user( conn: &mut PgConnection, user_id: i32 ) -> QueryResult<Vec<Libro>> {
  usuario_libro
    .filter(usuario_id.eq(user_id))
    .inner_join(libro)
    .select(Libro::as_select())
    .load(conn)
}

// pub fn select_books_by_user_images( conn: &mut PgConnection, user_id: i32 ) -> QueryResult<Vec<(Libro, Option<Imagelibro>)>>{
//   use crate::schema::imagen_libro::dsl::{imagen_libro, id as id_image_libro, tipo, is_active};
//   use crate::schema::usuario_libro::dsl::libro_id;
//   usuario_libro
//     .filter(usuario_id.eq(user_id))
//     .inner_join(libro)
//     .left_join(
//       imagen_libro.on(
//         libro_id
//           .eq(id_image_libro)
//           .and(tipo.eq("portada"))
//           .and(is_active.eq(true))
//       )
//     )
//     .select(( Libro::as_select(),Imagelibro::as_select().nullable()))
//     .load::<(Libro, Option<Imagelibro>)>(conn)
// }
pub fn select_books_by_user_images( conn: &mut PgConnection, user_id: i32 ) -> QueryResult<Vec<(Libro, Option<ImagenLibro>)>> {
  use crate::schema::usuario_libro::dsl::{ usuario_libro, usuario_id };
  use crate::schema::libro::dsl::id as id_libro;
  use crate::schema::imagen_libro::dsl::{ libro_id, imagen_libro, tipo, is_active };
  usuario_libro
    .filter(usuario_id.eq(user_id))
    .inner_join(libro)
    .left_join(
      imagen_libro.on(
        libro_id
          .eq(id_libro)
          .and(tipo.eq("portada"))
          .and(is_active.eq(true))
      )
    )
    .select((
      Libro::as_select(),
      Option::<ImagenLibro>::as_select(),
    ))
    .load::<(Libro, Option<ImagenLibro>)>(conn)
    // .left_join(
    //   imagen_libro.on(
    //     imagen_libro_id                  // imagen_libro.libro_id
    //     .eq(libro_id)               // libro.id
    //     .and(tipo.eq("portada"))
    //     .and(is_active.eq(true))
    //   )
    // )
}