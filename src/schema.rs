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
    autor (id) {
        id -> Int4,
        fk_libro -> Nullable<Int4>,
        #[max_length = 100]
        nombre -> Nullable<Varchar>,
        #[max_length = 100]
        apellido -> Nullable<Varchar>,
        perfil -> Nullable<Varchar>,
    }
}

diesel::table! {
    capitulos (id) {
        id -> Int4,
        nombre -> Nullable<Varchar>,
        link -> Nullable<Text>,
        imagen -> Nullable<Text>,
        fk_libro -> Nullable<Int4>,
    }
}

diesel::table! {
    etiqueta (id) {
        id -> Int4,
        imagen -> Nullable<Varchar>,
        descripcion -> Nullable<Varchar>,
        visibilidad -> Nullable<Bool>,
        color -> Nullable<Varchar>,
        fk_usuario -> Nullable<Int4>,
    }
}

diesel::table! {
    etiqueta_list (id) {
        id -> Int4,
        fk_etiqueta -> Nullable<Int4>,
        fk_libro -> Nullable<Int4>,
    }
}

diesel::table! {
    genero (id) {
        id -> Int4,
        #[max_length = 50]
        nombre -> Nullable<Varchar>,
        descripcion -> Nullable<Varchar>,
    }
}

diesel::table! {
    libro (id) {
        id -> Int4,
        #[max_length = 100]
        titulo -> Nullable<Varchar>,
        perfil -> Nullable<Text>,
        sinopsis -> Nullable<Text>,
        #[max_length = 100]
        tipo -> Nullable<Varchar>,
        #[max_length = 100]
        capitulos -> Nullable<Varchar>,
        publicacion -> Date,
        #[max_length = 100]
        estado -> Nullable<Varchar>,
    }
}

diesel::table! {
    libro_genero (id) {
        id -> Int4,
        libro_id -> Nullable<Int4>,
        genero_id -> Nullable<Int4>,
    }
}

diesel::table! {
    marcapaginas (id) {
        id -> Int4,
        fk_usuario -> Nullable<Int4>,
        posicion -> Nullable<Float8>,
        nota -> Nullable<Text>,
        #[max_length = 50]
        etiqueta -> Nullable<Varchar>,
        creado -> Timestamp,
        actualizado -> Timestamp,
        fk_capitulo -> Nullable<Int4>,
    }
}

diesel::table! {
    miembros (id) {
        id -> Int4,
        fk_scan -> Nullable<Int4>,
        fk_usuario -> Nullable<Int4>,
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
    nombres (id) {
        id -> Int4,
        libro_id -> Nullable<Int4>,
        #[max_length = 100]
        nombre -> Nullable<Varchar>,
        descripcion -> Nullable<Text>,
    }
}

diesel::table! {
    publicador (id) {
        id -> Int4,
        scan_id -> Nullable<Int4>,
        autor_id -> Nullable<Int4>,
        libro_id -> Nullable<Int4>,
    }
}

diesel::table! {
    puntaje (id) {
        id -> Int4,
        usuario_id -> Nullable<Int4>,
        libro_id -> Nullable<Int4>,
        calificacion -> Nullable<Float8>,
        fecha_calificacion -> Timestamp,
    }
}

diesel::table! {
    scan (id) {
        id -> Int4,
        nombre -> Nullable<Varchar>,
        portada -> Nullable<Text>,
        perfil -> Nullable<Text>,
        descripcion -> Nullable<Varchar>,
        redsocial -> Nullable<Varchar>,
        creacion -> Timestamp,
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
    usuario (id) {
        id -> Int4,
        nickname -> Nullable<Varchar>,
        perfil -> Nullable<Text>,
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
        #[max_length = 60]
        nombre -> Varchar,
        #[max_length = 60]
        apellido -> Nullable<Varchar>,
    }
}

diesel::joinable!(auth_tokens -> usuario (user_id));
diesel::joinable!(autor -> libro (fk_libro));
diesel::joinable!(capitulos -> libro (fk_libro));
diesel::joinable!(etiqueta -> usuario (fk_usuario));
diesel::joinable!(etiqueta_list -> etiqueta (fk_etiqueta));
diesel::joinable!(etiqueta_list -> libro (fk_libro));
diesel::joinable!(libro_genero -> genero (genero_id));
diesel::joinable!(libro_genero -> libro (libro_id));
diesel::joinable!(marcapaginas -> capitulos (fk_capitulo));
diesel::joinable!(marcapaginas -> usuario (fk_usuario));
diesel::joinable!(miembros -> scan (fk_scan));
diesel::joinable!(miembros -> usuario (fk_usuario));
diesel::joinable!(multidispositivos -> usuario (user_id));
diesel::joinable!(nombres -> libro (libro_id));
diesel::joinable!(publicador -> capitulos (libro_id));
diesel::joinable!(publicador -> scan (scan_id));
diesel::joinable!(publicador -> usuario (autor_id));
diesel::joinable!(puntaje -> libro (libro_id));
diesel::joinable!(puntaje -> usuario (usuario_id));
diesel::joinable!(sessions -> usuario (user_id));
diesel::joinable!(token_recuperacion -> usuario (user_id));

diesel::allow_tables_to_appear_in_same_query!(
    auth_tokens,
    autor,
    capitulos,
    etiqueta,
    etiqueta_list,
    genero,
    libro,
    libro_genero,
    marcapaginas,
    miembros,
    multidispositivos,
    nombres,
    publicador,
    puntaje,
    scan,
    sessions,
    token_recuperacion,
    usuario,
    usuariosss,
);
