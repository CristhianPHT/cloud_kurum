CREATE TABLE genero (
    id SERIAL PRIMARY KEY,
    nombre VARCHAR(50) NOT NULL UNIQUE
);
CREATE TABLE libro_genero (
  libro_id INT NOT NULL,
  genero_id INT NOT NULL,
  PRIMARY KEY (libro_id, genero_id),

  CONSTRAINT fk_genero_libro FOREIGN KEY (libro_id) REFERENCES libro(id) ON DELETE CASCADE,
  CONSTRAINT fk_genero_genero FOREIGN KEY (genero_id) REFERENCES genero(id) ON DELETE RESTRICT
  -- Restrict, evita eliminar el género si otro libro aún lo usa
);