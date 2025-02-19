use diesel::prelude::*;
use diesel::query_dsl::methods::{FindDsl, OffsetDsl, LimitDsl};
use dotenv::dotenv;
use std::env;
use diesel::pg::Pg;
// modulos públicos para usarlo con name de package. //(nube_kurum)
pub mod models;
pub mod schema;
pub mod web {
  pub mod basic;
  pub mod interface;
}


// Establece la conexión a la base de datos
pub fn establish_connection() -> PgConnection {  // para conectar a la base de datos
  dotenv().ok();
  let database_url = env::var("DATABASE_URL")
    .expect("Se debe configurar DATABASE_URL");
  PgConnection::establish(&database_url)
    .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}

use models::{NuevoUsuario, Usuario, UsuarioUpdate}; 
use schema::usuariosss::dsl::{usuariosss, id, nombre, apellido}; // para id y usuariosss
// use nube_kurum::establish_connection;  // ya no llamarlo si se usa en otro lado
pub fn select_id(conn: &mut PgConnection, usuario_id: i32) -> Usuario {
  FindDsl::
    find(usuariosss, usuario_id)
    .first::<Usuario>(conn)
    .expect("Error al buscar el usuario")
}

pub fn select_all_users(conn: &mut PgConnection, page: i64) -> Vec<Usuario> {
  let offset_val = 5 * page;
  OffsetDsl::offset(
      LimitDsl::limit(
          diesel::QueryDsl::order_by(usuariosss, id),
          5
      ),
      offset_val
  )
  .load::<Usuario>(conn)
  .expect("No cargó Usuarios: Error")
}

use diesel::dsl::{update, insert_into};


// pub fn update_user_id(conn: &mut PgConnection, usuario_id: i32, usuario: UsuarioUpdate) -> QueryResult<usize> {
//   let actualizar = update(usuariosss.filter(id.eq(usuario_id)));
//   if let (Some(nuevo_nombre), Some(nuevo_apellido)) = (&usuario.nombre, &usuario.apellido) {
//       actualizar.set((
//           nombre.eq(nuevo_nombre.clone()),
//           apellido.eq(nuevo_apellido.clone())
//       )).execute(conn)
//   } else if let Some(nuevo_nombre) = &usuario.nombre {
//       actualizar.set(nombre.eq(nuevo_nombre.clone())).execute(conn)
//   } else if let Some(nuevo_apellido) = &usuario.apellido {
//       actualizar.set(apellido.eq(nuevo_apellido.clone())).execute(conn)
//   } else {
//       println!("No se puede actualizar: ningún campo para modificar");
//       Ok(0)
//   }
// }


pub fn update_user_id(conn: &mut PgConnection, usuario_id: i32, usuario_local: UsuarioUpdate) -> QueryResult<usize> {
  // Iniciamos la consulta base
  let query = update(usuariosss.filter(id.eq(usuario_id)));

  match (&usuario_local.nombre, &usuario_local.apellido) {
    (Some(nuevo_nombre), Some(nuevo_apellido)) => {
      query.set((
        nombre.eq(nuevo_nombre.clone()),
        apellido.eq(nuevo_apellido.clone())
      )).execute(conn)
    },
    (Some(nuevo_nombre), None) => {
      query.set(nombre.eq(nuevo_nombre.clone())).execute(conn)
    },
    (None, Some(nuevo_apellido)) => {
      query.set(apellido.eq(nuevo_apellido.clone())).execute(conn)
    },
    (None, None) => {
      println!("No se puede actualizar: ningún campo para modificar");
      Ok(0)
    }
  }
}

pub fn insert_user(connec: &mut PgConnection, nuevo: NuevoUsuario) -> QueryResult<i32> {
  let inserted_id = insert_into(usuariosss)
      .values(nuevo)
      .returning(id)
      .get_result(connec);
  // println!("{:?}", inserted_id);

  inserted_id
}


// ..................................................................................................
  use models::{NuevoAccount, Account};
use schema::usuario::dsl::{usuario, id as account_id, username, password_hash};

pub fn insert_usuario(conn: &mut PgConnection, nuevo: NuevoAccount) -> QueryResult<i32> {
  let inserted_id = insert_into(usuario)
    .values(nuevo)
    .returning(account_id)
    .get_result(conn);
  inserted_id
}

pub fn select_id_usuario(conne: &mut PgConnection, usuario_id: i32) -> Account {
  let cuentas = diesel::query_dsl::methods::FindDsl::
      find(usuario, usuario_id)
      .select(Account::as_select())
      .first::<Account>(conne) // Especificamos el tipo aquí
      .expect("Error al buscar el usuario");
  cuentas
}


pub fn login_usuario_hashed(conn: &mut PgConnection, user_email: &str, hashed_password: &str) -> QueryResult<i32> {
  let cuentas = usuario
    .filter(username.eq(user_email))
    .filter(password_hash.eq(hashed_password))
    .select(account_id)
    .first::<i32>(conn);
  match cuentas {
    Ok(cuenta_id) => Ok(cuenta_id),
    Err(diesel::result::Error::NotFound) => Ok(0), // Devuelve None si no se encuentra el usuario
    Err(e) => Err(e), // Propaga otros errores
  }
}
pub fn update_login(conn: &mut PgConnection, usuario_id: i32, nuevo: NuevoAccount) -> QueryResult<usize> {
  let query = update(usuario.filter(account_id.eq(usuario_id)));
  query.set(nuevo).execute(conn)
}

// ..................................................................................................
// #[allow(unused_imports)]
use models::{NuevoAuthToken, Claims};
use schema::auth_tokens::dsl::{auth_tokens, token, user_id};
use jsonwebtoken::{encode, Header, EncodingKey};
use chrono::{Duration, DateTime, Utc};
fn calculate_expiration() -> DateTime<Utc> {
  let expiration_datetime = Utc::now() + Duration::hours(8);
  expiration_datetime
}
// Función para generar el JWT
pub fn generate_jwt(user_id_input: i32, expira: DateTime<Utc>) -> String {
  let my_claims = Claims {
      sub: user_id_input,
      exp: expira.timestamp() as usize,  // Expira en 8 horas
      iss: "kurumitokisaki.fun".to_string(),
  };

  let secret = "KurumiTokisaki453";  // Debes usar una clave más segura en producción
  let encoding_key = EncodingKey::from_secret(secret.as_ref());

  encode(&Header::default(), &my_claims, &encoding_key).unwrap()
}

pub fn insert_auth_token(conn: &mut PgConnection, user_id_input: i32, token_input: String, expira_input: DateTime<Utc>) -> QueryResult<String> {
  let auth_token = NuevoAuthToken {
    user_id: user_id_input,
    token: token_input,
    dispositivo: None,
    expira: expira_input.naive_utc(),
    activo: true,
  };

  let inserted_id = insert_into(auth_tokens)
    .values(auth_token)
    .returning(token)
    .get_result(conn);
  inserted_id
}

pub fn select_id_token(conn: &mut PgConnection, token_input: String) -> QueryResult<i32> {
  let tokenizador = auth_tokens
    .filter(token.eq(token_input))
    .select(user_id)
    .first::<i32>(conn);
  tokenizador
}

// Select para todo... por defecto con objetos parametrizados.

// pub fn select_params(conn: &mut PgConnection, id_clave: i32) -> T {  // para mostrar usuario por id = input(conn, id)
//   let tabla_diesel = name_tabla
//     .find(id_clave)
//     .first::<T>(conn)
//     // .optional() // Devuelve un Option<Usuario>
//     .expect("Error al buscar la data");
//   tabla_diesel
// }

use diesel::query_dsl::{LoadQuery, RunQueryDsl};
use diesel::Table;

pub fn select_by_id<T, U>(
    table: T,
    conn: &mut PgConnection,
    id_clave: i32,
) -> Result<U, diesel::result::Error>
where
    // T debe ser una tabla con FindDsl para i32
    T: Table + FindDsl<i32>,
    // La consulta resultante de `find()` debe admitir LIMIT 1
    <T as FindDsl<i32>>::Output: LimitDsl + RunQueryDsl<PgConnection>,
    // La consulta final (después de LIMIT) debe poder cargarse en U
    <<T as FindDsl<i32>>::Output as LimitDsl>::Output: LoadQuery<'static, PgConnection, U>,
    // U debe implementar Queryable para las columnas de la tabla T
    U: Queryable<<T as Table>::AllColumns, Pg>,
    // Las columnas de T deben ser compatibles con el backend PostgreSQL
    <T as Table>::AllColumns: diesel::Expression<SqlType = diesel::sql_types::Untyped>,
{
    table.find(id_clave).first(conn)
}


use diesel::insertable::CanInsertInSingleQuery; //use diesel::Table;
use diesel::query_builder::{InsertStatement, QueryFragment, QueryId};
use diesel::query_dsl::methods::ExecuteDsl;

pub fn generic_insert<T, U>(
    table: T,
    conn: &mut PgConnection,
    data: U,
) -> Result<usize, diesel::result::Error>
where
    T: Table + QuerySource + QueryId,
    U: Insertable<T>,
    InsertStatement<T, U::Values>: ExecuteDsl<PgConnection>,
    U::Values: QueryFragment<Pg> + QueryId + CanInsertInSingleQuery<Pg>,
{
    diesel::insert_into(table).values(data).execute(conn)
}





// pub fn select_id(conn: &mut PgConnection, usuario_id: i32) -> Usuario {  // para mostrar usuario por id = input(conn, id)
//   let variable_usuario = usuariosss
//     .find(usuario_id)
//     .first::<Usuario>(conn)
//     // .optional() // Devuelve un Option<Usuario>
//     .expect("Error al buscar el usuario");
//   variable_usuario
// }