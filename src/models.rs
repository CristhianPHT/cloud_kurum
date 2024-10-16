
use serde::Serialize;
use diesel::Queryable;
// use diesel::prelude::*; // eliminar ???

#[derive(Queryable, Serialize, Debug)]
pub struct Usuario {
    pub id: i32,
    pub nombre: String,
    pub apellido: String,
}

// -----------
// despues de crear eso ahora lo usamos?
// use actix_web::{web, HttpResponse, Responder};
// use diesel::prelude::*;

// #[post("/usuariosss")]
// async fn create_usuario(pool: web::Data<DbPool>, new_usuario: web::Json<NewUsuario>) -> impl Responder {
//     let conn = pool.get().map_err(|_| {
//         HttpResponse::InternalServerError().body("Error al obtener la conexión")
//     })?;

//     let new_user = NewUsuario {
//         nombre: new_usuario.nombre.clone(),
//         apellido: new_usuario.apellido.clone(),
//     };

//     // Insertar el nuevo usuario
//     diesel::insert_into(usuariosss::table)
//         .values(&new_user)
//         .execute(&conn)
//         .map_err(|_| HttpResponse::InternalServerError().body("Error al insertar el usuario"))?;

//     HttpResponse::Created().finish()
// }
