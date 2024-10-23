use nube_kurum::establish_connection;
mod consulta_sql;
use consulta_sql::mostrar_usuarios as mmss;
fn main(){
  mmss();
  let _mostrar = &mut establish_connection();
  println!("Hola mundo, conectando a la base de datos...");
}