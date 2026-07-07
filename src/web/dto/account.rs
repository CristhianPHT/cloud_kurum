use serde::{Serialize, Deserialize};
use diesel::{Queryable};

#[derive(Queryable, Serialize, Debug)]      // para mostrar (dashboard)
pub struct Account {    // Obtener datos (get) (Pagina principal del usuario para ver sus datos... dashboard)
    pub nickname: String,   // apodo
    // pub username: String,    // gmail, o con lo que ingresará por arriba ---> LoginAccount ...
    pub url_image: Option<String>     // imagen (portada/icon/foto de perfil)
}

#[derive(Deserialize, Debug)]
pub struct RegisterAccount{
    pub username: String,
    pub password: String,
    pub nickname: Option<String>,
    pub email: String
}

#[derive(Deserialize)]
pub struct ChangePassword {
    pub old_password: String,
    pub new_password: String,
}

#[derive(Deserialize)]
pub struct UpdateNickname {
    pub nickname: String,
}
