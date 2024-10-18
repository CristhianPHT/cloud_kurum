
use actix_cors::Cors;  // Importa Cors
// use serde::Serialize;

use actix_web::{get, web, App, HttpServer, HttpResponse, Responder};
use diesel::r2d2::{self, ConnectionManager};
use diesel::PgConnection;
use dotenv::dotenv;
use std::env;

mod models;
mod schema;
mod saludo;
use saludo::config as saludo_config; // Importamos la configuración del módulo saludo

mod nuevo_usuario;
use nuevo_usuario::insertar_usuario as insert_b;

type DbPool = r2d2::Pool<ConnectionManager<PgConnection>>;

#[get("/hello/{id}")]
async fn hello(path: web::Path<u32>) -> impl Responder {
    let id = path.into_inner();
    let nombre = match id {
        1 => "Kurumi",
        2 => "Alice",
        3 => "Bob",
        _ => "Usuario Desconocido",
    };
    HttpResponse::Ok().body(format!("Hola, {}!", nombre))
}

#[get("/test_connection")]
async fn test_connection(pool: web::Data<DbPool>) -> Result<HttpResponse, actix_web::Error> {
    let _conn = pool.get().map_err(|_| {
        HttpResponse::InternalServerError().body("Error al obtener la conexión")
    });
    
    println!("Conexión exitosa a la base de datos");
    Ok(HttpResponse::Ok().body("Conexion exitosa a la base de datos"))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL no está configurada");
    let manager = ConnectionManager::<PgConnection>::new(database_url);
    let pool = r2d2::Pool::builder()
        .build(manager)
        .expect("No se pudo crear el pool de conexiones");

    println!("Iniciando el servidor en http://127.0.0.1:5330/");
    
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .configure(saludo_config) // Configuramos las rutas del módulo saludo
            // .route("/saludo/{id}/", web::get().to(saludo)) // Ruta dinámica con ID
            .service(hello)
            .service(test_connection)   // usando macros #[get("/name_ruta")]
            // .service(new_user)
            // .route("/test_connection", web::get().to(test_connection)) // sin usar macros get
            .service(insert_b)
            .wrap(
                Cors::default() // Configuración de CORS
                    .allowed_origin("http://localhost:5173") // Cambia a la URL de tu frontend
                    .allowed_methods(vec!["GET", "POST"]) // Métodos permitidos
                    .allowed_headers(vec!["Content-Type"]) // Cabeceras permitidas
                    .max_age(3600), // Duración en segundos
            )
    })
    .bind("127.0.0.1:5330")?
    .run()
    .await
}
// ---------------------------------------------------

