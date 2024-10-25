use nube_kurum::establish_connection;
use nube_kurum::models::{Usuario, UsuarioUpdate}; 
mod use_sql;
use use_sql::{select_all_users, select_id, update_user_id};


pub fn main() {
  let mut connection = establish_connection();
  let lista_usuarios: Vec<Usuario> = select_all_users(&mut connection, 1);
  println!("Displaying {} usuarios", lista_usuarios.len());
  for usuario in &lista_usuarios {
    println!("ID: {}", usuario.id);
    println!("Nombre: {}", usuario.nombre);
    println!("Apellido: {}", usuario.apellido);
  }
  let user: Usuario = select_id(&mut connection, 10);
  println!("{:?}",user);
  let actualizar :UsuarioUpdate = UsuarioUpdate {
    // nombre: None,
    nombre: Some(String::from("Ban")),
    // apellido: Some(String::from("Yo Wun")),
    apellido: None,
};
  // println!("{:?}", update_user_id(&mut connection, 8, actualizar));
  update_user_id(&mut connection, 8, actualizar);
  println!("{:?}", select_id(&mut connection, 8));
}

