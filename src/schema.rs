table! {
    personas (id) {
        id -> Int4,
        id_registro -> Int4,
        num_ident -> Varchar,
        nombre -> Varchar,
        genero -> Varchar,
        estado_civil -> Varchar,
        fecha_nacimiento -> Varchar,
        telefono -> Varchar,
        direccion -> Varchar,
        email -> Varchar,
    }
}

table! {
    registros (id_registro) {
        id_registro -> Int4,
        descripcion -> Varchar,
        fecha -> Varchar,
    }
}

allow_tables_to_appear_in_same_query!(
    personas,
    registros,
);
