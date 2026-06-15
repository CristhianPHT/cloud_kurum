// @generated automatically by Diesel CLI.

diesel::table! {
    auth_tokens (id) {
        id -> Int4,
        user_id -> Int4,
        token -> Text,
        dispositivo -> Nullable<Text>,
        expira -> Timestamp,
        is_active -> Bool,
    }
}

diesel::table! {
    autor (id) {
        id -> Int4,
        #[max_length = 255]
        nombre -> Varchar,
        #[max_length = 255]
        apellido -> Varchar,
    }
}

diesel::table! {
    bloque_capitulo (id) {
        id -> Int4,
        capitulo_id -> Int4,
        orden -> Int4,
        #[max_length = 20]
        tipo -> Varchar,
        recurso_url -> Text,
        #[max_length = 20]
        layout -> Nullable<Varchar>,
        created_at -> Timestamp,
    }
}

diesel::table! {
    capitulo (id) {
        id -> Int4,
        libro_id -> Int4,
        numero -> Int4,
        #[max_length = 255]
        titulo -> Nullable<Varchar>,
        visibilidad -> Bool,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    etiqueta (id) {
        id -> Int4,
        #[max_length = 50]
        nombre -> Varchar,
        imagen -> Nullable<Varchar>,
        #[max_length = 200]
        descripcion -> Nullable<Varchar>,
        visibilidad -> Nullable<Bool>,
        color -> Nullable<Varchar>,
    }
}

diesel::table! {
    genero (id) {
        id -> Int4,
        #[max_length = 50]
        nombre -> Varchar,
        descripcion -> Nullable<Varchar>,
    }
}

diesel::table! {
    imagen_libro (id) {
        id -> Int4,
        libro_id -> Int4,
        url_image -> Text,
        #[max_length = 20]
        tipo -> Varchar,
        #[max_length = 255]
        nombre -> Varchar,
        is_active -> Bool,
        #[max_length = 255]
        mime_type -> Varchar,
        tamano_bytes -> Int8,
        ancho -> Nullable<Int4>,
        alto -> Nullable<Int4>,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    imagen_perfil (id) {
        id -> Int4,
        user_id -> Int4,
        url_image -> Text,
        #[max_length = 20]
        tipo -> Varchar,
        #[max_length = 255]
        nombre -> Varchar,
        is_active -> Bool,
        #[max_length = 255]
        mime_type -> Varchar,
        tamano_bytes -> Int8,
        ancho -> Nullable<Int4>,
        alto -> Nullable<Int4>,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    libro (id) {
        id -> Int4,
        #[max_length = 150]
        titulo -> Varchar,
        #[max_length = 255]
        slug -> Varchar,
        sinopsis -> Nullable<Text>,
        tipo_id -> Int4,
        publicacion -> Date,
        estado_id -> Int4,
        visibilidad -> Nullable<Bool>,
        updated_at -> Timestamp,
        created_at -> Timestamp,
    }
}

diesel::table! {
    libro_autor (libro_id, autor_id) {
        libro_id -> Int4,
        autor_id -> Int4,
    }
}

diesel::table! {
    libro_estado (id) {
        id -> Int4,
        #[max_length = 50]
        nombre -> Varchar,
    }
}

diesel::table! {
    libro_etiqueta (libro_id, etiqueta_id) {
        libro_id -> Int4,
        etiqueta_id -> Int4,
    }
}

diesel::table! {
    libro_genero (libro_id, genero_id) {
        libro_id -> Int4,
        genero_id -> Int4,
    }
}

diesel::table! {
    libro_tipo (id) {
        id -> Int4,
        #[max_length = 50]
        nombre -> Varchar,
    }
}

diesel::table! {
    nombre_alternativo (id) {
        id -> Int4,
        libro_id -> Int4,
        #[max_length = 12]
        codigo -> Varchar,
        nombre -> Text,
        is_original -> Bool,
    }
}

diesel::table! {
    token_recuperacion (id) {
        id -> Int4,
        user_id -> Int4,
        token -> Text,
        #[max_length = 20]
        tipo -> Varchar,
        expira -> Timestamp,
    }
}

diesel::table! {
    usuario (id) {
        id -> Int4,
        #[max_length = 60]
        nickname -> Varchar,
        #[max_length = 60]
        username -> Varchar,
        password_hash -> Text,
        #[max_length = 255]
        email -> Varchar,
        is_active -> Bool,
        updated_at -> Timestamp,
        created_at -> Timestamp,
    }
}

diesel::table! {
    usuario_libro (id) {
        id -> Int4,
        usuario_id -> Int4,
        libro_id -> Int4,
        #[max_length = 50]
        estado -> Nullable<Varchar>,
        favorito -> Bool,
        creado -> Timestamp,
    }
}

diesel::joinable!(auth_tokens -> usuario (user_id));
diesel::joinable!(bloque_capitulo -> capitulo (capitulo_id));
diesel::joinable!(capitulo -> libro (libro_id));
diesel::joinable!(imagen_libro -> libro (libro_id));
diesel::joinable!(imagen_perfil -> usuario (user_id));
diesel::joinable!(libro -> libro_estado (estado_id));
diesel::joinable!(libro -> libro_tipo (tipo_id));
diesel::joinable!(libro_autor -> autor (autor_id));
diesel::joinable!(libro_autor -> libro (libro_id));
diesel::joinable!(libro_etiqueta -> etiqueta (etiqueta_id));
diesel::joinable!(libro_etiqueta -> libro (libro_id));
diesel::joinable!(libro_genero -> genero (genero_id));
diesel::joinable!(libro_genero -> libro (libro_id));
diesel::joinable!(nombre_alternativo -> libro (libro_id));
diesel::joinable!(token_recuperacion -> usuario (user_id));
diesel::joinable!(usuario_libro -> libro (libro_id));
diesel::joinable!(usuario_libro -> usuario (usuario_id));

diesel::allow_tables_to_appear_in_same_query!(
    auth_tokens,autor,bloque_capitulo,capitulo,etiqueta,genero,imagen_libro,imagen_perfil,libro,libro_autor,libro_estado,libro_etiqueta,libro_genero,libro_tipo,nombre_alternativo,token_recuperacion,usuario,usuario_libro,);
