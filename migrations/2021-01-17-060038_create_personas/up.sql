-- Your SQL goes here
CREATE TABLE "registros" (
    id_registro SERIAL PRIMARY KEY NOT NULL,
    descripcion VARCHAR NOT NULL,
    fecha VARCHAR NOT NULL
);

CREATE TABLE "personas" (
    id SERIAL PRIMARY KEY,
    id_registro INT NOT NULL,
    num_ident VARCHAR NOT NULL,
    nombre VARCHAR NOT NULL,
    genero VARCHAR NOT NULL,
    estado_civil VARCHAR NOT NULL,
    fecha_nacimiento VARCHAR NOT NULL,
    telefono VARCHAR NOT NULL,
    direccion VARCHAR NOT NULL,
    email VARCHAR NOT NULL
);
