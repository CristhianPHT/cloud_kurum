// src/nuevo_usuario.rs
use actix_web::{post, web, HttpResponse, Responder};
use serde_json::json;
use diesel::prelude::*;
use crate::models::NuevoUsuario;

use crate::schema::usuariosss::dsl::usuariosss;

use crate::DbPool;

/// Ruta que inserta un nuevo usuario en la tabla usuariosss
#[post("/insertar_usuario")]
pub async fn insertar_usuario(pool: web::Data<DbPool>,usuario: web::Json<NuevoUsuario>) -> impl Responder {
    // Creamos un nuevo usuario manualmente
    let nuevo_usuario = NuevoUsuario {
        nombre: usuario.nombre.clone(),
        apellido: usuario.apellido.clone()
        // nombre: "Mayda".to_string(),
        // apellido: "Figueroa".to_string(),
    };
    let conn = match pool.get(){
        Ok(mut conn) => {
            let _usuario = match diesel::insert_into(usuariosss)
                                .values(&nuevo_usuario).execute(&mut conn)
            {
                Ok(usuario) => {usuario}
                Err(e) => {
                    println!("Error al ingresar usuario: {:?}", e);
                    return HttpResponse::InternalServerError().json(json!({
                        "Error":"Error al ingresar usuarios",
                        "Detalles": e.to_string()
                    }));
                }
            };
            println!("Usuario insertado exitosamente.");
            HttpResponse::Ok().json(json!({
                "user_name": &nuevo_usuario
            }))
        }, Err(_) => {
            println!("Error al obtener la conexión a la base de datos.");
            HttpResponse::InternalServerError().json(json!({
                "error" : "Error al conectar a la base de datos"
            }))
        }
    };
    conn
}

