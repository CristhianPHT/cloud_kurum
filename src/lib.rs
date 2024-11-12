
use diesel::prelude::*;
use dotenv::dotenv;
use std::env;
// modulos públicos para usarlo con name de package. //(nube_kurum)
pub mod models;
pub mod schema;
pub mod web {
  pub mod basic;
  pub mod interface;
}


// Establece la conexión a la base de datos
pub fn establish_connection() -> PgConnection {  // para conectar a la base de datos
  dotenv().ok();
  let database_url = env::var("DATABASE_URL")
    .expect("Se debe configurar DATABASE_URL");
  PgConnection::establish(&database_url)
    .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}

use models::{Usuario, UsuarioUpdate}; 
use schema::usuariosss::dsl::{usuariosss, id, nombre, apellido}; // para id y usuariosss
// use nube_kurum::establish_connection;  // ya no llamarlo si se usa en otro lado

pub fn select_id(conn: &mut PgConnection, usuario_id: i32) -> Usuario {  // para mostrar usuario por id = input(conn, id)
  let usuario = usuariosss
    .find(usuario_id)
    .first::<Usuario>(conn)
    // .optional() // Devuelve un Option<Usuario>
    .expect("Error al buscar el usuario");
  usuario
}
pub fn select_all_users(conn: &mut PgConnection, page: i64) -> Vec<Usuario> {  // para mostrar todos los usuarios = input(conn, page)
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


// pub fn update_user_id(conn: &mut PgConnection, usuario_id: i32, usuario: UsuarioUpdate) -> QueryResult<usize> {
//   let actualizar = update(usuariosss.filter(id.eq(usuario_id)));
  
//   if let (Some(nuevo_nombre), Some(nuevo_apellido)) = (&usuario.nombre, &usuario.apellido) {
//       actualizar.set((
//           nombre.eq(nuevo_nombre.clone()),
//           apellido.eq(nuevo_apellido.clone())
//       )).execute(conn)
//   } else if let Some(nuevo_nombre) = &usuario.nombre {
//       actualizar.set(nombre.eq(nuevo_nombre.clone())).execute(conn)
//   } else if let Some(nuevo_apellido) = &usuario.apellido {
//       actualizar.set(apellido.eq(nuevo_apellido.clone())).execute(conn)
//   } else {
//       println!("No se puede actualizar: ningún campo para modificar");
//       Ok(0)
//   }
// }


pub fn update_user_id(conn: &mut PgConnection, usuario_id: i32, usuario: UsuarioUpdate) -> QueryResult<usize> {
  // Iniciamos la consulta base
  let query = update(usuariosss.filter(id.eq(usuario_id)));

  match (&usuario.nombre, &usuario.apellido) {
    (Some(nuevo_nombre), Some(nuevo_apellido)) => {
      query.set((
        nombre.eq(nuevo_nombre.clone()),
        apellido.eq(nuevo_apellido.clone())
      )).execute(conn)
    },
    (Some(nuevo_nombre), None) => {
      query.set(nombre.eq(nuevo_nombre.clone())).execute(conn)
    },
    (None, Some(nuevo_apellido)) => {
      query.set(apellido.eq(nuevo_apellido.clone())).execute(conn)
    },
    (None, None) => {
      println!("No se puede actualizar: ningún campo para modificar");
      Ok(0)
    }
  }
}
