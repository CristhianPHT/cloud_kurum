use actix_web::{get, web, App, HttpResponse, HttpServer, Responder};
use actix_cors::Cors;  // Importa Cors
use serde::Serialize;

#[derive(Serialize)]
struct Usuario {
    nombre: String,
    apellido: String,
}

// #[get("/saludo/{id}")]
async fn saludo(path: web::Path<u32>) -> impl Responder {
    let id = path.into_inner();

    let usuario = match id {
        1 => Usuario {
            nombre: "Kurumi".to_string(),
            apellido: "Tokisaki".to_string(),
        },
        2 => Usuario {
            nombre: "Alice".to_string(),
            apellido: "Liddell".to_string(),
        },
        3 => Usuario {
            nombre: "Bob".to_string(),
            apellido: "Smith".to_string(),
        },
        _ => Usuario {
            nombre: "Usuario".to_string(),
            apellido: "Desconocido".to_string(),
        },
    };

    HttpResponse::Ok()
        .content_type("application/json")
        .json(usuario) // Devuelve la respuesta como JSON
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .wrap(Cors::permissive())  // Configuración permisiva para CORS
            .route("/saludo/{id}/", web::get().to(saludo)) // Ruta dinámica con ID
    })
    .bind("127.0.0.1:5330")? // Asegúrate de que esta línea esté bien
    .run()
    .await
}
