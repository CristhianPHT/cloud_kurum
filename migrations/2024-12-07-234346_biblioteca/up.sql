
-- Tabla Libro
CREATE TABLE libro (
    id SERIAL PRIMARY KEY,
    titulo VARCHAR(100),
    perfil TEXT, --text (link nosql)
    sinopsis TEXT,
    tipo VARCHAR(100),
    capitulos VARCHAR(100),
    publicacion DATE NOT NULL,
    estado VARCHAR(100)
);

-- Tabla capitulos
CREATE TABLE capitulos (
    id SERIAL PRIMARY KEY,
    nombre VARCHAR,
    link TEXT,
    imagen TEXT,
    fk_libro INT REFERENCES libro(id)
);

CREATE TABLE autor (
    id SERIAL PRIMARY KEY,
    fk_libro INT REFERENCES libro(id),
    nombre VARCHAR(100),
    apellido VARCHAR(100),
    perfil VARCHAR
);
-- Tabla Genero
CREATE TABLE genero (
    id SERIAL PRIMARY KEY,
    nombre VARCHAR(50),
    descripcion VARCHAR
);
-- Tabla Scan
CREATE TABLE scan (
    id SERIAL PRIMARY KEY,
    nombre VARCHAR,
    portada TEXT,
    perfil TEXT,
    descripcion VARCHAR,
    redsocial VARCHAR,
    creacion TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);
-- Tabla LibroGenero
CREATE TABLE libro_genero (
    id SERIAL PRIMARY KEY,
    libro_id INT REFERENCES libro(id),
    genero_id INT REFERENCES genero(id)
);

-- Tabla Puntaje
CREATE TABLE puntaje (
    id SERIAL PRIMARY KEY,
    usuario_id INT REFERENCES usuario(id),
    libro_id INT REFERENCES libro(id),
    calificacion FLOAT,
    fecha_calificacion TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);

-- Tabla Nombres
CREATE TABLE nombres (
    id SERIAL PRIMARY KEY,
    libro_id INT REFERENCES libro(id),
    nombre VARCHAR(100),
    descripcion TEXT
);

-- Tabla Publicador
CREATE TABLE publicador (
    id SERIAL PRIMARY KEY,
    scan_id INT REFERENCES scan(id),
    autor_id INT REFERENCES usuario(id),
    libro_id INT REFERENCES capitulos(id)
);

-- Tabla Etiqueta
CREATE TABLE etiqueta (
    id SERIAL PRIMARY KEY,
    imagen VARCHAR,
    descripcion VARCHAR,
    visibilidad BOOLEAN,
    color VARCHAR,
    fk_usuario INT REFERENCES usuario(id)
);

-- Tabla Etiqueta_list
CREATE TABLE etiqueta_list (
    id SERIAL PRIMARY KEY,
    fk_etiqueta INT REFERENCES etiqueta(id),
    fk_libro INT REFERENCES libro(id)
);


-- Tabla Miembros
CREATE TABLE miembros (
    id SERIAL PRIMARY KEY,
    fk_scan INT REFERENCES scan(id),
    fk_usuario INT REFERENCES usuario(id)
);


-- Tabla marcapaginas
CREATE TABLE marcapaginas (
    id SERIAL PRIMARY KEY,
    fk_usuario INT REFERENCES usuario(id),
    posicion FLOAT,
    nota TEXT,
    etiqueta VARCHAR(50),
    creado TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    actualizado TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    fk_capitulo INT REFERENCES capitulos(id)
);
