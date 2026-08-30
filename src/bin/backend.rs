use actix_web::{web, App, HttpServer};
// use serde_json::json;
use actix_cors::Cors;  // Importa Cors para habilitar CORS en la aplicación
use dotenv::dotenv;
use nube_kurum::web::interface::{ post_nuevo_libro_genero};
use nube_kurum::web::interface::{get_buscar_lib_gen,post_nuevo_genero}; //test
use nube_kurum::web::handlers::user::{
  insert_login, login_usuario, get_user_page, get_header, get_user, new_password, actualizar_user, logout_sesion};
use nube_kurum::web::handlers::book::{get_libro_all, get_libro_unique, post_nuevo_libro, get_libro_slug};
use nube_kurum::web::handlers::books_state::{post_nuevo_lib_tip, get_lib_tip_unique, get_lib_tip_all, put_lib_tip, delete_lib_tip_web};
use nube_kurum::web::handlers::books_state::{post_nuevo_lib_est, get_lib_est_unique, get_lib_est_all, put_lib_est, delete_lib_est_web};
use nube_kurum::web::handlers::books_user::{get_all_books_user, get_books_x_user, post_books_x_user, get_libros_publicos_x_user};
// use nube_kurum::web::interface::{select_generica,insert_generica}; // no sirve por que es casi imposible genericos en orm de diesel

use nube_kurum::infrastructure::r2::create_r2_client;
use nube_kurum::web::handlers::image_r2::{test_r2};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
  dotenv().ok();
  let r2_client = create_r2_client();
  println!("Iniciando el servidor en http://127.0.0.1:5330/");
  HttpServer::new(move || {
    App::new()
      .app_data(web::Data::new(r2_client.clone()))
      .service(test_r2)
      .service(insert_login)  // Registro de usuario "/register" (post)
      .service(login_usuario)  // Iniciar sesión "/login" (post)  
      .service(get_user_page)   // Perfil público
      .service(get_user)  // Select * from Account; a traves de un token
      .service(get_header)
      .service(new_password)  // Actualizar datos de usuario "/login/{id}" (put)
      .service(actualizar_user)  // Generar token jwt... "/auth" (post) ... solo eso... utíl para api?
      .service(logout_sesion)
      // --------------------libro--------------------
      .service(get_libro_all)  // Conseguir todos los libros, por página
      .service(get_libro_slug)  // Conseguir un libro por slug
      .service(post_nuevo_libro)  // Insertar un nuevo libro
      .service(get_libro_unique)  // Conseguir un libro por slug
      // ----------------- libro tipo -----------------
      .service(post_nuevo_lib_tip)
      .service(get_lib_tip_unique)
      .service(get_lib_tip_all)
      .service(put_lib_tip)
      .service(delete_lib_tip_web)
      // ----------------- libro estado ---------------
      .service(post_nuevo_lib_est)
      .service(get_lib_est_unique)
      .service(get_lib_est_all)
      .service(put_lib_est)
      .service(delete_lib_est_web)
      // -----------------libro usuario ---------------
      .service(get_all_books_user)  // Conseguir todos los libros del usuario
      .service(get_books_x_user)  // Conseguir todos los libros del usuario v2 con imagenes
      .service(post_books_x_user)  // Insertar conexión libro usuario v1, falta diversificar...
      .service(get_libros_publicos_x_user)  // Conseguir todos los libros públicos del usuario
      // --------------- interface -------------------
      .service(post_nuevo_libro_genero)
      .service(get_buscar_lib_gen)
      .service(post_nuevo_genero)
      // .wrap(Cors::permissive()) // para probar de todo y sin permisos
      .wrap(
          Cors::default() // Configuración de CORS
            .allowed_origin("http://localhost:5173") // Cambia a la URL de tu frontend
            .allowed_methods(vec!["GET", "POST", "PUT", "DELETE"]) // Métodos permitidos
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