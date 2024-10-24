use serde::{Deserialize, Serialize};
// use serde::Serialize;
use diesel::{Queryable,Insertable};
// use diesel::prelude::*; // eliminar ???
use crate::schema::usuariosss;

#[derive(Queryable, Serialize, Debug)]
pub struct Usuario {
    pub id: i32,
    pub nombre: String,
    pub apellido: String,
}

#[derive(Insertable, Deserialize, Serialize)]  // Agregamos Deserialize para recibir objetos JSON
#[diesel(table_name = usuariosss)]
pub struct NuevoUsuario {
    pub nombre: String,
    pub apellido: String,
}

#[derive(Deserialize)]
pub struct UsuarioUpdate {
    pub nombre: Option<String>,
    pub apellido: Option<String>,
}

// // up.sql
// CREATE TABLE posts (
//     id SERIAL PRIMARY KEY,
//     title VARCHAR NOT NULL,
//     body TEXT NOT NULL,
//     published BOOLEAN NOT NULL DEFAULT FALSE
//   )
// // down.sql
// DROP TABLE posts
// // terminal: diesel migration run
// // terminal: diesel migration redo