
#[derive(Queryable, Selectable, Associations, Debug, Serialize)]
#[diesel(belongs_to(NuevoAccount, foreign_key = usuario_id))]
#[diesel(table_name = libro_usuario)]
#[diesel(primary_key(id))]
pub struct AllLibroxUsuario {
    pub relacion_id: i32,
    pub usuario_id: Option<i32>,
    pub libro_id: Option<i32>,
    pub titulo: Option<String>,
    pub perfil: Option<String>,
    pub sinopsis: Option<String>,
    pub tipo: Option<String>,
    pub visibilidad: Option<bool>,
}