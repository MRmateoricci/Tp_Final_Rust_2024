use crate::fecha::Fecha;
use crate::sistema::sistema;
pub enum Rol{
    Votante(Votante),
    Candidato(Candidato),
    Administrador(Administrador)
}

pub struct Votante{
    nombre:String,
    id:u8,
    aprobado:bool
}

pub struct Candidato{
    nombre:String,
    id:u8,
    partido:String,

}

pub struct Administrador{
    fecha_inicio:Fecha,
    fecha_cierre:Fecha,
}
impl Administrador{
    pub fn verificar (&self, u:Rol) ->bool{
        
        true
    }
}



