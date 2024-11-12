use crate::establish_connection;
use actix_web::{get,web, HttpResponse, Responder};


#[get("/check")]
pub async fn health_check() -> impl Responder {
    // Intentar establecer la conexión con la base de datos
    match std::panic::catch_unwind(|| establish_connection()) {
        Ok(_) => HttpResponse::Ok().body("Conexión a la base de datos exitosa"),
        Err(_) => HttpResponse::InternalServerError().body("Error al conectar con la base de datos"),
    }
}

use crate::models::Usuario; // , UsuarioUpdate
use crate::{select_all_users, select_id}; // , update_user_id
use serde_json::json;

#[get("/users")]
pub async fn show_users() -> impl Responder {
  let mut conn = establish_connection();
  let lista_usuarios: Vec<Usuario> = select_all_users(&mut conn, 0);
  let respuesta: HttpResponse ;
  if lista_usuarios.is_empty() {
    println!("No se encontraron usuarios");
    respuesta = HttpResponse::NotFound().json(json!({
    // HttpResponse::NotFound().json(json!({
      "error": "No se encontraron usuarios"
    }))
  } else {
    respuesta = HttpResponse::Ok().json(json!({
      "usuarios": lista_usuarios
    }));
    // let usuarios_json = lista_usuarios.iter()
    //   .map(|usuario| json!({
    //     "id": usuario.id,
    //     "nombre": usuario.nombre,
    //     "apellido": usuario.apellido
    //   })
    //   .to_string()).collect::<Vec<String>>().join(",");
    // respuesta = HttpResponse::Ok()
    //   .content_type("application/json")
    //   .body(format!("{{{}}}", usuarios_json));
  }
  respuesta
}

#[get("/users/{id}")]
pub async fn show_user(id: web::Path<i32>) -> impl Responder {
  let user_id = id.into_inner();
  let mut conn = establish_connection();
  let user: Usuario = select_id(&mut conn, user_id);
  HttpResponse::Ok().json(json!({
    "usuario": user
  }))
}

// #[post("/users")]
// pub async fn create_user(user: web::Json<Usuario>) -> impl Responder {
//   let mut conn = establish_connection();
//   insert_user(&mut conn, user.into_inner());
//   HttpResponse::Created().json(json!({
//     "usuario": user
//   }))
// }