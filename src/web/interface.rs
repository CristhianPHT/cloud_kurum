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

#[get("/test")]
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

#[get("/test/{id}")]
pub async fn show_user(id: web::Path<i32>) -> impl Responder {
  let user_id = id.into_inner();
  let mut conn = establish_connection();
  let user: Usuario = select_id(&mut conn, user_id);
  HttpResponse::Ok().json(json!({
    "usuario": user
  }))
}

#[post("/test")]
pub async fn create_user(user: web::Json<NuevoUsuario>) -> impl Responder {
  let mut conn = establish_connection();
  let nuevo_usuario = user.into_inner();
  let _identidad = insert_user(&mut conn, nuevo_usuario.clone());
  HttpResponse::Ok().json(json!({
      "usuario": nuevo_usuario
  }))
}

#[put("/test/{id}")]
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
// --------------------------------------------------------------------------------------------
use crate::models::{NuevoAccount, Account, LoginAccount};
use crate::{insert_usuario, select_id_usuario, update_login, login_usuario_hashed, calculate_expiration, generate_jwt, insert_auth_token, select_id_token};
use actix_web::HttpRequest;
#[get("/login/{id}")]
pub async fn show_login(id: web::Path<i32>, req: HttpRequest) -> impl Responder {
  let user_id = id.into_inner();
    // Leer el token del encabezado Authorization
  let token_input = match req.headers().get("Authorization") {
    Some(header_value) => header_value.to_str().unwrap_or("").replace("Bearer ", ""),
    None => return HttpResponse::Unauthorized().body("Token no proporcionado"),
  };
  let mut conn = establish_connection();
  let user: Account = select_id_usuario(&mut conn, user_id);
  let _id_token_output = select_id_token(&mut conn, token_input);
  HttpResponse::Ok().json(json!({
    // "token": token_output, // Clonamos token_input para evitar el error de movimiento
    "usuario": user
  }))
}

#[post("/login")]
pub async fn login_usuario(user: web::Json<LoginAccount>) -> impl Responder {
  let mut conn = establish_connection();
  let usuario_login = user.into_inner();
  let identidad = login_usuario_hashed(&mut conn, usuario_login.username.as_str(), usuario_login.password_hash.as_str());
  match identidad {
    Ok(identidad)
      if identidad != 0 => {
        let expira = calculate_expiration();
        let token = generate_jwt(identidad, expira);
        let _data_base = insert_auth_token(&mut conn, identidad, token.clone(), expira);
        HttpResponse::Ok().json(json!({
          "token": token.clone()
        }))
      },
    Ok(_) => HttpResponse::Unauthorized().json(json!({
      "error": "Usuario o contraseña incorrectos"
    })),
    Err(_) => HttpResponse::InternalServerError().json(json!({
        "error": "Error al autenticar el usuario"
    })),
  }
}

#[post("/login_all")]
pub async fn insert_login(user: web::Json<NuevoAccount>) -> impl Responder {
  let mut conn = establish_connection();
  let usuario_all = user.into_inner();
  let _identidad = insert_usuario(&mut conn, usuario_all.clone());
  HttpResponse::Ok().json(json!({
      "usuario": usuario_all
  }))
}
#[put("/login/{id}")]
pub async fn update_usuario_login(id: web::Path<i32>, user: web::Json<NuevoAccount>) -> impl Responder {
  let user_id = id.into_inner();
  let mut conn = establish_connection();
  let updated_user = update_login(&mut conn, user_id, user.into_inner());
  println!("{:?}", updated_user);
  match updated_user {
    Ok(rows_updated) => HttpResponse::Ok().json(json!({
        "filas_actualizadas": rows_updated
    })),
    Err(_) => HttpResponse::InternalServerError().json(json!({
        "error": "Error al actualizar el usuario"
    }))
  }
}
// --------------------------------------------------------------------------------------------
use crate::models::NuevoAuthToken;
#[post("/auth")]
pub async fn auth_user(user: web::Json<NuevoAuthToken>) -> impl Responder {
  let mut conn = establish_connection();

  let auth_token = user.into_inner();
  let expira = calculate_expiration();
  let token = generate_jwt(auth_token.user_id, expira);
  let _data_base = insert_auth_token(&mut conn, auth_token.user_id, token.clone(), expira);
  HttpResponse::Ok().json(json!({
      "auth_token": token.clone()
  }))
}

// ---------------------------------------------------------------------------------------------
// Generica
#[get("/generica/{id}")]
pub async fn select_generica(id: web::Path<i32>) -> impl Responder {
  let mut conn = establish_connection();
  let user_id = id.into_inner();
  let user: Usuario = select_id(&mut conn, user_id);
  HttpResponse::Ok().json(json!({
    "usuario": user
  }))
}

#[post("/generica")]
pub async fn insert_generica(user: web::Json<NuevoUsuario>) -> impl Responder {
  let mut conn = establish_connection();
  let nuevo_usuario = user.into_inner();
  let _identidad = insert_user(&mut conn, nuevo_usuario.clone());
  HttpResponse::Ok().json(json!({
      "usuario": nuevo_usuario
  }))
}