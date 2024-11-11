## Saludo = Select * from usuarios wheree id == id;
## Nuevo_usuario = Inser into Usuarios VALUES (x,y,z);
## elimiinar = DELETE usurios por id
## modific_user = modificar el usuario por el id.
## test termnal inputs and outputs:

### cargo run --bin show_post
# Proyecto Actix Web con Diesel ORM

Este proyecto es una API backend construida con **Actix Web** y **Diesel ORM** para interactuar con una base de datos PostgreSQL. El objetivo es crear un sistema modularizado en Rust, donde la base de datos y la lógica de la API se gestionan de forma estructurada.

## Estructura del Proyecto

Este es un resumen de los archivos y directorios más importantes del proyecto:

### Archivos Principales

- **`main.rs`**: Es el archivo de entrada de la aplicación. Aquí se configura la conexión a la base de datos y se inician las rutas y el servidor web con Actix.
- **`lib.rs`**: Contiene la lógica central para interactuar con la base de datos utilizando **Diesel ORM**. Aquí se gestionan las consultas SQL, inserciones y modificaciones.
- **`use_sql.rs`**: Este archivo en el directorio `bin/` se utiliza para hacer las llamadas necesarias a las funciones definidas en `lib.rs` con personalización y sin lógica redundante.
- **`models.rs`**: Define los modelos de datos para las entidades que manejas, como los usuarios. Utiliza la funcionalidad de Diesel para mapear las tablas de la base de datos.
- **`schema.rs`**: Contiene las definiciones de las tablas en la base de datos que **Diesel** utiliza para generar las consultas SQL.

### Archivos Raíz

- **`.env`**: Contiene las variables de entorno necesarias para la configuración de la base de datos (como `DATABASE_URL`).
- **`Cargo.toml`**: Es el archivo de configuración de dependencias de Cargo, que define las bibliotecas utilizadas, como Actix Web y Diesel.

## Descripción de los Archivos

### `src/lib.rs`
Este archivo contiene las sentencias SQL que se utilizan para interactuar con la base de datos, junto con sus requisitos y configuraciones. **Diesel ORM** es utilizado aquí para gestionar la base de datos y las operaciones CRUD (crear, leer, actualizar, eliminar). También se definen las estructuras que representan las tablas en la base de datos.

#### Ejemplo de uso:
```rust
pub fn obtener_usuario_por_id(id: i32) -> QueryResult<Usuario> {
    use crate::schema::usuarios::dsl::*;
    usuarios.filter(id.eq(id)).first::<Usuario>(&connection)
}
