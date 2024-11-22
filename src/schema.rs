// @generated automatically by Diesel CLI.

diesel::table! {
    auth_tokens (id) {
        id -> Int4,
        user_id -> Int4,
        token -> Varchar,
        dispositivo -> Nullable<Varchar>,
        expira -> Timestamp,
        activo -> Bool,
    }
}

diesel::table! {
    multidispositivos (id) {
        id -> Int4,
        user_id -> Int4,
        dispositivo_nombre -> Varchar,
        device_tipo -> Nullable<Varchar>,
        confianza -> Bool,
    }
}

diesel::table! {
    sessions (id) {
        id -> Int4,
        user_id -> Int4,
        dispositivo -> Nullable<Varchar>,
        direccion_ip -> Nullable<Varchar>,
        inicio -> Timestamp,
        actualizacion -> Timestamp,
    }
}

diesel::table! {
    token_recuperacion (id) {
        id -> Int4,
        user_id -> Int4,
        token -> Varchar,
        tipo -> Nullable<Bool>,
        expira -> Timestamp,
    }
}

diesel::table! {
    usuarios (id) {
        id -> Int4,
        nickname -> Nullable<Varchar>,
        username -> Varchar,
        password_hash -> Varchar,
        email -> Varchar,
        actualizacion -> Timestamp,
        activo -> Bool,
        creado -> Timestamp,
    }
}

diesel::table! {
    usuariosss (id) {
        id -> Int4,
        nombre -> Varchar,
        apellido -> Varchar,
    }
}

diesel::joinable!(auth_tokens -> usuarios (user_id));
diesel::joinable!(multidispositivos -> usuarios (user_id));
diesel::joinable!(sessions -> usuarios (user_id));
diesel::joinable!(token_recuperacion -> usuarios (user_id));

diesel::allow_tables_to_appear_in_same_query!(
    auth_tokens,
    multidispositivos,
    sessions,
    token_recuperacion,
    usuarios,
    usuariosss,
);
