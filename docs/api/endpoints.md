# 📡 API de Biblioteca - Guía de Endpoints

# 📡 API de Biblioteca - Guía de Endpoints

## Estructura de un endpoint

Todos los endpoints siguen esta estructura.

### Campos base (siempre presentes)

```json
{
  "method": "",
  "path": "",
  "descripcion": "",
  "autenticacion": false,
  "respuesta": {},
  "errores": {}
}
```

### Campos opcionales

Estos solo aparecen cuando el endpoint los requiere.

```json
{
  "headers": {},
  "params": {},
  "query": {},
  "body": {}
}
```

### Descripción de los campos

| Campo          | Obligatorio | Descripción                                      |
|----------------|:-----------:|--------------------------------------------------|
| method         | ✅          | Método HTTP (`GET`, `POST`, `PUT`, etc.).        |
| path           | ✅          | Ruta del endpoint.                               |
| descripcion    | ✅          | Qué hace el endpoint.                            |
| autenticacion  | ✅          | Indica si requiere JWT.                          |
| respuesta      | ✅          | Ejemplo de respuesta exitosa.                    |
| errores        | ✅          | Errores que puede devolver.                      |
| headers        | ❌          | Headers requeridos (ej. `Authorization`).        |
| params         | ❌          | Parámetros de ruta (`:id`, `:nickname`).         |
| query          | ❌          | Parámetros de consulta (`?page=1`).              |
| body           | ❌          | Datos enviados en la petición.                   |

## Base URL
## json format

Todos los endpoints con sus métodos, rutas y descripciones, enfocado en CRUD

./endpoints/usuario.jsonc

./endpoints/libro.jsonc

## Example
Se definen los ejemplos de entrada o salida de datos que pueda requerir el endpoint junto con comentarios sobre lo opcional o notas

./example/usuario.jsonc

./example/libro.json

## Endpoints backend usuario
|    | Método | Endpoint              | Versión | Auth | Descripción                       |
|----|--------|-----------------------|---------|------|-----------------------------------|
| 🧩 | POST   | /users/register       | v1.0    | ❌   | Registrar usuario                 |
| 🧩 | POST   | /users/login          | v1.0    | ❌   | Iniciar sesión                    |
| 🧩 | GET    | /users/:nickname      | v1.0    | ❌   | Perfil público                    |
| 🧩 | GET    | /me                   | v1.0    | 🔒   | Mi perfil privado                 |
| 🧩 | GET    | /me/header            | v1.0    | 🔒   | Mi perfil indispensable           |
|    | POST   | /me/avatar            | v1.0    | 🔒   | Subir avatar                      |
|    | POST   | /me/portada           | v1.0    | 🔒   | Subir portada                     |
|    | GET    | /me/imagenes          | v1.0    | 🔒   | Mis imágenes                      |
| 🧩 | PUT    | /me/password          | v2.0    | 🔒   | Cambiar contraseña                |
| 🧩 | POST   | /me/logout            | v2.0    | 🔒   | Cerrar sesión                     |
| 🧩 | PUT    | /me/nickname          | v2.0    | 🔒   | Actualizar perfil                 |
|    | PUT    | /me/email             | v2.0    | 🔒   | Actualizar email                  |
|    | PUT    | /me/username          | v2.0    | 🔒   | Actualizar username               |
|    | PUT    | /me/delete            | v2.0    | 🔒   | Desactivar cuenta                 |
|    | DELETE | /me/avatar            | v2.0    | 🔒   | Eliminar avatar                   |
|    | DELETE | /me/portada           | v2.0    | 🔒   | Eliminar portada                  |
|    | PUT    | /me/reactivar         | v3.0    | 🔒   | Reactivar cuenta                  |


8. GET `/api/me/imagenes`
9. POST `/api/me/avatar`
10. POST `/api/me/portada`
11. DELETE `/api/me/avatar`
12. DELETE `/api/me/portada`
13. PUT `/api/me/delete`
14. PUT `/api/me/reactivar`
falta arriba endpoint (ennumerados)
## Endpoints backend libro
|    | Método | Endpoint                  | Versión | Auth | Descripción                                       |
|----|--------|---------------------------|---------|------|---------------------------------------------------|
| 🧩 | GET    | /users/:nick_name/books   | v1.0    | ❌   | Libros públicos de un usuario                     |
| 🧩 | GET    | /books/:page              | v1.0    | ❌   | Últimos libros publicados (falta completar)       |
| 🧩 | GET    | /books/:slug              | v1.0    | ❌   | Obtener un libro mediante su slug                 |
| 🧩 | GET    | /me/books                 | v1.0    | 🔒   | Todos los libros del usuario                      |
| 🧩 | GET    | /me/sufle_books           | v1.0    | 🔒   | Todos los libros con imagen del usuario           |
| 🧩 | POST   | /books                    | v1.0    | 🔒   | Subir un libro                                    |
| 🧩 | POST   | /me/books_by_user         | v1.0    | 🔒   | Crea relación entre libro y usuario (transacción) |