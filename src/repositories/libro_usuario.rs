#[allow(unused_imports)]
use crate::models::{RelationLibroUsuario, NuevoLibroUsuario, AllLibroxUsuario, NiceAccount, Libro};   // AllLibroxUsuario
use crate::schema::usuario::dsl::{usuario, id as id_usuario, username};
use crate::schema::libro;
use crate::schema::libro_usuario::dsl::{fk_libro, fk_usuario, id as IDLiUs, libro_usuario};
    use diesel::dsl::insert_into;
use diesel::prelude::*;

pub fn insert_libro_usuario(conn: &mut PgConnection, modelo: NuevoLibroUsuario) -> QueryResult<i32> {   // siempre returning id, para poder retornar errores
    insert_into(libro_usuario)
    .values(modelo)
    .returning(IDLiUs)
    .get_result(conn)
}

pub fn select_libros_public_username(conn: &mut PgConnection, nombre_usuario: &str) -> QueryResult<Vec<Libro>> {    // Seleccionar los libros públicos para mostrarlos, para todos
    let user = usuario
        .filter(username.eq(nombre_usuario))
        .select(NiceAccount::as_select())
        .get_result(conn)?;

        RelationLibroUsuario::belonging_to(&user)
        .inner_join(libro::table)
        .filter(libro::visibilidad.eq(true))
        .select(Libro::as_select())
        .load(conn)
}

pub fn select_all_books_of_user(conn: &mut PgConnection, user_id: i32) -> QueryResult<Vec<Libro>> {
    let user = usuario
        .filter(id_usuario.eq(user_id))
        .select(NiceAccount::as_select())
        .get_result(conn)?;
    
        RelationLibroUsuario::belonging_to(&user)
        .inner_join(libro::table)
        .select(Libro::as_select())
        .load(conn)
}

pub fn select_libros_por_usuario( conn: &mut PgConnection, usuario_id: i32 ) -> QueryResult<Vec<AllLibroxUsuario>> {
    let rows = libro_usuario
        .inner_join(libro::table.on(fk_libro.eq(libro::id.nullable())))
        .filter(fk_usuario.eq(Some(usuario_id)))
        .select((
            IDLiUs,
            fk_usuario,
            fk_libro,
            libro::titulo,
            libro::perfil,
            libro::sinopsis,
            libro::tipo,
            libro::visibilidad,
        ))
        .load::<(
            i32,
            Option<i32>,
            Option<i32>,
            Option<String>,
            Option<String>,
            Option<String>,
            Option<String>,
            Option<bool>,
        )>(conn)?;

    Ok(rows
        .into_iter()
        .map(
            |(
                relacion_id,
                usuario_id,
                libro_id,
                titulo,
                perfil,
                sinopsis,
                tipo,
                visibilidad,
            )| AllLibroxUsuario {
                relacion_id,
                usuario_id,
                libro_id,
                titulo,
                perfil,
                sinopsis,
                tipo,
                visibilidad,
            },
        )
        .collect())
}