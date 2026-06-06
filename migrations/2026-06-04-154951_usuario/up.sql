-- Your SQL goes here
CREATE TABLE usuario (
  id SERIAL PRIMARY KEY,
  nickname VARCHAR (60) NOT NULL, -- por defecto aleatorio como reddit o manipulable manualmente
  username VARCHAR (60) NOT NULL UNIQUE, -- unique, aquí falta este dato para la creación de la página web para el frontend publico
  password_hash TEXT NOT NULL,  -- al ser un hash, se debe almacenar en la base de datos como texto 
  email VARCHAR (255) NOT NULL UNIQUE,
  is_active BOOLEAN NOT NULL DEFAULT true,
  updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
  created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE imagen_perfil (
  id SERIAL PRIMARY KEY,
  user_id INT NOT NULL,
  url_image TEXT NOT NULL,  -- url o link de la imagen, cloudflare r2.
  tipo VARCHAR(20) NOT NULL, -- "avatar" o "portada"
  nombre VARCHAR (255) NOT NULL, -- necesario para frontend, alt = nombre.
  -- descripcion TEXT, -- podría ser útil o no.
  is_active BOOLEAN NOT NULL, -- true si la imagen está activa, false si está inactiva
  mime_type VARCHAR (255) NOT NULL, -- image/jpeg, image/png, image/webp, etc.
  tamano_bytes BIGINT NOT NULL, -- tamaño en bytes de la imagen.
  ancho INT NOT NULL, -- pixeles
  alto INT NOT NULL, -- PIXELES
  created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
  updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
  CONSTRAINT fk_user FOREIGN KEY (user_id) REFERENCES usuario(id) ON DELETE CASCADE
);

CREATE TABLE auth_tokens (
  id SERIAL PRIMARY KEY,
  user_id INT NOT NULL,
  token TEXT NOT NULL,
  dispositivo TEXT,
  expira TIMESTAMP NOT NULL,
  is_active BOOLEAN NOT NULL DEFAULT true,
  CONSTRAINT fk_user FOREIGN KEY (user_id) REFERENCES usuario(id) ON DELETE CASCADE
);

CREATE TABLE token_recuperacion ( -- 2026-06-04: falta tabla en models.rs
  id SERIAL PRIMARY KEY,
  user_id INT NOT NULL,
  token TEXT NOT NULL,  
  tipo VARCHAR(20) NOT NULL,
  expira TIMESTAMP NOT NULL,
  CONSTRAINT fk_user FOREIGN KEY (user_id) REFERENCES usuario(id) ON DELETE CASCADE
);
