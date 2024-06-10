use crate::usuario::*;
use crate::usuario;


pub struct sistema{
    usuarios:Vec<usuario::Rol>,
    votantes:Vec<Votante>,
    candidatos:Vec<Candidato>,
    admin:Administrador

}
impl sistema{
    fn registro_usuario(&mut self,u:usuario::Rol){
        if self.admin.verificar(u){

        }
    }
}