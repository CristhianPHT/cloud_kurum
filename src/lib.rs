
use diesel::prelude::*;
use dotenv::dotenv;
use std::env;
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

pub fn select_id(conn: &mut PgConnection, usuario_id: i32) -> Usuario {  // para mostrar usuario por id = input(conn, id)
  let usuario = usuariosss
    .find(usuario_id)
    .first::<Usuario>(conn)
    // .optional() // Devuelve un Option<Usuario>
    .expect("Error al buscar el usuario");
  usuario
}
pub fn select_all_users(conn: &mut PgConnection, page: i64) -> Vec<Usuario> {  // para mostrar todos los usuarios = input(conn, page)
  let offset = 5 * page;
  let select_users: Vec<Usuario> = usuariosss
    .order_by(id) // ORDER BY
    .limit(5) //
    .offset(offset) //
    .load::<Usuario>( conn) //
    .expect("No cargó Usuarios: Error");
  select_users
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


pub fn update_user_id(conn: &mut PgConnection, usuario_id: i32, usuario: UsuarioUpdate) -> QueryResult<usize> {
  // Iniciamos la consulta base
  let query = update(usuariosss.filter(id.eq(usuario_id)));

  match (&usuario.nombre, &usuario.apellido) {
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

pub fn insert_user(connec: &mut PgConnection, usuario: NuevoUsuario) -> QueryResult<i32> {
  let inserted_id = insert_into(usuariosss)
      .values(usuario)
      .returning(id)
      .get_result(connec);
  // println!("{:?}", inserted_id);

  inserted_id
}

// ..................................................................................................
use models::{NuevoAccount, Account};
use schema::usuarios::dsl::{usuarios, id as account_id, username, password_hash};

pub fn insert_usuario(conn: &mut PgConnection, usuario: NuevoAccount) -> QueryResult<i32> {
  let inserted_id = insert_into(usuarios)
    .values(usuario)
    .returning(account_id)
    .get_result(conn);
  inserted_id
}
pub fn select_id_usuario(conne: &mut PgConnection, usuario_id: i32) -> Account {
  let cuentas = usuarios
      .find(usuario_id)
      .select(Account::as_select())
      .first(conne) // Usar `load` para obtener un vector de resultados
      .expect("Error al buscar el usuario");
  cuentas
}
pub fn login_usuario_hashed(conn: &mut PgConnection, user_email: &str, hashed_password: &str) -> QueryResult<i32> {
  let cuentas = usuarios
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
pub fn update_login(conn: &mut PgConnection, usuario_id: i32, usuario: NuevoAccount) -> QueryResult<usize> {
  let query = update(usuarios.filter(account_id.eq(usuario_id)));
  query.set(usuario).execute(conn)
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