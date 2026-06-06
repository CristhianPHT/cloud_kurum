use crate::models::Claims;
use chrono::{DateTime, Duration, Utc};
use jsonwebtoken::{encode, EncodingKey, Header};

pub fn calculate_expiration() -> DateTime<Utc> {
    Utc::now() + Duration::hours(8)
}

pub fn generate_jwt(user_id_input: i32, expira: DateTime<Utc>) -> Result<String, jsonwebtoken::errors::Error> {
    let my_claims = Claims {
        sub: user_id_input,
        exp: expira.timestamp() as usize,
        iss: "kurumitokisaki.fun".to_string(),
    };

    let secret = "KurumiTokisaki453";   // mejorar mucho clave y expect, panic!
    let encoding_key = EncodingKey::from_secret(secret.as_ref());
    encode(&Header::default(), &my_claims, &encoding_key)
}
