CREATE TABLE usuariosss (
  id SERIAL PRIMARY KEY,
  nombre VARCHAR (60) NOT NULL,
  apellido VARCHAR (60)
);

-- Tabla para Usuario, fue eliminaod
CREATE TABLE usuario (
  id SERIAL PRIMARY KEY,
  nickname VARCHAR (60) NOT NULL,
  perfil TEXT,
  username VARCHAR (60) NOT NULL, -- unique, aquí falta este dato para la creación de la página web para el frontend publico
  password_hash VARCHAR (60) NOT NULL,
  email VARCHAR (60) NOT NULL,
  actualizacion TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
  activo BOOLEAN NOT NULL DEFAULT true,
  creado TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);  

CREATE TABLE token_recuperacion (
  id SERIAL PRIMARY KEY,
  user_id INT NOT NULL,
  token VARCHAR NOT NULL,
  tipo BOOLEAN DEFAULT true,
  expira TIMESTAMP NOT NULL,
  CONSTRAINT fk_user FOREIGN KEY (user_id) REFERENCES usuario(id)
);

CREATE TABLE auth_tokens (
  id SERIAL PRIMARY KEY,
  user_id INT NOT NULL,
  token VARCHAR NOT NULL,
  dispositivo VARCHAR,
  expira TIMESTAMP NOT NULL,
  activo BOOLEAN NOT NULL DEFAULT true,
  CONSTRAINT fk_user FOREIGN KEY (user_id) REFERENCES usuario(id)
);

CREATE TABLE sessions (
  id SERIAL PRIMARY KEY,
  user_id INT NOT NULL,
  dispositivo VARCHAR,
  direccion_ip VARCHAR,
  inicio TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
  actualizacion TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
  CONSTRAINT fk_user FOREIGN KEY (user_id) REFERENCES usuario(id)
);

CREATE TABLE multidispositivos (
  id SERIAL PRIMARY KEY,
  user_id INT NOT NULL,
  dispositivo_nombre VARCHAR NOT NULL,
  device_tipo VARCHAR,
  confianza BOOLEAN NOT NULL DEFAULT false,
  CONSTRAINT fk_user FOREIGN KEY (user_id) REFERENCES usuario(id)
);