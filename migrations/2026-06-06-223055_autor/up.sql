CREATE TABLE autor (
  id SERIAL PRIMARY KEY,
  nombre VARCHAR(255) NOT NULL
);
CREATE TABLE libro_autor (
  libro_id INT NOT NULL,
  autor_id INT NOT NULL,
  
  PRIMARY KEY (libro_id, autor_id),
  CONSTRAINT fk_libro_autor_libro FOREIGN KEY (libro_id) REFERENCES libro(id) ON DELETE CASCADE,
  CONSTRAINT fk_libro_autor_autor FOREIGN KEY (autor_id) REFERENCES autor(id) ON DELETE RESTRICT
);
-- aquí podría ser necesario image_url, galería de imágenes, nombre, apellido