-- This file should undo anything in `up.sql`
DROP TABLE miembros;
DROP TABLE publicador CASCADE; -- Eliminar primero las tablas que dependen de capitulos
DROP TABLE marcapaginas CASCADE;
DROP TABLE nombres CASCADE;
DROP TABLE puntaje CASCADE;
DROP TABLE libro_genero CASCADE;
DROP TABLE scan CASCADE;
DROP TABLE etiqueta_list CASCADE;
DROP TABLE etiqueta CASCADE;
DROP TABLE autor CASCADE;
DROP TABLE capitulos CASCADE; -- Ahora puedes eliminar capitulos
DROP TABLE libro CASCADE;
DROP TABLE genero CASCADE;
