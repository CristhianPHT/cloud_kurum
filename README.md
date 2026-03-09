# 📚 Documentación

- [📄 SQL](./SQL.md)
- [🚀 Servidor Rust](./SERVER.md)

## Saludo = Select * from usuarios wheree id == id;
## Nuevo_usuario = Inser into Usuarios VALUES (x,y,z);
## elimiinar = DELETE usurios por id
## modific_user = modificar el usuario por el id.
## test termnal inputs and outputs:

### cargo run --bin backend
# Proyecto Actix Web con Diesel ORM

Este proyecto es una API backend construida con **Actix Web** y **Diesel ORM** para interactuar con una base de datos PostgreSQL. El objetivo es crear un sistema modularizado en Rust, donde la base de datos y la lógica de la API se gestionan de forma estructurada.

## Estructura del Proyecto

Este es un resumen de los archivos y directorios más importantes del proyecto:

### 📁 Nueva Estructura Modular

El código ha sido reorganizado en módulos especializados para mejorar la mantenibilidad y escalabilidad:

#### 🔧 **Módulos Creados:**

1. **`modules/database.rs`** - Funciones de conexión y utilidades genéricas
   - `establish_connection()` - Establece conexión a PostgreSQL
   - `select_by_id()` - Función genérica para seleccionar por ID
   - `generic_insert()` - Función genérica para insertar datos

2. **`modules/usuarios.rs`** - Operaciones con la tabla `usuariosss`
   - `select_id()` - Seleccionar usuario por ID
   - `select_all_users()` - Obtener todos los usuarios con paginación
   - `update_user_id()` - Actualizar usuario por ID
   - `insert_user()` - Insertar nuevo usuario

3. **`modules/account.rs`** - Funciones de autenticación y login
   - `insert_usuario()` - Crear nueva cuenta con hash de contraseña
   - `login_usuario_hashed()` - Autenticar usuario con bcrypt
   - `username_existe()` - Verificar si existe el nombre de usuario
   - `select_id_usuario()` - Obtener cuenta por ID
   - `update_login()` - Actualizar datos de login

4. **`modules/auth.rs`** - Manejo de tokens JWT
   - `generate_jwt()` - Generar token JWT
   - `insert_auth_token()` - Guardar token en base de datos
   - `select_id_token()` - Obtener usuario por token
   - `calculate_expiration()` - Calcular fecha de expiración

5. **`modules/libros.rs`** - Operaciones con libros
   - `select_nombre_libros()` - Obtener lista de libros para dashboard
   - `insert_libro_nuevo()` - Insertar nuevo libro
   - `select_libro_main()` - Obtener libro por ID

6. **`modules/generos.rs`** - Operaciones con géneros
   - `insert_gen_new()` - Insertar nuevo género
   - `select_gen_all()` - Obtener todos los géneros
   - `select_gen_unico()` - Obtener género por ID

7. **`modules/relaciones.rs`** - Relaciones entre tablas (libro-género)
   - `insert_libro_genero()` - Relacionar libro con género
   - `buscar_libros_por_genero()` - Buscar libros filtrados por género
   - `OrdenamientoLibro` - Enum para ordenamiento de resultados

#### ✅ **Ventajas de esta organización:**

- **Mantenibilidad**: Código más fácil de encontrar y modificar
- **Escalabilidad**: Fácil agregar nuevas funciones a cada módulo  
- **Legibilidad**: Estructura clara y lógica
- **Reutilización**: Módulos independientes y reutilizables
- **Compatibilidad**: Mantiene la API existente gracias a los re-exports

#### 🔄 **Uso del código:**

El código existente sigue funcionando igual. Puedes usar las funciones como antes:
```rust
use nube_kurum::insert_usuario;  // Función directa
```

O usar los módulos específicos:
```rust
use nube_kurum::usuarios::select_id;    // Desde módulo específico
use nube_kurum::account::login_usuario_hashed;
use nube_kurum::libros::insert_libro_nuevo;
```

### Archivos Principales

- **`bin/backend.rs`**: Es el archivo de entrada de la aplicación. Aquí se configura la conexión a la base de datos y se inician las rutas y el servidor web con Actix.
- **`lib.rs`**: Contiene la lógica central para interactuar con la base de datos utilizando **Diesel ORM**. Aquí se gestionan las consultas SQL, (insert_id, update_id, select_all, select_id).
- **`models.rs`**: Define los modelos de datos para las entidades que manejas. Utiliza la funcionalidad de Diesel para mapear las tablas de la base de datos (Usuario, NuevoUsuario, UsuarioUpdate).
- **`schema.rs`**: Contiene las definiciones de las tablas en la base de datos que **Diesel** utiliza para generar las consultas SQL (usuariosss).
- **`web/interface.rs`**: Este archivo en el directorio `web/` se utiliza para hacer las llamadas necesarias a las funciones definidas en `lib.rs` dónde será la salida en formato json como API para get, post y put (show_user, create_user, update_user).

### Archivos Raíz

- **`.env`**: Contiene las variables de entorno necesarias para la configuración de la base de datos (como `DATABASE_URL`).
- **`Cargo.toml`**: Es el archivo de configuración de dependencias de Cargo, que define las bibliotecas utilizadas, como Actix Web y Diesel.

## Descripción de los Archivos

### `src/lib.rs`
Este archivo contiene las sentencias SQL que se utilizan para interactuar con la base de datos, junto con sus requisitos y configuraciones. **Diesel ORM** es utilizado aquí para gestionar la base de datos y las operaciones CRUD (crear, leer, actualizar, eliminar). También se definen las estructuras que representan las tablas en la base de datos.

#### Ejemplo de uso:
```rust
pub fn select_id(conn: &mut PgConnection, usuario_id: i32) -> Usuario {  // para mostrar usuario por id = input(conn, id)
  let usuario = usuariosss
    .find(usuario_id)
    .first::<Usuario>(conn)
    .expect("Error al buscar el usuario");
  usuario
}
```


## Creación de tablas SQL en terminal:
diesel migrations generate name_file -----> from reference `https://diesel.rs/guides/getting-started`
### Correr el programa y migrar a postgres psql
'''diesel migration run'''
### Eliminar tablas creadas antes de postgres psql
'''diesel migraition redo'''
generando un archivo schema.rs por defecto que utilizarás


## Tablas Principales:
Usuario, Libro,

## Cloudflare R2
Cloudfrare R2 es almacenamiento privado (no muestra imágenes por sí solo).

#### Necesitas:

- POST /imagen/perfil → sube imagen a R2 y guarda la key en PostgreSQL.
- GET /imagen/perfil → obtiene la key, verifica el JWT, descarga imagen de R2 y la devuelve.

#### Tu backend:
- Verifica el token JWT.
- Usa las Access Key y Secret Key de R2 para conectar.
- Sirve las imágenes protegidas como una API segura.
#### No se guardan URLs públicas, solo claves (keys) internas.


# Para poder implementar la API de imágenes se requiere reestructurar los archivos y carpetas.
src/
├── bin/
│   └── backend.rs                 # Punto de entrada
├── lib.rs                         # Configuración y exports principales
├── config/
│   ├── mod.rs                     # Configuración de la app
│   ├── database.rs                # Setup de DB
│   └── cloudflare.rs              # Setup de R2
├── models/
│   ├── mod.rs
│   ├── user.rs                    # Modelos de usuario
│   ├── book.rs                    # Modelos de libros
│   ├── auth.rs                    # Modelos de autenticación
│   └── image.rs                   # Modelos de imágenes (NUEVO)
├── repositories/
│   ├── mod.rs
│   ├── user_repository.rs         # Acceso a datos de usuarios
│   ├── book_repository.rs         # Acceso a datos de libros
│   └── image_repository.rs        # Acceso a datos de imágenes (NUEVO)
├── services/
│   ├── mod.rs
│   ├── user_service.rs            # Lógica de negocio usuarios
│   ├── book_service.rs            # Lógica de negocio libros
│   ├── auth_service.rs            # Lógica de autenticación
│   └── image_service.rs           # Lógica de imágenes R2 (NUEVO)
├── handlers/
│   ├── mod.rs
│   ├── user_handler.rs            # Endpoints de usuarios
│   ├── book_handler.rs            # Endpoints de libros
│   ├── auth_handler.rs            # Endpoints de auth
│   └── image_handler.rs           # Endpoints de imágenes (NUEVO)
├── middleware/
│   ├── mod.rs
│   ├── auth.rs                    # Middleware de autenticación
│   └── cors.rs                    # Configuración CORS
├── utils/
│   ├── mod.rs
│   ├── jwt.rs                     # Utilidades JWT
│   ├── validation.rs              # Validaciones
│   └── r2_client.rs               # Cliente R2 (NUEVO)
└── schema.rs                      # Esquema Diesel (como está)
## Patrón Arquitectónico

El proyecto sigue una arquitectura en capas:

- Handlers: Exponen endpoints HTTP
- Services: Contienen lógica de negocio
- Repositories: Acceso a datos
- Models: Representación de entidades

#### -------------------
architecture.md debería tener:

Patrón arquitectónico

Estructura de carpetas

Tecnologías usadas

Decisiones importantes

Dependencias externas

flow.md debería tener:

Flujo de autenticación

Flujo de creación de libro

Flujo de subida de imagen

Flujo de refresh token (si existe)
#### -------------------
Guardarlos en /docs/flow.md:
Diagrama de casos de uso:...
Sección “Estado Actual” 

Sección “Versión Ideal / Meta”