// Service: Lógica de negocio.
pub mod service;
pub mod account;
pub mod auth;
pub mod book;

// Re-export services for easy access
pub use account::*;
pub use auth::*;
pub use book::*;

