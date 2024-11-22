use serde::{Deserialize, Serialize};
use diesel::{Queryable,Insertable, Selectable, AsChangeset};
#[allow(unused_imports)]
use crate::schema::{usuariosss, usuarios, token_recuperacion, auth_tokens, multidispositivos, sessions};
use chrono::NaiveDateTime;    // O NaiveDateTime si usas timestamps con zona horaria

#[derive(Queryable, Serialize, Debug)]  // Queryable para obtener datos de la base de datos con ID 
pub struct Usuario {  // Struct para obtener datos de la base de datos (SELECT)
    pub id: i32,
    pub nombre: String,
    pub apellido: String,
}

#[derive(Insertable, Deserialize, Serialize, Clone)]  // Agregamos Deserialize, Serialize para recibir y enviar objetos JSON
#[diesel(table_name = usuariosss)]
pub struct NuevoUsuario {  // Struct para insertar datos en la base de datos (INSERT)
    pub nombre: String,
    pub apellido: String,
}

#[derive(Deserialize)]  // Deserialize para recibir objetos JSON
pub struct UsuarioUpdate {  // Struct para actualizar datos de la base de datos (UPDATE)
    pub nombre: Option<String>,
    pub apellido: Option<String>,
}


#[derive(Queryable, Serialize, Debug, Deserialize)]
#[diesel(table_name = usuarios)]
pub struct LoginAccount {
    pub id: Option<i32>,
    pub username: String,
    pub password_hash: String,
}

#[derive(Queryable, Serialize, Selectable, Debug)]
#[diesel(table_name = usuarios)]
pub struct Account {
    pub id: i32,
    pub nickname: Option<String>,
    pub username: String,
    pub password_hash: String,
    pub email: String,
    pub actualizacion: chrono::NaiveDateTime,
    pub activo: bool,
    pub creado: chrono::NaiveDateTime,
}

#[derive(Insertable, Deserialize, Serialize, Clone, AsChangeset)]  // Agregamos Deserialize, Serialize para recibir y enviar objetos JSON
#[diesel(table_name = usuarios)]
pub struct NuevoAccount {  // Struct para insertar datos en la base de datos (INSERT)
    pub nickname: Option<String>,
    pub username: String,
    pub password_hash: String,
    pub email: String,
    pub actualizacion: NaiveDateTime,
}

#[derive(Queryable, Serialize, Debug)]  // Queryable para obtener datos de la base de datos con ID 
pub struct Recuperacion {  // Struct para obtener datos de la base de datos (SELECT)
    pub id: i32,
    pub user_id: i32,
    pub token: String,
    pub tipo: bool,
    pub expira: NaiveDateTime,
    // #[serde(with = "chrono::serde::ts_seconds")]
    // pub expira: DateTime<Utc>,
}

#[derive(Insertable, Deserialize, Serialize, Clone)]  // Agregamos Deserialize, Serialize para recibir y enviar objetos JSON
#[diesel(table_name = token_recuperacion)]
pub struct NuevoRecuperacion {  // Struct para insertar datos en la base de datos (INSERT)
    pub user_id: i32,
    pub token: String,
    pub tipo: bool,
    pub expira: NaiveDateTime,
}

#[derive(Insertable, Deserialize, Serialize, AsChangeset, Clone)]  // Agregamos Deserialize, Serialize para recibir y enviar objetos JSON
#[diesel(table_name = auth_tokens)]
pub struct NuevoAuthToken {  // Struct para insertar datos en la base de datos (INSERT)
    pub user_id: i32,
    pub token: String,
    pub dispositivo: Option<String>,
    pub expira: NaiveDateTime,
    pub activo: bool,
}
#[derive(Queryable, Serialize, Debug)]
#[diesel(table_name = auth_tokens)]
pub struct AuthToken {
    pub id: i32,
    pub user_id: i32,
    pub token: String,
    pub dispositivo: Option<String>,
    pub expira: NaiveDateTime,
    pub activo: bool,
}
#[derive(Serialize, Deserialize)]
pub struct Claims {
    pub sub: i32,      // user_id
    pub exp: usize,    // Expiration time
    pub iss: String,   // Issuer (tu dominio)
}
  
  
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