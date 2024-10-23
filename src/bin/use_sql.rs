// Selecciona todos los usuarios de la base de datos
use diesel::prelude::*;
use nube_kurum::models::Usuario; 
use nube_kurum::schema::usuariosss::dsl::{usuariosss, id}; // para id y usuariosss
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
// fn main(){
//   // establish_connection();
//   let mut connection = establish_connection();
//   let lista_usuarios: Vec<Usuario> = select_all_users(&mut connection, 1);
//   println!("Displaying {} usuarios", lista_usuarios.len());
//   for usuario in &lista_usuarios {
//     println!("ID: {}", usuario.id);
//     println!("Nombre: {}", usuario.nombre);
//     println!("Apellido: {}", usuario.apellido);
//   }
// }