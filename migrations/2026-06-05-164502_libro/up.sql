CREATE TABLE libro_tipo (
  id SERIAL PRIMARY KEY,
  nombre VARCHAR(50) NOT NULL UNIQUE
);
CREATE TABLE libro_estado (
  id SERIAL PRIMARY KEY,
  nombre VARCHAR(50) NOT NULL UNIQUE
);
CREATE TABLE libro (
  id SERIAL PRIMARY KEY,
  titulo VARCHAR(150) NOT NULL,
  slug VARCHAR(255) NOT NULL UNIQUE, -- identificador legible y único utilizado para la URL.
  sinopsis TEXT,
  tipo_id INTEGER NOT NULL,  -- Novela, Comic, Manga, etc.
  publicacion DATE NOT NULL,  -- fecha de publicación del libro.
  estado_id INTEGER NOT NULL,  -- "Publicando", "En proceso", "Finalizado", "Suspendido", etc.
  visibilidad BOOLEAN DEFAULT TRUE, -- útil para libros privados o públicos.
  updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP, -- fecha de actualización del libro.
  created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP, -- fecha de creación del libro.
  CONSTRAINT fk_libro_tipo FOREIGN KEY (tipo_id) REFERENCES libro_tipo(id) ON DELETE RESTRICT,
  CONSTRAINT fk_libro_estado FOREIGN KEY (estado_id) REFERENCES libro_estado(id) ON DELETE RESTRICT
);
-- Tabla 1 a muchos de libro a imágenes, sobre las posibles imágenes
CREATE TABLE imagen_libro (
  id SERIAL PRIMARY KEY,
  libro_id INT NOT NULL,
  url_image TEXT NOT NULL,  -- url o link de la imagen, cloudflare r2.
  tipo VARCHAR(20) NOT NULL, -- será check luego: "portada", "BANNER", etc
  nombre VARCHAR (255) NOT NULL, -- necesario para frontend, alt = nombre.
  is_active BOOLEAN NOT NULL, -- false para imágen de portada antiguas y true for main
  mime_type VARCHAR (255) NOT NULL, -- image/jpeg, image/png, image/webp, etc.
  tamano_bytes BIGINT NOT NULL, -- tamaño en bytes de la imagen.
  ancho INT, -- pixeles
  alto INT, -- PIXELES
  created_at TIMESTAMP NOT NULL,
  updated_at TIMESTAMP NOT NULL,
  CONSTRAINT fk_imagen_libro FOREIGN KEY (libro_id) REFERENCES libro(id) ON DELETE CASCADE
);
-- Tabla Nombres alternativos del libro
CREATE TABLE nombre_alternativo ( -- sobre todos los nombres conocidos de una obra
  id SERIAL PRIMARY KEY,
  libro_id INT NOT NULL,
  codigo VARCHAR(12) NOT NULL,  -- modificable el n° varchar (pais)
  nombre TEXT NOT NULL, -- algunos nombres literalmente ponen la descripción aquí...
  is_original BOOLEAN NOT NULL DEFAULT FALSE, -- el nombre oficial y del idioma original
  CONSTRAINT fk_nombre_alternativo
    FOREIGN KEY (libro_id)
    REFERENCES libro(id)
    ON DELETE CASCADE
);