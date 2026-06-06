use serde::{Serialize, Deserialize};
use diesel::{Queryable, Selectable};
use crate::schema::usuario;   // Login (usuario)

#[derive(Queryable, Serialize, Selectable, Debug)]      // para mostrar (dashboard)
#[diesel(table_name = usuario)]
pub struct Account {    // Obtener datos (get) (Pagina principal del usuario para ver sus datos... dashboard)
    pub nickname: Option<String>,   // apodo
    pub perfil: Option<String>,     // imagen (portada/icon/foto de perfil)
    pub username: String,    // gmail, o con lo que ingresará por arriba ---> LoginAccount ...
}

#[derive(Deserialize, Debug)]
pub struct RegisterAccount{
    pub username: String,
    pub password: String,
    pub nickname: Option<String>,
    pub email: String
}