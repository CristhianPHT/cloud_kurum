CREATE TABLE capitulo (
  id SERIAL PRIMARY KEY,
  libro_id INT NOT NULL,
  numero INT NOT NULL,  -- Número secuencial del capítulo dentro del libro (1, 2, 3, ...)
  titulo VARCHAR(255),  -- titulo del capítulo (personalizable)
  visibilidad BOOLEAN NOT NULL DEFAULT TRUE,
  created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
  updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
  CONSTRAINT fk_capitulo_libro FOREIGN KEY (libro_id) REFERENCES libro(id) ON DELETE CASCADE
);

CREATE TABLE bloque_capitulo (  -- No representa bloques visuales, Representa recursos.
  id SERIAL PRIMARY KEY,
  capitulo_id INT NOT NULL,
  orden INT NOT NULL, -- Posición del bloque dentro del capítulo para mantener el orden de lectura
  tipo VARCHAR(20) NOT NULL,  -- Tipo de contenido del bloque (texto, imagen, video, markdown, etc.)
  recurso_url TEXT NOT NULL, -- ULR del markdown, archivos o directamente contenido txt aquí.
  layout VARCHAR(20),   -- Disposición visual opcional del bloque (left, right, center, full, etc.)
  created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
  CONSTRAINT fk_bloque_capitulo FOREIGN KEY (capitulo_id) REFERENCES capitulo(id) ON DELETE CASCADE
);