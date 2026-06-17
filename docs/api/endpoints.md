# 📡 API de Biblioteca - Guía de Endpoints

## Base URL

## json format
Todos los endpoints con sus métodos, rutas y descripciones, enfocado en CRUD

./endpoints/usuario.jsonc

./endpoints/libro.jsonc

## Example
Se definen los ejemplos de entrada o salida de datos que pueda requerir el endpoint junto con comentarios sobre lo opcional o notas

./example/usuario.jsonc

./example/libro.json

## Endpoints backend

Método	Endpoint	Autenticación	Descripción
POST	/api/usuarios/registrar	❌	Registrar nuevo usuario
POST	/api/usuarios/login	❌	Iniciar sesión (JWT)
GET	/api/usuarios/perfil	✅	Obtener perfil
PUT	/api/usuarios/perfil	✅	Actualizar perfil
PUT	/api/usuarios/perfil/password	✅	Cambiar contraseña
PUT	/api/usuarios/perfil/delete	✅	Desactivar cuenta (soft delete)
PUT	/api/usuarios/perfil/reactivar	✅	Reactivar cuenta
POST	/api/usuarios/logout	✅	Cerrar sesión
POST	/api/usuarios/perfil/avatar	✅	Subir avatar
POST	/api/usuarios/perfil/portada	✅	Subir portada
GET	/api/usuarios/perfil/imagenes	✅	Obtener imágenes
DELETE	/api/usuarios/perfil/avatar	✅	Eliminar avatar
DELETE	/api/usuarios/perfil/portada	✅	Eliminar portada
