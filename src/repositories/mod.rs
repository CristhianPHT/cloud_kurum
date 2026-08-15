// Repositori: Acceso a base de datos.
pub mod account;
pub mod auth;
pub mod database;
pub mod generos;
pub mod libros;
pub mod relaciones;
pub mod repository;
pub mod libro_estados;
pub mod libro_usuario;

pub use account::*;
pub use auth::*;
pub use database::*;
pub use generos::*;
pub use libros::*;
pub use relaciones::*;
pub use repository::*;  // aquí repository podría ser eliminado y se usaria el mod.rs de la carpeta repositories
