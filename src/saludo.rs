use actix_web::{web, HttpResponse, Responder};
use diesel::prelude::*;
use diesel::r2d2::{ConnectionManager, Pool};
use crate::models::Usuario;
use crate::schema::usuariosss::dsl::{usuariosss, id};

type DbPool = Pool<ConnectionManager<PgConnection>>;

pub async fn saludo(
    pool: web::Data<DbPool>,
    path: web::Path<i32>,
) -> impl Responder {
    let user_id = path.into_inner();
    println!("ID del usuario recibido: {}", user_id);
    
    // Intentar obtener una conexión a la base de datos
    let conn = match pool.get() {     // conectar_data_base as conn
        Ok(conn) => {
            println!("Conexión a la base de datos establecida correctamente.");
            HttpResponse::Ok().body(format!("Conexion exitosa, ID: {}", user_id))
        },
        Err(_) => {
            println!("Error al obtener la conexión a la base de datos.");
            HttpResponse::InternalServerError().body("Error al conectar a la base de datos.")
        }
    };
    // let user_id = path.into_inner();
    // let conn = pool.get().expect("No se pudo obtener la conexión");

    // let result = web::block(move || usuariosss.filter(id.eq(user_id)).first::<Usuario>(&conn)).await;

    // match result {
    //     Ok(usuario) => HttpResponse::Ok().json(usuario),
    //     Err(_) => HttpResponse::NotFound().body("Usuario no encontrado"),
    // }
    conn
}

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(web::resource("/saludo/{id}").route(web::get().to(saludo)));
}


// ----------------------- ahora otra versión de gpt ¿mejorado?