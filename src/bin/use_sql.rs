// Selecciona todos los usuarios de la base de datos
use diesel::prelude::*;
use nube_kurum::models::{Usuario, UsuarioUpdate}; 
use nube_kurum::schema::usuariosss::dsl::{usuariosss, id, nombre, apellido}; // para id y usuariosss
// use nube_kurum::establish_connection;  // ya no llamarlo si se usa en otro lado

pub fn select_id(conn: &mut PgConnection, usuario_id: i32) -> Usuario {
  let usuario = usuariosss
    .find(usuario_id)
    .first::<Usuario>(conn)
    // .optional() // Devuelve un Option<Usuario>
    .expect("Error al buscar el usuario");
  usuario
}
pub fn select_all_users(conn: &mut PgConnection, page: i64) -> Vec<Usuario> {
  let offset = 5 * page;
  let select_users: Vec<Usuario> = usuariosss
    .order_by(id) // ORDER BY
    .limit(5) //
    .offset(offset) //
    .load::<Usuario>( conn) //
    .expect("No cargó Usuarios: Error");
  select_users
}
use diesel::dsl::update;

  // pub fn update_user_id(
  //     conn: &mut PgConnection,
  //     usuario_id: i32,
  //     usuario: UsuarioUpdate,
  // ) -> Result<usize, diesel::result::Error> {
  //     // Ejecutar el update
  //     let result = update(usuariosss.filter(id.eq(usuario_id) ) )
  //         .set((
  //             nombre.eq(usuario.nombre.clone().unwrap_or_default()),
  //             apellido.eq(usuario.apellido.clone().unwrap_or_default()),
  //         ))
  //         .execute(conn); // Ejecutar con la conexión mutable

  //     result // Devolver el resultado (número de filas afectadas o error)
  // }
// Errores, no sirve
pub fn actualizar_usuario(conn: &mut PgConnection, usuario_id: i32, usuario: Usuario) -> QueryResult<usize> {
  let mut query = update(usuariosss.filter(id.eq(usuario_id)));
  if let Some(nuevo_nombre) = usuario.nombre {
    query = query.set(nombre.eq(nuevo_nombre));
  }
  if let Some(b) = usuario.nombre.map(|a| nombre.eq(a)) {
    query = query.set(b);
}

  if let Some(q) = usuario.apellido.map(|a| apellido.eq(a)) {
    query = query.set(q);
}

  
  query.execute(conn)
}
  