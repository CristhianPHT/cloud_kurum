use chrono::NaiveDate;
use serde::{Deserialize, Serialize};
use diesel::{Queryable};

#[derive(Deserialize, Serialize)]    // Insertable = Jamás usar id
// #[diesel(table_name = libro)]
pub struct NewBook {     // Struct para insertar datos en la base de datos (INSERT)
    pub titulo: String,
    pub slug: String,
    pub sinopsis: Option<String>,
    pub tipo_id: i32,
    pub publicacion: NaiveDate,     // NaiveDate por que en la base de datos es type Date (solo fecha)
    pub estado_id: i32,
    pub visibilidad: Option<bool>,
}

#[derive(Queryable, Serialize)]
pub struct LibroDetalle {     // Será DTO para select
    pub id: i32,
    pub titulo: String,
    pub slug: String,
    pub sinopsis: Option<String>,
    pub tipo_id: i32,
    pub tipo_nombre: String,
    pub estado_id: i32,
    pub estado_nombre: String,
    pub publicacion: chrono::NaiveDate, // NaiveDateTime por sql...
    pub visibilidad: Option<bool>,
}