use actix_web::{App, HttpServer};
// use serde_json::json;
use actix_cors::Cors;  // Importa Cors para habilitar CORS en la aplicación
use dotenv::dotenv;
use nube_kurum::web::interface::{health_check, show_users, show_user, create_user, update_user};

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
          .service(create_user)
          .service(update_user)
          .wrap(
            Cors::default() // Configuración de CORS
              .allowed_origin("http://localhost:5173") // Cambia a la URL de tu frontend
              .allowed_methods(vec!["GET", "POST", "PUT"]) // Métodos permitidos
              .allowed_headers(vec!["Content-Type"]) // Cabeceras permitidas
              .max_age(3600), // Duración en segundos
          )
    })
    .bind("127.0.0.1:5330")?
    .run()
    .await
}