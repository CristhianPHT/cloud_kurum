CREATE TABLE etiqueta (
  id SERIAL PRIMARY KEY,
  nombre VARCHAR(50) NOT NULL UNIQUE,
  imagen VARCHAR, -- icon o estilo (sólo ascii, emojis, etc)
  descripcion VARCHAR(200),  -- alguna etiqueta muy larga o muy personal que requiere un recordatorio largo
  visibilidad BOOLEAN DEFAULT true,  -- para etiquetas privadas sólo para el usuario
  color VARCHAR  -- en relación con imagen, le da carácteristica y personalización
);
CREATE TABLE libro_etiqueta (
  libro_id INT NOT NULL,
  etiqueta_id INT NOT NULL,

  PRIMARY KEY (libro_id, etiqueta_id),
  CONSTRAINT fk_libro_etiqueta_libro FOREIGN KEY (libro_id) REFERENCES libro(id) ON DELETE CASCADE,
  CONSTRAINT fk_libro_etiqueta_etiqueta FOREIGN KEY (etiqueta_id) REFERENCES etiqueta(id) ON DELETE RESTRICT
  -- Restrict, evita eliminar la etiqueta si otro libro aún lo usa
);