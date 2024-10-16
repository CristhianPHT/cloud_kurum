use actix_web::{web, HttpResponse, Responder};
use diesel::prelude::*;
use diesel::r2d2::{ConnectionManager, Pool};
use crate::models::Usuario;
use crate::schema::usuariosss::dsl::{usuariosss, id};
use serde_json::json;
// use serde::Serialize; // Asegúrate de que esta importación esté presente

type DbPool = Pool<ConnectionManager<PgConnection>>;

pub async fn saludo(
    pool: web::Data<DbPool>,
    path: web::Path<i32>,) -> impl Responder {
    let user_id = path.into_inner(); // println!("ID del usuario recibido: {}", user_id);

    let conn = match pool.get() {
        Ok(mut conn) => {
            // println!("Conexión a la base de datos establecida correctamente."); // conectarse base de datos
            let usuarios = match usuariosss.filter(id.eq(user_id)).first::<Usuario>(&mut conn) {
            // let usuarios = match usuariosss.load::<Usuario>(&mut conn) {
                Ok(usuarios) => {
                    // println!("Usuarios obtenidos: {:?}\n", usuarios); // Mostrar usuarios en consola
                    usuarios // Retornamos los usuarios si todo está bien
                }
                Err(err) => {
                    println!("Error al obtener usuarios: {:?}", err); // Mostrar error en consola
                    return HttpResponse::InternalServerError().json(json!({
                        "error": "Error al obtener usuarios",
                        "details": err.to_string()
                    }));
                }
            };
            HttpResponse::Ok().json(json!({
                "user_id": user_id,
                "usuarios": usuarios
            }))
            // HttpResponse::Ok().body(format!("Identificado con ID: {}\nMis datos son: {:?}", user_id, usuarios)) // Devolvemos la respuesta con los usuarios y el ID
        },
        Err(_) => {
            println!("Error al obtener la conexión a la base de datos.");
            HttpResponse::InternalServerError().json(json!({
                "error": "Error al conectar a la base de datos"
            }))
        }
    };
    conn
}
pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::resource("/saludo/{id}/")
            .route(web::get().to(saludo)),  // Solo ejecuta la función
    );
}

// let result = web::block(move || usuariosss.load::<Usuario>(&conn));
// // let result = web::block(move || usuariosss.filter(id.eq(user_id)).first::<Usuario>(&conn)).await;

// -----------------------  Establecer una conección con la base de datos:
    // let conn = match pool.get() {     // conectar_data_base as conn
    //     Ok(conn) => {
    //         println!("Conexión a la base de datos establecida correctamente.");
    //         HttpResponse::Ok().body(format!("Conexion exitosa, ID: {}", user_id))
    //     },
    //     Err(_) => {
    //         println!("Error al obtener la conexión a la base de datos.");
    //         HttpResponse::InternalServerError().body("Error al conectar a la base de datos.")
    //     }
    // };