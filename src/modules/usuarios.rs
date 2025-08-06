use crate::models::{NuevoUsuario, Usuario, UsuarioUpdate};
use crate::schema::usuariosss::dsl::{apellido, id, nombre, usuariosss};
use diesel::dsl::update;
use diesel::prelude::*;
use diesel::query_dsl::methods::{LimitDsl, OffsetDsl};

pub fn select_id(
    conn: &mut PgConnection,
    usuario_id: i32,
) -> Result<Usuario, diesel::result::Error> {
    use crate::schema::usuariosss::dsl::*;

    usuariosss.filter(id.eq(usuario_id)).first::<Usuario>(conn)
}

pub fn select_all_users(conn: &mut PgConnection, page: i64) -> Vec<Usuario> {
    let offset_val = 5 * page;
    OffsetDsl::offset(
        LimitDsl::limit(diesel::QueryDsl::order_by(usuariosss, id), 5),
        offset_val,
    )
    .load::<Usuario>(conn)
    .expect("No cargó Usuarios: Error")
}

pub fn update_user_id(
    conn: &mut PgConnection,
    usuario_id: i32,
    usuario_local: UsuarioUpdate,
) -> QueryResult<usize> {
    let query = update(usuariosss.filter(id.eq(usuario_id)));

    match (&usuario_local.nombre, &usuario_local.apellido) {
        (Some(nuevo_nombre), Some(nuevo_apellido)) => query
            .set((
                nombre.eq(nuevo_nombre.clone()),
                apellido.eq(nuevo_apellido.clone()),
            ))
            .execute(conn),
        (Some(nuevo_nombre), None) => query.set(nombre.eq(nuevo_nombre.clone())).execute(conn),
        (None, Some(nuevo_apellido)) => {
            query.set(apellido.eq(nuevo_apellido.clone())).execute(conn)
        }
        (None, None) => {
            println!("No se puede actualizar: ningún campo para modificar");
            Ok(0)
        }
    }
}

pub fn insert_user(connec: &mut PgConnection, nuevo: NuevoUsuario) -> QueryResult<i32> {
    use diesel::dsl::insert_into;

    let inserted_id = insert_into(usuariosss)
        .values(nuevo)
        .returning(id)
        .get_result(connec);

    inserted_id
}
