use crate::establish_connection;
use actix_web::{get, HttpResponse, Responder};

#[get("/check")]
async fn health_check() -> impl Responder {
    // Intentar establecer la conexión con la base de datos
    match std::panic::catch_unwind(|| establish_connection()) {
        Ok(_) => HttpResponse::Ok().body("Conexión a la base de datos exitosa"),
        Err(_) => HttpResponse::InternalServerError().body("Error al conectar con la base de datos"),
    }
}