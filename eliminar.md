Generalmente chatgpt
--------------------
música

Dearly Beloved (Kingdom Hearts II Version) de Yoko Shimomura

Música de videojuegos (muy cercana al estilo)
To Zanarkand – Nobuo Uematsu
Piano simple, melancólico y muy emotivo. Probablemente lo más cercano en sentimiento.
Promise (Reprise) – Akira Yamaoka
Más oscuro, pero igual de introspectivo.
Aerith's Theme – Nobuo Uematsu
Triste pero cálido, muy emocional.
The Path – Gustavo Santaolalla
Minimalista, con una tristeza muy natural.
🎹 Piano / instrumental melancólico
Nuvole Bianche – Ludovico Einaudi
Muy emocional, va creciendo poco a poco.
Comptine d’un autre été – Yann Tiersen
Nostalgia pura, ligera pero con fondo triste.
River Flows in You – Yiruma
Más romántica, pero comparte esa suavidad emocional.
🌌 Más ambiental / etéreo
Weightless – Marconi Union
Muy calmante, casi meditativo.
An Ending (Ascent) – Brian Eno
Melancolía espacial, muy suave.
------------------------------

<!-- #[post("/auth")] fn auth_user(user:<NuevoAuthToken>) // Registrar un nuevo token con datos crudos, salida un token -->
#[post("/login")] fn login_usuario(user:<LoginAccount>) // ingresar con usuario y contraseña, salida un token
#[post("/register")] fn insert_login(user:<NuevoAccount>) // Registrar un nuevo usario con las validado, salida un token
#[put("/login/{id}")] fn update_usuario_login(user:<NuevoAccount>) // Actualizar usuario, simple, salida un mensaje
#[get("/libros")] fn get_libro_all() // Obtener absolutamente todos los libros
#[post{"/nuevolibro"}] fn post_nuevo_libro(param:<NuevoLibro>) // Registrar los datos para un nuevo libro, salida id
#[get("/libro/{id}")] fn get_libro_unique(id:Path<i32>) // Obtener toda la información sobre un id de libro, salida data

#[get("/me")] fn get_user(req: HttpRequest) // Validamos y mostramos la información del usuario usando un token, salida data
#[get("/me/header")] fn get_header(req: HttpRequest) // Validamos token y mostramos sólo lo indispensable para el header.
#[get("/user/{username_link}")] fn get_user_page(name:<String>) // Página publica del usuario.

#[get("/user/{username}/libros")] fn get_libros_publicos_x_user(username:<String>) // todos los libros públicos de un usuario
#[get("/me/libros")] fn get_all_books_user(req: HttpRequest) // Validamos token, y mostramos todos los libros de 1 usuario
#[post("/me/libros")] fn post_books_x_user(param:<NuevoLibroUsuario>) // Registrar la relación entre un libro y usuario. Tabla relacional, salida id
#[get("/me/sufle_libro")] fn get_books_x_user(id: Path<i32>) // Obtener varias tablas sql, de libros, relacionadas sobre 1 user

---------------------------------------

Backend (Rust + Actix + Diesel)
├── bin/backend.rs           (entry point)
├── infrastructure/db.rs     (conexión BD)
├── models/                  (reflejan tablas exactas)
├── repositories/            (queries Diesel + lógica BD)
├── services/                (lógica de negociok, reutilizable)
├── web/
│   ├── auth/               (tokens, validación)
│   ├── dto/                (struct, models para input/output JSON seguro)
│   └── handlers/           (rutas Actix, orquesta todo api)
├── lib.rs                  (módulos, crate)
└── schema.rs               (Diesel automático)
--------------------------------------------------
usuario	soft delete + auditoría
libro	soft delete
capitulo	soft delete
autor	soft delete
genero	delete físico
etiqueta	delete físico
lectura	depende del negocio
--------------------------------------------------
{
  "pasos_desarrollo": [
    {
      "paso": 1,
      "nombre": "Diseño de Base de Datos",
      "estado": "✅ COMPLETADO",
      "entregable": "Tablas SQL (usuario, imagen_perfil, libro, etc.)"
    },
    {
      "paso": 2,
      "nombre": "Definición de contratos de datos",
      "estado": "✅ COMPLETADO", 
      "entregable": "JSON con estructura de datos (el que acabas de hacer)"
    },
    {
      "paso": 3,
      "nombre": "Backend - API Endpoints",
      "estado": "⏳ EN PROGRESO",
      "tareas": [
        "CRUD para libro_tipo y libro_estado (tablas catálogo)",
        "CRUD para libro (crear, leer, actualizar, eliminar)",
        "Endpoint para subir imágenes a Cloudflare R2",
        "CRUD para nombre_alternativo",
        "Validaciones de datos",
        "Generación de slugs automática"
      ]
    },
    {
      "paso": 4,
      "nombre": "Backend - Lógica de negocio",
      "estado": "📋 PENDIENTE",
      "tareas": [
        "Procesamiento de imágenes (dimensiones, mime_type, tamaño)",
        "Manejo de portada activa (solo una por libro)",
        "Generación de slugs únicos",
        "Manejo de errores y validaciones"
      ]
    },
    {
      "paso": 5,
      "nombre": "Frontend - Consumo de API",
      "estado": "📋 PENDIENTE",
      "tareas": [
        "Formularios para crear/editar libros",
        "Subida de imágenes con preview",
        "Manejo de nombres alternativos",
        "Slug automático desde título"
      ]
    }
  ]
}