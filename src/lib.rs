// use actix_web::web::Query;
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
pub fn select_id(conn: &mut PgConnection, usuario_id: i32) -> Result<Usuario, diesel::result::Error> {
    use schema::usuariosss::dsl::*; // Asegúrate de importar las columnas necesarias

    usuariosss
        .filter(id.eq(usuario_id)) // Usa filter para especificar el id
        .first::<Usuario>(conn)
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


// ................................................................................................................................................
use models::{NuevoAccount, Account};
use schema::usuario::dsl::{usuario, id as account_id, username, password_hash};
use bcrypt::{hash, DEFAULT_COST, verify}; // Importamos bcrypt

pub fn insert_usuario(conn: &mut PgConnection, mut nuevo: NuevoAccount) -> QueryResult<i32> {
  // Hashear la contraseña antes de insertar
  let hashed_password = hash(&nuevo.password_hash, DEFAULT_COST)
    .map_err(|e| diesel::result::Error::DeserializationError(Box::new(e)))?;
  
  nuevo.password_hash = hashed_password; // Actualizar el campo password_hash con el hash generado
  let inserted_id = insert_into(usuario)  // Insertar el usuario en la base de datos
    .values(nuevo)
    .returning(account_id)
    .get_result(conn);
  inserted_id
}

pub fn select_id_usuario(conn: &mut PgConnection, usuario_id: i32) -> QueryResult<Account> {
  let cuentas = diesel::query_dsl::methods::FindDsl::
    find(usuario, usuario_id)
    .select(Account::as_select())
    .first::<Account>(conn );
  cuentas
}

pub fn login_usuario_hashed(conn: &mut PgConnection, user_email: &str, password_plano: &str) -> QueryResult<i32> {
  let (identificador, contrasenia): (i32, String) = usuario
      .filter(username.eq(user_email))
      .select((account_id, password_hash))
      .first(conn)?;    // posible salida ----> Ok((identificador, contrasenia)) -> QueryResult<(i32, String)>

  // Verificar la contraseña en texto plano contra el hash almacenado
  let is_valid = verify(password_plano, &contrasenia)
      .map_err(|e| diesel::result::Error::DeserializationError(Box::new(e)))?;

  if is_valid {
      Ok(identificador)
  } else {
      Err(diesel::result::Error::NotFound)
  }
}


pub fn update_login(conn: &mut PgConnection, usuario_id: i32, nuevo: NuevoAccount) -> QueryResult<usize> {
  let query = update(usuario.filter(account_id.eq(usuario_id)));
  query.set(nuevo).execute(conn)
}

pub fn username_existe(conn: &mut PgConnection, user_name: &String) -> Result<bool, diesel::result::Error> { // true si existe username en la base de datos
  use diesel::dsl::count_star;    // llamada al conteo de usernames
  let count: i64 = usuario
      .filter(username.eq(user_name))
      .select(count_star())
      .first::<i64>(conn)?;
  Ok(count > 0)
}

pub fn login_usuario_hashed_old(conn: &mut PgConnection, user_email: &str, hashed_password: &str) -> QueryResult<i32> {
  let cuentas = usuario
    .filter(username.eq(user_email))
    .filter(password_hash.eq(hashed_password))
    .select(account_id)
    .first::<i32>(conn);  

  cuentas   // se usa de la siguiente manera: abajo el uso
  // match cuentas {
  //   Ok(cuenta_id) => { ... cuenta_id },
  //   Err(DieselError::NotFound) => { ... }, // Devuelve None si no se encuentra el usuario
  //   Err(e) => Err(e), // Propaga otros errores
  // }
}
// ................................................................................................................................................
// #[allow(unused_imports)]
use models::{NuevoAuthToken, Claims};
use schema::auth_tokens::dsl::{auth_tokens, token, user_id};
use jsonwebtoken::{encode, Header, EncodingKey};
use chrono::{Duration, DateTime, Utc};

fn calculate_expiration() -> DateTime<Utc> {    // aun no se usa... será para la generación del token y validaciones.
  let expiration_datetime = Utc::now() + Duration::hours(8);
  expiration_datetime
}
// Función para generar el JWT
pub fn generate_jwt(user_id_input: i32, expira: DateTime<Utc>) -> Result<String, jsonwebtoken::errors::Error> {
  let my_claims = Claims {
      sub: user_id_input,
      exp: expira.timestamp() as usize,  // Expira en 8 horas
      iss: "kurumitokisaki.fun".to_string(),
  };

  let secret = "KurumiTokisaki453";  // Debes usar una clave más segura en producción
  let encoding_key = EncodingKey::from_secret(secret.as_ref());

  encode(&Header::default(), &my_claims, &encoding_key)
}   // esto es solo para tokens (jwt)

// creo que sería una actualización de token... por que pide token_input...
pub fn insert_auth_token(conn: &mut PgConnection, user_id_input: i32, token_input: &str, expira_input: DateTime<Utc>) -> QueryResult<String> {
  let auth_token = NuevoAuthToken {
    user_id: user_id_input,
    token: token_input.to_string(),
    dispositivo: None,
    expira: expira_input.naive_utc(),
    activo: true,
  };

  let inserted_id = insert_into(auth_tokens)
    .values(auth_token)
    .returning(token)
    .get_result(conn); // Para poder obtener un QueryResult ponemos map_err
  inserted_id
}

pub fn select_id_token(conn: &mut PgConnection, token_input: String) -> QueryResult<i32>{  // Buscar el ID del usuario a traves del token jwt
  auth_tokens
  .filter(token.eq(token_input))
  .select(user_id)
  .first::<i32>(conn)
}

// ..................................................................................................
//  para las funciones genericas de select e insert
use diesel::query_dsl::{LoadQuery, RunQueryDsl};
use diesel::Table;

pub fn select_by_id<T, U>(
  table: T,
  conn: &mut PgConnection,
  id_clave: i32,
) -> Result<U, diesel::result::Error>
where
  T: Table + FindDsl<i32>,
  <T as FindDsl<i32>>::Output: LimitDsl + RunQueryDsl<PgConnection>,
  <<T as FindDsl<i32>>::Output as LimitDsl>::Output: LoadQuery<'static, PgConnection, U>,
  U: Queryable<<T as Table>::AllColumns, Pg>,
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

// ................................................................................................................................................
// ---------------------------------------------------Libros---------------------------------------------------------------
// use diesel::dsl::load;
// use diesel::prelude::RunQueryDsl; load
use crate::models::{Libro, LibroDashboard, NuevoLibro};
use schema::libro::dsl::{libro, titulo, id as libro_id_all, perfil};

pub fn select_nombre_libros(conn: &mut PgConnection) -> Result<Vec<LibroDashboard>, diesel::result::Error> {
  libro
    .select((libro_id_all, titulo, perfil))   //.select(Libro::as_select()) //seŕia otra opcion
    .load::<LibroDashboard>(conn)
}

pub fn insert_libro_nuevo(conn : &mut PgConnection, nuevo_libro : NuevoLibro) -> QueryResult<i32> {
  let salida = insert_into(libro)
    .values(nuevo_libro)
    .returning(libro_id_all)
    .get_result(conn);
  salida
}
pub fn select_libro_main(conn: &mut PgConnection, identificador: i32) -> QueryResult<Libro>{
  let test = libro
    .filter(libro_id_all.eq(identificador))
    .first::<Libro>(conn);
  test
}