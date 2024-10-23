use nube_kurum::establish_connection;
use nube_kurum::models::Usuario; 
mod use_sql;
use use_sql::{select_all_users, select_id};


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
  println!("{:?}", select_id(&mut connection, 5));
}

