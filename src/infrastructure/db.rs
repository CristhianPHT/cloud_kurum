use diesel::prelude::*;
use dotenv::dotenv;
use std::env;

// Establece la conexión a la base de datos
pub fn establish_connection() -> PgConnection {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("Se debe configurar DATABASE_URL");
    PgConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}