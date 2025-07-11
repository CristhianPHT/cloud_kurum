use crate::establish_connection;
use actix_web::{get,post, put,web, HttpResponse, Responder};


#[get("/check")]    // Comprobar la conexión con la base de datos con diesel/rust
pub async fn health_check() -> impl Responder {
    // Intentar establecer la conexión con la base de datos
    match std::panic::catch_unwind(|| establish_connection()) {
        Ok(_) => HttpResponse::Ok().body("Conexión a la base de datos exitosa"),
        Err(_) => HttpResponse::InternalServerError().body("Error al conectar con la base de datos"),
    }
}

use crate::models::{Libro, NuevoLibro, NuevoUsuario, Usuario, UsuarioUpdate}; // , UsuarioUpdate
use crate::{select_all_users, select_id, insert_user, update_user_id}; // , update_user_id
use serde_json::json;

#[get("/test")]   // Se busca a todos los usuarios  en una lista de usuarios... (select_all_users)
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

#[get("/test/{id}")]    // Se busca un usuario a través del id del usuario en el link (select_id)
pub async fn show_user(id: web::Path<i32>) -> impl Responder {
    let user_id = id.into_inner();
    let mut conn = establish_connection();
    let user_result: Result<Usuario, diesel::result::Error> = select_id(&mut conn, user_id);
    match user_result {
        Ok(user) => HttpResponse::Ok().json(json!({
            "usuario": user
        })),
        Err(_) => HttpResponse::NotFound().json(json!({
            "error": "Usuario no encontrado"
        })),
    }
}

#[post("/test")]    // Se ingresa un nuevo usuario (class=NuevoUsuario, insert_user)
pub async fn create_user(user: web::Json<NuevoUsuario>) -> impl Responder {
  let mut conn = establish_connection();
  let nuevo_usuario = user.into_inner();
  let _identidad = insert_user(&mut conn, nuevo_usuario.clone());
  HttpResponse::Ok().json(json!({
      "usuario": nuevo_usuario
  }))
}

#[put("/test/{id}")]    // Se actualiza un usuario existente con update user
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
// --------------------------------------------------------------------------------------------
//  select_id_token   Se encarga de las acciones con la autenticación obtenida
#[post("/usuario")]
pub async fn show_login(req: HttpRequest) -> impl Responder {
    // Leer el token del encabezado Authorization
  let token_input = match req.headers().get("Authorization") {
    Some(header_value) => {
      let token_str = header_value.to_str().unwrap_or("").trim();
      token_str.strip_prefix("Bearer ").unwrap_or(token_str).to_string()
    },
    None => return HttpResponse::Unauthorized().body("Token no proporcionado"),
  };
  let mut conn = establish_connection();
  let id_usuario_find = match select_id_token(&mut conn, token_input) {
    Ok(id) => id,
    Err(_) => return HttpResponse::Unauthorized().body("Token inválido o expirado"),
};
  let usuario_encontrado = select_id_usuario(&mut conn, id_usuario_find);
  HttpResponse::Ok().json(json!({
    "usuario": usuario_encontrado
  }))
}
// --------------------------------------------------------------------------------------------
// insert_auth_token con username y password para obtener el authtoken
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
// --------------------------------------------------------------------------------------------
// ingresar usuario sin token (insert usuario), retorna los mismos datos (no debería?)
#[post("/register")]
pub async fn insert_login(user: web::Json<NuevoAccount>) -> impl Responder {
  let mut conn = establish_connection();
  let usuario_all = user.into_inner();
  let _identidad = insert_usuario(&mut conn, usuario_all.clone());
  HttpResponse::Ok().json(json!({
      "usuario": usuario_all
  }))
}
// --------------------------------------------------------------------------------------------
// Actualizar los datos de usuario (update login)
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
// insert_auth_token Se crea un nuevo token... solo eso...
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
// use crate::{select_by_id, generic_insert};
// use crate::select_by_id;
// // use crate::schema::{libro, usuario, usuariosss};
// use crate::schema::usuariosss;
// // use crate::usuariosss;
// #[get("/generica/{id}")]
// pub async fn select_generica(id: web::Path<i32>) -> impl Responder {
//     let mut conn = establish_connection();
//     let user_id = id.into_inner();
    
//     match select_by_id(usuariosss::table, &mut conn, user_id) {
//       Ok(user) => HttpResponse::Ok().json(user),
//       Err(diesel::result::Error::NotFound) => {
//           HttpResponse::NotFound().json(json!({"error": "Usuario no encontrado"}))
//       },
//       Err(_) => HttpResponse::InternalServerError().finish(),
//   }
// }

// #[post("/generica")]
// pub async fn insert_generica(user: web::Json<NuevoUsuario>) -> impl Responder {
//   let mut conn = establish_connection();
//   let nuevo_usuario = user.into_inner();
//   let _identidad = generic_insert(&mut conn, nuevo_usuario.clone());
//   HttpResponse::Ok().json(json!({
//       "usuario": nuevo_usuario
//   }))
// }

// ---------------------------------------------------------------------------------------------
// -----------------------------------------libros----------------------------------------------
// use crate::models::{NuevoLibro, Libro};

// #[get("/generica/{id}")]
// pub async fn get_libro_data(id: web::Path<i32>) -> impl Responder {
//   let mut conn = establish_connection();
//   let user_id = id.into_inner();
//   let libro: Libro = select_by_id(libro, &mut conn, user_id);
//   HttpResponse::Ok().json(json!({
//     "libros": libro
//   }))
// }

// #[post("/generica")]
// pub async fn insert_libro(user: web::Json<NuevoLibro>) -> impl Responder {
//   let mut conn = establish_connection();
//   let nuevo_usuario = user.into_inner();
//   let _identidad = insert_user(&mut conn, nuevo_usuario.clone());
//   HttpResponse::Ok().json(json!({
//       "usuario": nuevo_usuario
//   }))
// }