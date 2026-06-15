use crate::models::NuevoAuthToken;
use crate::schema::auth_tokens::dsl::{auth_tokens, token, user_id};
use chrono::{DateTime, Utc};
use diesel::dsl::insert_into;
use diesel::prelude::*;

pub fn insert_auth_token(conn: &mut PgConnection, user_id_input: i32, token_input: &str, expira_input: DateTime<Utc>) -> QueryResult<String> {
    let auth_token = NuevoAuthToken {
        user_id: user_id_input,
        token: token_input.to_string(),
        dispositivo: None,
        expira: expira_input.naive_utc(),
        is_active: true,
    };
    insert_into(auth_tokens)
        .values(auth_token)
        .returning(token)
        .get_result(conn)
}

pub fn select_id_token(conn: &mut PgConnection, token_input: String) -> QueryResult<i32> {
    auth_tokens
        .filter(token.eq(token_input))
        .select(user_id)
        .first::<i32>(conn)
}
