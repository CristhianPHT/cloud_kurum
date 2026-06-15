-- Tabla relacional usuario con libros
CREATE TABLE usuario_libro (  --  Biblioteca personal (privado)
  id SERIAL PRIMARY KEY,
  usuario_id INT NOT NULL REFERENCES usuario(id),
  libro_id INT NOT NULL REFERENCES libro(id),
  estado VARCHAR(50), -- "Pendiente", "Completado", "Visto", "abandonado", etc.
  favorito BOOLEAN DEFAULT false NOT NULL,
  creado TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
  CONSTRAINT unique_user_lib UNIQUE (usuario_id, libro_id)
);
-- Modulo Lectura... 
-- usuario_libro 
-- progreso_lectura 
-- favorito 
-- historial_lectura