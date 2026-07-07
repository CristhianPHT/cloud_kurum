use serde::{Deserialize, Serialize};
use diesel::{AsChangeset, Insertable, Queryable, Selectable};
#[allow(unused_imports)]
use crate::schema::{usuario, token_recuperacion, auth_tokens, imagen_perfil};   // Login (usuario)

use chrono::NaiveDateTime;    // O NaiveDateTime si usas timestamps con zona horaria

// ------------------- Usuario real - Usuario - real --------------------------------
#[derive(Queryable, Serialize, Debug, Deserialize)]
#[diesel(table_name = usuario)]
pub struct LoginAccount {   // Logearse legalmente como usuario (post)
    // pub id: Option<i32>,
    pub username: String,
    pub password: String,
}

#[derive(Queryable, Identifiable, Selectable)]
#[diesel(table_name = usuario)]
pub struct NiceAccount {    // Temporalmente con todos los atributos de usuario|account que será usado para lectura usando el id 
    pub id: i32,
    pub nickname: String,
    pub username: String,
    pub password_hash: String,
    pub email: String,
    // pub is_active: bool, 
    pub is_active: bool,   // validación para cuentas antiguas y eliminación...
    pub updated_at: NaiveDateTime,  // Fecha de actualización
    pub created_at: NaiveDateTime   // Fecha de creación
}

#[derive(Queryable, Serialize)]
#[diesel(table_name = usuario )]
pub struct HeaderAccount {
    pub nickname: String,   // apodo
    pub url_image: Option<String>     // imagen (portada/icon/foto de perfil)
}

#[derive(Insertable, Deserialize, Serialize, Clone, AsChangeset)]  // Agregamos Deserialize, Serialize para recibir y enviar objetos JSON
#[diesel(table_name = usuario)]
pub struct NuevoAccount {  // Struct para insertar datos en la base de datos (INSERT, UPDATE) (post, put) (Para nuevos usuario y para configuración o edit de perfil)
    pub nickname: Option<String>,       // Apodo
    pub username: String,       // gmail, o con lo que ingresará por arriba ---> LoginAccount ...
    pub password_hash: String,      // Contraseña o con lo que ingresará por abajo ---> LoginAccount ...
    pub email: String,      // gmail, para recuperacion de la cuenta
    pub is_active: bool,
    pub updated_at: NaiveDateTime,   // Última actualización hecho sobre la cuenta
    pub created_at: NaiveDateTime
}
#[derive(Insertable)]
#[diesel(table_name = imagen_perfil)]
pub struct NewImagenPerfil {
    pub user_id: i32,
    pub url_image: String,
    pub tipo: String,
    pub nombre: String,
    pub is_active: bool,
    pub mime_type: String,
    pub tamano_bytes: i64,
    pub ancho: Option<i32>,
    pub alto: Option<i32>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime
}
#[derive(Queryable)]
pub struct ImagenPerfil{
    pub id: i32,
    pub user_id: i32,
    pub url_image: String,
    pub tipo: String,
    pub nombre: String,
    pub is_activo: bool,
    pub mime_type: String,
    pub tamano_bytes: i64,
    pub ancho: i32,
    pub alto: i32,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime
}
// *-*-*-*-*-*-*-*-* Finalización para el manejo de la Cuenta *-*-*-*-*-*-*-*-*
// ------------------- Clave para poder recuperar cuenta -------------------
#[derive(Queryable, Serialize, Debug)]  // Queryable para obtener datos de la base de datos con ID 
pub struct Recuperacion {  // Struct para obtener datos de la base de datos (SELECT)
    pub id: i32,        // id de la tabla de recuperacion
    pub user_id: i32,       // id del usuario a recuperar...?
    pub token: String,      // token para su ingreso...?
    pub tipo: bool,     // true si la recuperación es válida
    pub expira: NaiveDateTime,      // fecha de expiración de la cable para poder recuperar...?
    // #[serde(with = "chrono::serde::ts_seconds")]
    // pub expira: DateTime<Utc>,
}

#[derive(Insertable, Deserialize, Serialize, Clone)]  // Agregamos Deserialize, Serialize para recibir y enviar objetos JSON
#[diesel(table_name = token_recuperacion)]
pub struct NuevoRecuperacion {  // Struct para insertar datos en la base de datos (INSERT)
    pub user_id: i32,
    pub token: String,
    pub tipo: String, // era bool?
    pub expira: NaiveDateTime,
}
// *-*-*-*-*-*-*-*-* Finalización para la recuperación *-*-*-*-*-*-*-*-*


// ------------------- Tokenización para seguridad -------------------
#[derive(Insertable, Deserialize, Serialize, AsChangeset, Clone)]  // Agregamos Deserialize, Serialize para recibir y enviar objetos JSON
#[diesel(table_name = auth_tokens)]
pub struct NuevoAuthToken {  // Struct para insertar datos en la base de datos (INSERT)
    pub user_id: i32,
    pub token: String,
    pub dispositivo: Option<String>,
    pub expira: NaiveDateTime,
    pub is_active: bool,
}
#[derive(Queryable, Serialize, Debug)] // Select de los datos
#[diesel(table_name = auth_tokens)]
pub struct AuthToken {
    pub id: i32,
    pub user_id: i32,
    pub token: String,
    pub dispositivo: Option<String>,
    pub expira: NaiveDateTime,
    pub is_active: bool,
}
#[derive(Serialize, Deserialize)] // no base de datos.
pub struct Claims {
    pub sub: i32,      // user_id
    pub exp: usize,    // Expiration time
    pub iss: String,   // Issuer (tu dominio)
}
/// *-*-*-*-*-*-*-*-* Finalización de la tokenización de la seguridad *-*-*-*-*-*-*-*-*
  
  
//   CREATE TABLE sessions (
//     id SERIAL PRIMARY KEY,
//     user_id INT NOT NULL,
//     dispositivo VARCHAR,
//     direccion_ip VARCHAR,
//     inicio TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
//     actualizacion TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
//     CONSTRAINT fk_user FOREIGN KEY (user_id) REFERENCES usuarios (id)
//   );
  
//   CREATE TABLE multidispositivos (
//     id SERIAL PRIMARY KEY,
//     user_id INT NOT NULL,
//     dispositivo_nombre VARCHAR NOT NULL,
//     device_tipo VARCHAR,
//     confianza BOOLEAN NOT NULL DEFAULT false,
//     CONSTRAINT fk_user FOREIGN KEY (user_id) REFERENCES usuarios (id)
//   );


// Biblioteca todo lo de abajo. ------------------------------
use crate::schema::libro;   // Biblioteca all

// use diesel::prelude::*;
// use serde::Serialize;

#[derive(Queryable, Serialize)]
#[diesel(table_name = libro)]
pub struct LibroDashboard { // Dashboard de libros, para mostrar en la pagina principal
    pub id: i32,
    pub titulo: String,
    pub perfil: Option<String>,
}

#[derive(Queryable, Serialize, Selectable, Identifiable)]
#[diesel(table_name = libro)]
pub struct Libro {     // Esta Structura como get (json) o select * from libro (postgres)
    pub id: i32,
    pub titulo: String,
    pub slug: String,
    pub sinopsis: Option<String>,
    pub tipo_id: i32,
    pub publicacion: chrono::NaiveDate, // NaiveDateTime por sql...
    pub estado_id: i32,
    pub visibilidad: Option<bool>,
    pub updated_at: NaiveDateTime,
    pub created_at: NaiveDateTime
}

use chrono::NaiveDate;

#[derive(Insertable, Deserialize, Serialize, Clone)]    // Insertable = Jamás usar id
#[diesel(table_name = libro)]
pub struct NuevoLibro {     // Struct para insertar datos en la base de datos (INSERT)
    pub titulo: String,
    pub slug: String,
    pub sinopsis: Option<String>,
    pub tipo_id: i32,
    pub publicacion: NaiveDate,     // NaiveDate por que en la base de datos es type Date (solo fecha)
    pub estado_id: i32,
    pub visibilidad: Option<bool>,
    pub updated_at: NaiveDateTime,
    pub created_at: NaiveDateTime,
}

// CREATE TABLE imagen_libro (
//     id SERIAL PRIMARY KEY,
//     libro_id INT NOT NULL,
//     url_image TEXT NOT NULL,  -- url o link de la imagen, cloudflare r2.
//     tipo VARCHAR(20) NOT NULL, -- será check luego: "portada", "BANNER", etc
//     nombre VARCHAR (255) NOT NULL, -- necesario para frontend, alt = nombre.
//     is_active BOOLEAN NOT NULL, -- false para imágen de portada antiguas y true for main
//     mime_type VARCHAR (255) NOT NULL, -- image/jpeg, image/png, image/webp, etc.
//     tamano_bytes BIGINT NOT NULL, -- tamaño en bytes de la imagen.
//     ancho INT, -- pixeles
//     alto INT, -- PIXELES
//     created_at TIMESTAMP NOT NULL,
//     updated_at TIMESTAMP NOT NULL,
//     CONSTRAINT fk_imagen_libro FOREIGN KEY (libro_id) REFERENCES libro(id) ON DELETE CASCADE
//   );
use crate::schema::imagen_libro;
#[derive(Queryable, Serialize, Selectable, Identifiable)]
#[diesel(table_name = imagen_libro)]
pub struct ImagenLibro{
    pub id: i32,
    pub libro_id: i32,
    pub url_image: String,
    pub tipo: String,
    pub nombre: String,
    pub is_active: bool,
    pub mime_type: String,
    pub tamano_bytes: i64,
    pub ancho: Option<i32>,
    pub alto: Option<i32>,
    pub updated_at: NaiveDateTime
}

// --------------------------------------- Libro x Usuario -------------------------------------------
// #[derive(Queryable, Serialize)]
// #[diesel(table_name = libro_usuario)]
// pub struct LibroUsuario {     // Esta Structura como get (json) o select * from libro (postgres)
//     pub id: i32, // -> Int4
//     pub fk_usuario: i32, // -> Nullable<Int4>
//     pub fk_libro: i32, // -> Nullable<Int4>
//     pub estado: Option<String>, // -> Nullable<Varchar> maxlength 50
//     // estado ejemplos: Pendiente, completado, visto..., etc
//     pub creado: NaiveDateTime, // -> Timestamp
// }

use crate::schema::usuario_libro;   // siempre llamar crate::schema para "Insertable"
#[derive(Insertable, Deserialize, Serialize)]   // Clone
#[diesel(table_name = usuario_libro)]
pub struct NuevoLibroUsuario { // tabla relacional
    pub usuario_id: i32,
    pub libro_id: i32,
    pub estado: Option<String>,
    pub favorito: bool,
    pub creado: NaiveDateTime
}

// #[derive(Queryable, Selectable, Serialize, Debug)]
// #[diesel(check_for_backend(diesel::pg::Pg))]
// pub struct AllLibroxUsuario {
//     #[diesel(select_expression = libro_usuario::id)]
//     pub relacion_id: i32,

//     #[diesel(select_expression = libro_usuario::fk_usuario)]
//     pub usuario_id: Option<i32>,

//     #[diesel(select_expression = libro_usuario::fk_libro)]
//     pub libro_id: Option<i32>,

//     #[diesel(select_expression = libro::titulo)]
//     pub titulo: String,

//     #[diesel(select_expression = libro::perfil)]
//     pub perfil: Option<String>,

//     #[diesel(select_expression = libro::sinopsis)]
//     pub sinopsis: Option<String>,

//     #[diesel(select_expression = libro::tipo)]
//     pub tipo: Option<String>,

//     #[diesel(select_expression = libro::visibilidad)]
//     pub visibilidad: Option<bool>,
// }
use diesel::Associations;
#[derive(Queryable, Identifiable, Associations)] // ,Selectable, Queryable, Associations, Debug, 
#[diesel(belongs_to(Libro, foreign_key = libro_id))]
#[diesel(belongs_to(NiceAccount, foreign_key = usuario_id))]
#[diesel(table_name = usuario_libro)]
pub struct RelationLibroUsuario{
    pub id: i32, // -> Int4
    pub usuario_id: i32, // -> Nullable<Int4>
    pub libro_id: i32, // -> Nullable<Int4>
    pub estado: Option<String>, // -> Nullable<Varchar> maxlength 50
    pub favorito: bool,
    pub creado: NaiveDateTime, // -> Timestamp
}



#[derive(Queryable, Serialize, Debug)]
#[diesel(table_name = usuario_libro)]
pub struct AllLibroxUsuario {
    pub relacion_id: i32,           // id en libro_usuario
    pub usuario_id: Option<i32>,    // fk_usuario   
    pub libro_id: Option<i32>,      // fk_libro
    pub titulo: Option<String>, // tabla=libro
    pub perfil: Option<String>, // tabla=libro
    pub sinopsis: Option<String>, // tabla=libro
    pub tipo: Option<String>, // tabla=libro
    pub visibilidad: Option<bool>, // tabla=libro
}

// -------------------------------------------- Género ------------------------------------------------
use crate::schema::genero;
#[derive(Queryable, Serialize)]
#[diesel(table_name = genero)]
pub struct Genero {
    pub id: i32,
    pub nombre: String,
    pub descripcion: Option<String>,
}
#[derive(Insertable, Deserialize, Serialize)]
#[diesel(table_name = genero)]
pub struct NuevoGenero {
    pub nombre: String,
    pub descripcion: Option<String>,
}
// ----------------------------------------------------------------------
use crate::schema::libro_genero;

#[derive(Insertable, Queryable, Serialize, Deserialize, Selectable)]
#[diesel(table_name = libro_genero)]
pub struct NuevoLibroGenero {
    pub libro_id: i32,
    pub genero_id: i32,
}

use diesel::Identifiable;
#[derive(Queryable, Identifiable, Associations)]
#[diesel(belongs_to(Libro))]
#[diesel(belongs_to(Genero))]
#[diesel(table_name = libro_genero)]
pub struct LibroGenero {
    pub id: i32,
    pub libro_id: i32,
    pub genero_id: i32,
}
// ---------------------------------------------













// todo --- > abajo para el manejo de imágenes R2
// pub mod image {
//     use chrono::NaiveDateTime;
//     use diesel::{Insertable, Queryable, Selectable};
//     use serde::{Deserialize, Serialize};
//     use std::fmt;

//     use crate::schema::images;

//     #[derive(Debug, Clone, Serialize, Deserialize)]
//     pub struct R2Config {
//         pub access_key_id: String,
//         pub secret_access_key: String,
//         pub endpoint: String,
//         pub bucket_name: String,
//     }

//     #[derive(Debug, Clone, Serialize, Deserialize)]
//     pub enum ImageCategory {
//         ProfilePicture,
//         BookCover,
//         ChapterImage,
//         ScanLogo,
//     }

//     #[derive(Debug, Clone, Serialize, Deserialize)]
//     pub enum HttpMethod {
//         Get,
//         Put,
//         Delete,
//     }

//     #[derive(Debug, Clone, Serialize, Deserialize)]
//     pub struct R2PresignedUrlRequest {
//         pub method: HttpMethod,
//         pub key: String,
//         pub expires_in_seconds: u64,
//         pub content_type: Option<String>,
//     }

//     #[derive(Debug, Clone, Serialize, Deserialize)]
//     pub struct ImageUploadRequest {
//         pub file_name: String,
//         pub file_size: u64,
//         pub content_type: String,
//         pub category: ImageCategory,
//     }

//     #[derive(Debug, Clone, Serialize, Deserialize)]
//     pub struct ImageUploadResponse {
//         pub upload_url: String,
//         pub image_id: String,
//         pub expires_at: NaiveDateTime,
//     }

//     #[derive(Debug, Clone, Serialize, Deserialize)]
//     pub struct ImageAccessRequest {
//         pub image_id: String,
//     }

//     #[derive(Debug, Clone, Serialize, Deserialize)]
//     pub struct ImageAccessResponse {
//         pub access_url: String,
//         pub expires_at: NaiveDateTime,
//     }

//     #[derive(Queryable, Selectable, Debug, Clone, Serialize, Deserialize)]
//     #[diesel(table_name = images)]
//     pub struct ImageMetadata {
//         pub image_id: String,
//         pub owner_id: i32,
//         pub file_name: String,
//         pub file_size: i64,
//         pub content_type: String,
//         pub category: String,
//         pub r2_key: String,
//         pub is_public: bool,
//         pub created_at: NaiveDateTime,
//         pub updated_at: NaiveDateTime,
//     }

//     #[derive(Insertable, Debug, Clone, Serialize, Deserialize)]
//     #[diesel(table_name = images)]
//     pub struct NewImageMetadata {
//         pub image_id: String,
//         pub owner_id: i32,
//         pub file_name: String,
//         pub file_size: i64,
//         pub content_type: String,
//         pub category: String,
//         pub r2_key: String,
//         pub is_public: bool,
//         pub created_at: NaiveDateTime,
//         pub updated_at: NaiveDateTime,
//     }

//     #[derive(Debug)]
//     pub enum ImageError {
//         InvalidFormat,
//         TooLarge,
//         Unauthorized,
//         NotFound,
//         DatabaseError(String),
//         R2Error(String),
//     }

//     impl fmt::Display for ImageError {
//         fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//             match self {
//                 ImageError::InvalidFormat => write!(f, "Formato de imagen inválido"),
//                 ImageError::TooLarge => write!(f, "Imagen demasiado grande"),
//                 ImageError::Unauthorized => write!(f, "No autorizado"),
//                 ImageError::NotFound => write!(f, "Imagen no encontrada"),
//                 ImageError::DatabaseError(e) => write!(f, "Error de base de datos: {}", e),
//                 ImageError::R2Error(e) => write!(f, "Error de R2: {}", e),
//             }
//         }
//     }

//     impl std::error::Error for ImageError {}
// }

