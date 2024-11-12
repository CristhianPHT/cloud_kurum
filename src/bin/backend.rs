use actix_web::{App, HttpServer};
// use serde_json::json;
// use actix_cors::Cors;  // Importa Cors para habilitar CORS en la aplicación
use dotenv::dotenv;
use nube_kurum::web::interface::{health_check, show_users, show_user};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    println!("Iniciando el servidor en http://127.0.0.1:5330/");

    HttpServer::new(move || {
        App::new()
            // .service(health_check)  // Agregar el endpoint de health_check
            .service(health_check)  // Agregar el endpoint de health_check
            .service(show_users)
            .service(show_user)
    })
    .bind("127.0.0.1:5330")?
    .run()
    .await
}