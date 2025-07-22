# Base de datos
### Comprobar que está encendido localmente:
```bash
sudo systemctl status postgresql
```

### Encender postgresql en el sistema:
```bash
sudo systemctl start postgresql
```

### Verifica si la base de datos existe:
```bash
sudo -u postgres psql -l
```
##### Conectarse con la base de datos:
```bash
sudo -i -u postgres
psql
```
Ver las base de datos:
```bash
\l
```
Conectarse con la base de datos:
```bash
\c name_data_base      -- Conectarte a la base de datos llamada "name_data_base"
```
Dentro de la conección con una base de datos, poder ver las tablas de esa base de datos:
```bash
\dt          -- Ver tablas (si estás dentro de una base)
```
### Crear la base de datos si no existe y asignando a un usuarioX:
```bash
sudo -u postgres createdb -O usuarioXexistente nameXdataBase
```
## Rust (diesel) y postgresql:
### Instalar la herramienta CLI para que Diesel funcione:
```bash
cargo install diesel_cli
```
Si presenta errores para la instalación/actualización:
```bash
cargo install diesel_cli --no-default-features --features postgres
``` 
## Crear los archivos para la migración a de diesel (rust) a PostgreSQL
```bash
diesel migration generate contenedorFileSQL
```
Creará los archivos down.sql y up.sql dentro de una carpeta nueva con la fecha y el nombre dado(contenedorFileSQL), ingresar en formato sql para crear tablas en up.sql, para eliminar las tablas en down.sql.
Ya hecho eso debería estar listo todas sus tablas para poder migrar a postgresql.
### Migrar tablas OMR a la base de datos postgresql:
```bash
diesel migration run
```
Eliminar las tablas creadas en nuestra migración:
```bash
diesel migration redo
```