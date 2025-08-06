use crate::models::{Claims, NuevoAuthToken};
use crate::schema::auth_tokens::dsl::{auth_tokens, token, user_id};
use chrono::{DateTime, Duration, Utc};
use diesel::dsl::insert_into;
use diesel::prelude::*;
use jsonwebtoken::{encode, EncodingKey, Header};

pub fn calculate_expiration() -> DateTime<Utc> {
    let expiration_datetime = Utc::now() + Duration::hours(8);
    expiration_datetime
}

// Función para generar el JWT
pub fn generate_jwt(
    user_id_input: i32,
    expira: DateTime<Utc>,
) -> Result<String, jsonwebtoken::errors::Error> {
    let my_claims = Claims {
        sub: user_id_input,
        exp: expira.timestamp() as usize,
        iss: "kurumitokisaki.fun".to_string(),
    };

    let secret = "KurumiTokisaki453"; // Debes usar una clave más segura en producción
    let encoding_key = EncodingKey::from_secret(secret.as_ref());

    encode(&Header::default(), &my_claims, &encoding_key)
}

pub fn insert_auth_token(
    conn: &mut PgConnection,
    user_id_input: i32,
    token_input: &str,
    expira_input: DateTime<Utc>,
) -> QueryResult<String> {
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
        .get_result(conn);
    inserted_id
}

pub fn select_id_token(conn: &mut PgConnection, token_input: String) -> QueryResult<i32> {
    // Buscar el ID del usuario a través del token jwt
    auth_tokens
        .filter(token.eq(token_input))
        .select(user_id)
        .first::<i32>(conn)
}
