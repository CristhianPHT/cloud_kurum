use actix_web::{put, web, HttpResponse, Responder};
use diesel::prelude::*;
use serde_json::json;
use crate::schema::usuariosss::dsl::*;
use crate::DbPool;
use crate::models::UsuarioUpdate;

/// Estructura para recibir los datos de actualización


/// Ruta para modificar un usuario en la tabla usuariosss
#[put("/modificar_usuario/{id}")]
pub async fn modificar_usuario(
  pool: web::Data<DbPool>,
  usuario: web::Json<UsuarioUpdate>,
  path: web::Path<i32>, // Recibimos el ID del usuario en la ruta
) -> impl Responder {
  let usuario_id = path.into_inner();
  let conn = match pool.get() {
    Ok(mut conn) => {
      let resultado = match diesel::update(usuariosss.filter(id.eq(usuario_id)))
          .set((
            nombre.eq(usuario.nombre.clone().unwrap_or_default()),
            apellido.eq(usuario.apellido.clone().unwrap_or_default()),
          )).execute(&mut conn)
        {
          Ok(0) => HttpResponse::NotFound().json(json!({"error": "Usuario no encontrado"})),
          Ok(_) => {HttpResponse::Ok().json(json!({
            "nombre": usuario.nombre.clone(),
            "apellido": usuario.apellido.clone()
          }))}
          Err(e) => HttpResponse::InternalServerError().json(json!({
            "Error":"Error al ingresar usuarios",
            "Detalles": e.to_string()
          })),
        };
        println!("Modificando usuario. {}",usuario_id);
        resultado
      }, Err(_) => {
        println!("Error conectandose con la base de datos.");
        HttpResponse::InternalServerError().json(json!({
          "error": "Error al conectar a la base de datos"
      }))}
    };
    conn
}
