use crate::establish_connection;
use actix_web::{get,post, put,web, HttpResponse, Responder};


#[get("/check")]
pub async fn health_check() -> impl Responder {
    // Intentar establecer la conexión con la base de datos
    match std::panic::catch_unwind(|| establish_connection()) {
        Ok(_) => HttpResponse::Ok().body("Conexión a la base de datos exitosa"),
        Err(_) => HttpResponse::InternalServerError().body("Error al conectar con la base de datos"),
    }
}

use crate::models::{NuevoUsuario, Usuario, UsuarioUpdate}; // , UsuarioUpdate
use crate::{select_all_users, select_id, insert_user, update_user_id}; // , update_user_id
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

#[post("/users")]
pub async fn create_user(user: web::Json<NuevoUsuario>) -> impl Responder {
  let mut conn = establish_connection();
  let nuevo_usuario = user.into_inner();
  let _identidad = insert_user(&mut conn, nuevo_usuario.clone());
  HttpResponse::Ok().json(json!({
      "usuario": nuevo_usuario
  }))
}

#[put("/users/{id}")]
pub async fn update_user(id: web::Path<i32>, user: web::Json<UsuarioUpdate>) -> impl Responder {
  let user_id = id.into_inner();
  let mut conn = establish_connection();
  let updated_user = update_user_id(&mut conn, user_id, user.into_inner());
  println!("{:?}", updated_user);
  match updated_user {
    Ok(rows_updated) => HttpResponse::Ok().json(json!({
        // "usuario": user.into_inner().nombre.clone(), // Error no puede mostrar el nombre
        "filas_actualizadas": rows_updated
    })),
    Err(_) => HttpResponse::InternalServerError().json(json!({
        "error": "Error al actualizar el usuario"
    }))
}
}
