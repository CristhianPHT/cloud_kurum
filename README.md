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


# Base de datos
### Comprobar que está encendido localmente:
```bash
sudo systemctl status postgresql
```

### Encender postgresql en el sistema:
```bash
sudo systemctl start postgresql
```
# 🚀 Mi Proyecto

Bienvenido a la documentación principal del proyecto. Usa las secciones siguientes como referencia rápida:

## 📂 Índice

- [📦 Base de datos](#base-de-datos)
- [🦀 Servidor Rust](#servidor-rust)

---
