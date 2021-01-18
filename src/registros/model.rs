use crate::db;
use crate::error_handler::CustomError;
use crate::schema::registros;
use crate::schema::personas;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

//Model Registro
#[derive(Serialize, Deserialize, AsChangeset, Insertable)]
#[table_name = "registros"]
pub struct Registro {   
    pub descripcion :String,
    pub fecha :String,
}

#[derive(Serialize, Deserialize, Queryable, Insertable)]
#[table_name = "registros"]
pub struct Registros {
    pub id_registro: i32,
    pub descripcion:String,
    pub fecha :String,
}

//Model Persona

#[derive(Serialize, Deserialize, AsChangeset, Insertable)]
#[table_name = "personas"]
pub struct Persona {   
    pub id_registro : i32,
    pub num_ident :String,
    pub nombre :String,
    pub genero :String,
    pub estado_civil :String,
    pub fecha_nacimiento :String,
    pub telefono :String,
    pub direccion :String,
    pub email :String,
}

#[derive(Serialize, Deserialize, Queryable, Insertable)]
#[table_name = "personas"]
pub struct Personas {
    pub id: i32,
    pub id_registro : i32,
    pub num_ident :String,
    pub nombre :String,
    pub genero :String,
    pub estado_civil :String,
    pub fecha_nacimiento :String,
    pub telefono :String,
    pub direccion :String,
    pub email :String,
}

impl Registros {
    pub fn find_registros() -> Result<Vec<Self>, CustomError> {
        let conn = db::connection()?;
        let registros = registros::table.load::<Registros>(&conn)?;
        Ok(registros)
    }

    pub fn create_registro(registro: Registro) -> Result<Self, CustomError> {
        let conn = db::connection()?;
        let registro = Registro::from(registro);
        let registro = diesel::insert_into(registros::table)
            .values(registro)
            .get_result(&conn)?;
        Ok(registro)
    }

}

impl Personas {
   
    pub fn find_personas() -> Result<Vec<Self>, CustomError> {
        let conn = db::connection()?;
        let personas = personas::table.load::<Personas>(&conn)?;
        Ok(personas)
    }

    pub fn create_persona(persona: Persona) -> Result<Self, CustomError> {
        let conn = db::connection()?;
        let persona = Persona::from(persona);
        let persona = diesel::insert_into(personas::table)
            .values(persona)
            .get_result(&conn)?;
        Ok(persona)
    }

    pub fn create_registro_persona(personas: Vec<Persona>) -> Result<Self, CustomError> {
        let conn = db::connection()?;
        let persona = diesel::insert_into(personas::table)
            .values(personas)
            .get_result(&conn)?;
        Ok(persona)
    }

    pub fn find_by_id(id: i32) -> Result<Vec<Self>, CustomError> {
        let conn = db::connection()?;
        let personas = personas::table.filter(personas::id_registro.eq(id))
        .load::<Personas>(&conn)?;
        Ok(personas)
    }
}

impl Registro {
    fn from(registro: Registro) -> Registro {
        Registro {
            descripcion:registro.descripcion,
            fecha :registro.fecha,

        }
    }
}

impl Persona {
    fn from(persona: Persona) -> Persona {
        Persona {
            id_registro : persona.id_registro,
            num_ident :persona.num_ident,
            nombre :persona.nombre,
            genero :persona.genero,
            estado_civil :persona.estado_civil,
            fecha_nacimiento :persona.fecha_nacimiento,
            telefono :persona.telefono,
            direccion :persona.direccion,
            email :persona.email,

        }
    }
}

