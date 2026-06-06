use actix_web::{App, HttpServer};
// use serde_json::json;
use actix_cors::Cors;  // Importa Cors para habilitar CORS en la aplicación
use dotenv::dotenv;
use nube_kurum::web::interface::{health_check, show_users, show_user, create_user, update_user}; // falta agregar get_user_images y view_r2_image
use nube_kurum::web::interface::{insert_login, update_usuario_login, login_usuario};
use nube_kurum::web::interface::{get_libro_all, post_nuevo_libro, get_libro_unique, post_nuevo_libro_genero};
use nube_kurum::web::interface::{get_buscar_lib_gen,post_nuevo_genero}; //test
use nube_kurum::web::interface::auth_user;
use nube_kurum::web::handlers::user::{get_header, get_user, get_user_page};
use nube_kurum::web::handlers::books_user::{get_books_x_user, post_books_x_user, get_all_books_user, get_libros_publicos_x_user};
// use nube_kurum::web::interface::{select_generica,insert_generica}; // no sirve por que es casi imposible genericos en orm de diesel

#[actix_web::main]
async fn main() -> std::io::Result<()> {
  dotenv().ok();
  println!("Iniciando el servidor en http://127.0.0.1:5330/");
  HttpServer::new(move || {
    App::new()
      // .service(health_check)  // Agregar el endpoint de health_check
      .service(health_check)  // Agregar health_check, verificación de acceso a la base de datos
      //.service(get_user_images) // Obtener imágenes del usuario
      // .service(view_r2_image) // Ver imagen en R2 por GET
      .service(show_users)  // tutorial
      .service(show_user)   // tutorial
      .service(create_user)   // tutorial
      .service(update_user)    // tutorial
      .service(insert_login)  // Registro de usuario "/register" (post)
      .service(update_usuario_login)  // Actualizar datos de usuario "/login/{id}" (put)
      .service(auth_user)  // Generar token jwt... "/auth" (post) ... solo eso... utíl para api?
      .service(login_usuario)  // Iniciar sesión "/login" (post)  
      .service(get_header)
      .service(get_libro_all) 
      .service(get_libro_unique)
      .service(post_nuevo_libro)
      .service(post_nuevo_libro_genero)
      .service(get_buscar_lib_gen)
      .service(post_nuevo_genero)
      .service(get_user)  // Select * from Account; a traves de un token
      .service(get_user_page)
      .service(get_books_x_user)
      .service(post_books_x_user)
      .service(get_all_books_user)
      .service(get_libros_publicos_x_user)
      // .wrap(Cors::permissive()) // para probar de todo y sin permisos
      .wrap(
          Cors::default() // Configuración de CORS
            .allowed_origin("http://localhost:5173") // Cambia a la URL de tu frontend
            .allowed_methods(vec!["GET", "POST", "PUT"]) // Métodos permitidos
            .allowed_headers(vec!["Authorization", "Content-Type"]) // Cabeceras permitidas
            .max_age(3600), // Duración en segundos
        )
      })
      .bind("127.0.0.1:5330")?
      .run()
      .await
}
// .service(select_generica)
// .service(insert_generica)