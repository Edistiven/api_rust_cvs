use crate::error_handler::CustomError;
use crate::registros::{Persona, Personas};
use crate::registros::{Registro, Registros};
use actix_web::{get, post, web, HttpResponse};
extern crate csv;
//use serde_json::json;

#[get("/api/registros")]
async fn find() -> Result<HttpResponse, CustomError> {
    let registros = Registros::find_registros()?;
    Ok(HttpResponse::Ok().json(registros))
}

#[post("/api/registros")]
async fn create(registro: web::Json<Registro>) -> Result<HttpResponse, CustomError> {
    let registro = Registros::create_registro(registro.into_inner())?;
    Ok(HttpResponse::Ok().json(registro))
}

#[get("/api/personas")]
async fn find_p() -> Result<HttpResponse, CustomError> {
    let personas = Personas::find_personas()?;
    Ok(HttpResponse::Ok().json(personas))
}

#[get("/api/personas/{id}")]
async fn find_personas_by_id(id: web::Path<i32>) -> Result<HttpResponse, CustomError> {
    let persona = Personas::find_by_id(id.into_inner())?;
    Ok(HttpResponse::Ok().json(persona))
}

#[post("/api/personas")]
async fn create_p(persona: web::Json<Persona>) -> Result<HttpResponse, CustomError> {
    let persona = Personas::create_persona(persona.into_inner())?;
    Ok(HttpResponse::Ok().json(persona))
}

#[get("/api/registro/personas/{id}")]
async fn find_registros(id: web::Path<i32>) -> Result<HttpResponse, CustomError> {
    println!("Ya esta adentro :'v");
    let data = DataFrame::read_csv("./info2.csv", true);
    let mut i = 0;
    let _num_ident = data.num_ident;
    let _nombre = data.nombre;
    let _genero = data.genero;
    let _estado_civil = data.estado_civil;
    let _fecha_nacimiento = data.fecha_nacimiento;
    let _telefono = data.telefono;
    let _direccion = data.direccion;
    let _email = data.email;

    let mut _personas: Vec<Persona> = Vec::new();
    
    let l = _num_ident.iter().count();
    //let res2: i32 = l.trim().parse().unwrap();
    println!("{}", l);

    let _id_registro: i32 =id.into_inner();

    while i < l {
        let _persona = Persona {
            id_registro: _id_registro,
            nombre: _nombre[i].to_string().to_uppercase(),
            num_ident: _num_ident[i].to_string(),
            genero: _genero[i].to_string().to_uppercase(),
            estado_civil: _estado_civil[i].to_string().to_uppercase(),
            fecha_nacimiento: _fecha_nacimiento[i].to_string(),
            telefono: _telefono[i].to_string(),
            direccion: _direccion[i].to_string().to_uppercase(),
            email: _email[i].to_string(),
        };

        _personas.push(_persona);

        i = i + 1;

    }
    
    let persona = Personas::create_registro_persona(_personas)?;
    Ok(HttpResponse::Ok().json(persona))
}

#[derive(Debug)]
struct DataFrame {
    num_ident: Vec<String>,
    nombre: Vec<String>,
    genero: Vec<String>,
    estado_civil: Vec<String>,
    fecha_nacimiento: Vec<String>,
    telefono: Vec<String>,
    direccion: Vec<String>,
    email: Vec<String>,
}

impl DataFrame {
    fn new() -> DataFrame {
        DataFrame {
            num_ident: Vec::new(),
            nombre: Vec::new(),
            genero: Vec::new(),
            estado_civil: Vec::new(),
            fecha_nacimiento: Vec::new(),
            telefono: Vec::new(),
            direccion: Vec::new(),
            email: Vec::new(),
        }
    }

    fn read_csv(filepath: &str, has_headers: bool) -> DataFrame {
        // Open file
        let file = std::fs::File::open(filepath).unwrap();
        let mut rdr = csv::ReaderBuilder::new()
            .has_headers(has_headers)
            .from_reader(file);

        let mut data_frame = DataFrame::new();

        // push all the records
        for result in rdr.records().into_iter() {
            let record = result.unwrap();
            data_frame.push(&record);
        }
        return data_frame;
    }

    fn push(&mut self, row: &csv::StringRecord) {
        self.num_ident.push(row[0].to_string());
        self.nombre.push(row[1].to_string());
        self.genero.push(row[2].to_string());
        self.estado_civil.push(row[3].to_string());
        self.fecha_nacimiento.push(row[4].to_string());
        self.telefono.push(row[5].to_string());
        self.direccion.push(row[6].to_string());
        self.email.push(row[7].to_string());
    }
}

pub fn registro_routes(config: &mut web::ServiceConfig) {
    config.service(find);
    config.service(find_p);
    config.service(create);
    config.service(create_p);
    config.service(find_registros);
    config.service(find_personas_by_id);

}
