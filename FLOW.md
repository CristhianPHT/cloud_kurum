# Referencia a digragrama de casos de uso UML, BPMN~

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

# Flujo de datos sobre Registrarse una Nueva Cuenta -> Account
- Se crea una conección a través de Diesel usando la creación automática de modelo/esquemas (up.sql->schema.rs)
- Se crea múltiples 'struct' según se requiera o se desea (models.rs)
- Para nuestro caso "struct NuevoAccount" con la mayor o total similitud con el esquema que se creó (models.rs)
- El ingreso se hace a través de la api, actix_web, entonces recibimos un POST con todos los datos en la función 'insert_login'

## Función insert_login
(interface.rs)
### Descripción:
Recibe los datos de registro de un usuario en formato JSON, procesa la información, ejecuta validaciones internas caso ideal responde enviando el token al cliente para que pase directamente cómo si se ubiese logeado directamente.
-El token se genera y envía inmediatamente después del registro, lo que implica que no se requiere verificación en dos pasos ni confirmación de correo electrónico. El usuario queda autenticado directamente tras completar el registro.
#### Parámetro:
web::Json<[NuevoAccount](#estructura-nuevoaccount)>
#### Retorno:
impl Responder
### Flujo de la función
1. Se establece la conexión con la base de datos mediante la función `establish_connection()`.
2. Se transforma el JSON recibido en el struct `NuevoAccount` usando `into_inner()`.
3. Se ejecutan validaciones básicas:
   * `username` no debe estar vacío.
   * `password` no debe estar vacío.
   * la contraseña no debe superar **128 caracteres**.
4. Se intenta registrar el usuario en la base de datos usando la función `insert_usuario`.
   * Si el registro es exitoso, se obtiene el **ID del nuevo usuario**.
   * Si ocurre una violación de unicidad (`UNIQUE`) en `username` o `email`, se devuelve un **409 Conflict**.
5. Se calcula el tiempo de expiración del token mediante `calculate_expiration()`.
6. Se genera un **token de autenticación (JWT)** usando el ID del usuario y el tiempo de expiración.
7. El token generado se almacena en la base de datos mediante la función `insert_auth_token`.
8. Finalmente, el servidor responde al cliente enviando el token generado.


## Función insert_auth_token
(/modules/auth.rs)
### Descripción
Inserta un token de autenticación en la base de datos asociado a un usuario.
Este token representa una sesión activa que permitirá al usuario autenticarse en futuras solicitudes.
### Parámetros
conn: &mut PgConnection ; Conexión activa a la base de datos
user_id_input: i32 ; Identificador(id) del usuario al que pertenece el token
token_input: &str ; Token de autenticación generado previamente
expira_input: DateTime<Utc> ; Fecha y hora de expiración del token
### Retorno:
QueryResult<String>
- Ok(token) si el token se insertó correctamente en la base de datos.
- Err(Error) si ocurre algún problema durante la inserción.
### Flujo de la función
1. Se reciben los datos necesarios para registrar un token de autenticación.
2. Se construye una instancia del struct `NuevoAuthToken`, que contiene los campos requeridos para insertar un nuevo registro en la tabla `auth_tokens`.
3. Se asignan los valores correspondientes:
   * `user_id` se establece con el ID del usuario recibido.
   * `token` se convierte a `String`.
   * `dispositivo` se establece como `None`.
   * `expira` se convierte de `DateTime<Utc>` a `NaiveDateTime`.
   * `activo` se establece como `true`.
4. Se ejecuta la operación de inserción utilizando la función `insert_into` de Diesel.
5. Se usa `.returning(token)` para obtener el token insertado desde la base de datos.
6. Finalmente, se devuelve el resultado de la operación.


## Función login_usuario
(interface.rs)
### Descripción
Endpoint encargado de autenticar a un usuario existente.
Recibe las credenciales del usuario en formato JSON, valida los datos, verifica la contraseña contra el hash almacenado en la base de datos y, si la autenticación es correcta, genera un **token de sesión** que se almacena en la base de datos y se envía al cliente.
Este token permite que el usuario quede autenticado y pueda realizar futuras solicitudes sin volver a enviar sus credenciales.
### Parámetro
`web::Json<LoginAccount>`
Estructura que contiene las credenciales del usuario enviadas desde el cliente.
### Retorno
`impl Responder`
Responde con un objeto JSON que contiene:
* **token** → si la autenticación es exitosa.
* **error** → si ocurre algún problema durante el proceso de autenticación.
# Flujo de la función
* Primero se establece una conexión con la base de datos mediante la función `establish_connection()`.
* Se transforma el JSON recibido en la estructura `LoginAccount` usando `into_inner()` para trabajar directamente con sus datos.
* Se valida que los campos `username` y `password_hash` no estén vacíos.
  Si alguno de ellos está vacío se retorna un **BadRequest** indicando que ambos campos son obligatorios.
* Se llama a la función de base de datos `login_usuario_hashed`, la cual:
  * Busca el usuario en la base de datos.
  * Obtiene el `password_hash` almacenado.
  * Verifica la contraseña enviada contra el hash almacenado.
* Si la verificación es correcta, la función retorna el `id` del usuario autenticado.
* Se calcula el tiempo de expiración del token utilizando la función `calculate_expiration()`.
* Se genera un **JWT** utilizando la función `generate_jwt`, que incluye el identificador del usuario y la fecha de expiración.
* Una vez generado el token, se almacena en la base de datos utilizando la función `insert_auth_token`.
  Esta función registra el token como una sesión activa asociada al usuario.
* Si el token se almacena correctamente, se responde al cliente con el token generado.




## Estructura NuevoAccount


### Componentes/funciones internas
...




# Acciones
1️⃣ Register → crear usuario
2️⃣ Login → verificar usuario
3️⃣ Get profile → leer datos
4️⃣ Update profile → editar datos
5️⃣ cambiar contraseña
6️⃣ recuperar cuenta
7️⃣ logout











