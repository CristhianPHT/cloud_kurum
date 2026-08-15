use serde::{Deserialize, Serialize};
use diesel::{Insertable, Queryable, AsChangeset };
use crate::schema::{libro_estado, libro_tipo};

#[derive(Queryable, Serialize, AsChangeset, Deserialize)]
#[diesel(table_name = libro_estado)]
pub struct LibroEstado {
    pub id: i32,
    pub nombre: String,
}

#[derive(Insertable, Deserialize)]
#[diesel(table_name = libro_estado )]
pub struct NewLibroEstado {
    pub nombre: String,
}

#[derive(Queryable, Serialize, AsChangeset, Deserialize)]
#[diesel(table_name = libro_tipo)]
pub struct LibroTipo {
    pub id: i32,
    pub nombre: String,
}

#[derive(Insertable, Deserialize)]
#[diesel(table_name = libro_tipo )]
pub struct NewLibroTipo {
    pub nombre: String,
}