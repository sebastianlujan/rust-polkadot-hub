#[derive(Debug)]
pub struct Persona {
    apellidos : String,
    nombres: String,
    edad: u32
}

fn main() {
    let persona = Persona { apellidos : String::from("VALERA"),
        nombres : String::from("RAMON"),
        edad : 30_u32
    };
   println!("{:?}",persona);    
}