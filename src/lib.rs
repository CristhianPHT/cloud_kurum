// Módulos públicos para usarlo con name de package
pub mod models;
pub mod infrastructure;
pub mod repositories;
pub mod services;
pub mod schema;
pub mod utils;
pub mod handlers;
pub mod web {
    pub mod basic;
    pub mod interface;
    pub mod auth;
    pub mod handlers;
    pub mod dto;
}

// Re-exportar las funciones más utilizadas para mantener compatibilidad
// pub use modules::database::establish_connection;
pub use infrastructure::db::establish_connection;

// Re-exportar funciones por categoría
pub mod usuarios {
    pub use crate::repositories::usuarios::*;
}

pub mod account {
    pub use crate::services::account::{register_user, login_usuario_hashed, update_login};
    pub use crate::repositories::account::{
        login_usuario_hashed_old, select_id_usuario, select_usuario_por_username, username_existe, select_header_user,
    };
}

pub mod auth {
    pub use crate::services::auth::{calculate_expiration, generate_jwt};
    pub use crate::repositories::auth::{insert_auth_token, select_id_token};
}

pub mod libros {
    pub use crate::repositories::libros::{insert_libro_nuevo, select_libro_main, select_nombre_libros};
}
pub mod libro_usuario {
    pub use crate::repositories::libro_usuario::select_libros_por_usuario;
}
pub mod generos {
    pub use crate::repositories::generos::*;
}

pub mod relaciones {
    pub use crate::repositories::relaciones::*;
}

// Re-exportar funciones principales para mantener compatibilidad con código existente
pub use services::account::{register_user, login_usuario_hashed, update_login};
pub use repositories::account::{
    login_usuario_hashed_old, select_id_usuario, select_usuario_por_username, username_existe, select_header_user,
};
pub use repositories::auth::{insert_auth_token, select_id_token};
pub use services::auth::{calculate_expiration, generate_jwt};
pub use repositories::database::{generic_insert, select_by_id};
pub use repositories::generos::{insert_gen_new, select_gen_all, select_gen_unico};
pub use repositories::libros::{insert_libro_nuevo, select_libro_main, select_nombre_libros};
pub use repositories::relaciones::{buscar_libros_por_genero, insert_libro_genero, OrdenamientoLibro};
pub use repositories::usuarios::{insert_user, select_all_users, select_id, update_user_id};
