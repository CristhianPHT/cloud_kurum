// Módulos públicos para usarlo con name de package
pub mod models;
pub mod modules;
pub mod schema;
pub mod web {
    pub mod basic;
    pub mod interface;
}

// Re-exportar las funciones más utilizadas para mantener compatibilidad
pub use modules::database::establish_connection;

// Re-exportar funciones por categoría
pub mod usuarios {
    pub use crate::modules::usuarios::*;
}

pub mod account {
    pub use crate::modules::account::*;
}

pub mod auth {
    pub use crate::modules::auth::*;
}

pub mod libros {
    pub use crate::modules::libros::*;
}

pub mod generos {
    pub use crate::modules::generos::*;
}

pub mod relaciones {
    pub use crate::modules::relaciones::*;
}

pub mod database {
    pub use crate::modules::database::*;
}

// Re-exportar funciones principales para mantener compatibilidad con código existente
pub use modules::account::{
    insert_usuario, login_usuario_hashed, login_usuario_hashed_old, select_id_usuario,
    update_login, username_existe,
};
pub use modules::auth::{calculate_expiration, generate_jwt, insert_auth_token, select_id_token};
pub use modules::database::{generic_insert, select_by_id};
pub use modules::generos::{insert_gen_new, select_gen_all, select_gen_unico};
pub use modules::libros::{insert_libro_nuevo, select_libro_main, select_nombre_libros};
pub use modules::relaciones::{buscar_libros_por_genero, insert_libro_genero, OrdenamientoLibro};
pub use modules::usuarios::{insert_user, select_all_users, select_id, update_user_id};
