use nube_kurum::establish_connection;
pub mod use_sql;
use use_sql::update_user_id;

use nube_kurum::models::UsuarioUpdate;
// mod consulta_sql;
// use consulta_sql::mostrar_usuarios as mmss;
fn main(){
  // mmss();
  let mostrar = &mut establish_connection();
  println!("Hola mundo, conectando a la base de datos...");
  let _ = update_user_id(mostrar, 1, UsuarioUpdate {
    nombre: Some(String::from("Juan")),
    apellido: Some(String::from("Pérez")),
  });
}
