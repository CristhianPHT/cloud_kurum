use actix_web::{App, HttpServer};
// use serde_json::json;
use actix_cors::Cors;  // Importa Cors para habilitar CORS en la aplicación
use dotenv::dotenv;
use nube_kurum::web::interface::{ post_nuevo_libro_genero};
use nube_kurum::web::interface::{get_buscar_lib_gen,post_nuevo_genero}; //test
use nube_kurum::web::handlers::user::{
  insert_login, login_usuario, get_user_page, get_header, get_user, new_password, actualizar_user, logout_sesion};
use nube_kurum::web::handlers::book::{get_libro_all, get_libro_unique, post_nuevo_libro};
use nube_kurum::web::handlers::books_user::{get_all_books_user, get_books_x_user, post_books_x_user, get_libros_publicos_x_user};
// use nube_kurum::web::interface::{select_generica,insert_generica}; // no sirve por que es casi imposible genericos en orm de diesel

#[actix_web::main]
async fn main() -> std::io::Result<()> {
  dotenv().ok();
  println!("Iniciando el servidor en http://127.0.0.1:5330/");
  HttpServer::new(move || {
    App::new()
      .service(insert_login)  // Registro de usuario "/register" (post)
      .service(login_usuario)  // Iniciar sesión "/login" (post)  
      .service(get_user_page)   // Perfil público
      .service(get_user)  // Select * from Account; a traves de un token
      .service(get_header)
      .service(new_password)  // Actualizar datos de usuario "/login/{id}" (put)
      .service(actualizar_user)  // Generar token jwt... "/auth" (post) ... solo eso... utíl para api?
      .service(logout_sesion)
      // --------------------libro--------------------
      .service(get_libro_all)
      .service(get_libro_unique)
      .service(post_nuevo_libro)
      // -----------------libro usuario ---------------
      .service(get_all_books_user)
      .service(get_books_x_user)
      .service(post_books_x_user)
      .service(get_libros_publicos_x_user)
      // --------------- interface -------------------
      .service(post_nuevo_libro_genero)
      .service(get_buscar_lib_gen)
      .service(post_nuevo_genero)
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