use chrono::Local;
use diesel::PgConnection;

use crate::repositories::libros::insert_libro_nuevo;
use crate::web::dto::book::NewBook;
use crate::models::InsertLibro;

pub fn create_book_service( conn: &mut PgConnection, nuevo_libro: NewBook ) -> Result<String, diesel::result::Error> {
  let now = Local::now().naive_local();
  let libro = InsertLibro {
    titulo: nuevo_libro.titulo,
    slug: nuevo_libro.slug,
    sinopsis: nuevo_libro.sinopsis,
    tipo_id: nuevo_libro.tipo_id,
    publicacion: nuevo_libro.publicacion,
    estado_id: nuevo_libro.estado_id,
    visibilidad: nuevo_libro.visibilidad,
    created_at: now,
    updated_at: now,
  };
  insert_libro_nuevo(conn, libro)
}