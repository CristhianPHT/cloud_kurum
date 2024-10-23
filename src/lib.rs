
use diesel::prelude::*;
use dotenv::dotenv;
use std::env;
// modulos públicos para usarlo con name de package. //(nube_kurum)
pub mod models;
pub mod schema;
// Establece la conexión a la base de datos
pub fn establish_connection() -> PgConnection {
  dotenv().ok();
  let database_url = env::var("DATABASE_URL")
    .expect("Se debe configurar DATABASE_URL");
  PgConnection::establish(&database_url)
    .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}
