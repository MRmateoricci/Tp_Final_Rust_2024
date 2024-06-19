#![cfg_attr(not(feature = "std"), no_std, no_main)]

#[ink::contract]
mod sistema {
    use ink::prelude::vec::Vec;
    use ink::prelude::string::String;
    

    #[ink::scale_derive(Encode, Decode, TypeInfo)]
    #[cfg_attr(
        feature = "std",
        derive(ink::storage::traits::StorageLayout)
    )]
    #[derive(Debug)]
    pub enum Rol{
        Votante,
        Candidato(String),
    }

    #[ink::scale_derive(Encode, Decode, TypeInfo)]
    #[cfg_attr(
        feature = "std",
        derive(ink::storage::traits::StorageLayout)
    )]
    #[derive(Debug)]
    pub struct Usuario{
        nombre:String,
        apellido:String,
        edad:i32,
        dni:i32,
        verificado:bool,
        rol:Option<Rol>,
        voto:bool,
        acc_id:AccountId
    }
    impl Usuario{

        pub fn new(nombre:String,apellido:String,dni:i32,edad:i32,verificado:bool,rol:Option<Rol>,acc_id:AccountId)->Self{
            Self{nombre,apellido,dni,edad,verificado,rol,voto:false,acc_id}
        }
    }

    /// Defines the storage of your contract.
    /// Add new fields to the below struct in order
    /// to add new static storage fields to your contract.
    #[ink(storage)]
    pub struct Sistema {
        nombre_administrador:String,
        usuarios_reg: Vec<Usuario>,
        votantes: Vec<Usuario>,
        candidatos: Vec<Usuario>,
        admin:AccountId
    }
    

    impl Sistema {
        
        #[ink(constructor)]
        pub fn new(nombre_administrador: String) -> Self {
            Self { nombre_administrador, usuarios_reg:Vec::new(),votantes:Vec::new(),candidatos:Vec::new(), admin: Self::env().caller() }
        }

        #[ink(message)]
        pub fn registrar_usuario(&mut self, nom:String,apellido:String,edad:i32, dni:i32){
            let aux:Usuario = Usuario::new(nom, apellido, dni,edad,false, None, self.env().caller());
            if edad >= 18 {
                if  !self.usuarios_reg.iter().any(|u|u.dni == dni){
                self.usuarios_reg.push(aux);
                }
            }
        }


        #[ink(message)]
        pub fn postularse_como_candidato(&mut self, nombre_de_partido:String){
            let caller = self.env().caller();
            self.usuarios_reg.iter_mut().for_each(|u|{
                if u.acc_id==caller {
                    if !u.verificado{
                        u.rol = Some(Rol::Candidato(nombre_de_partido.clone()));
                        u.verificado=true;
                    }   
                }
            })

            
        }

        #[ink(message)]
        pub fn postularse_como_votante(&self){
            
        }

        #[ink(message)]
        pub fn get_cant_usuarios(&self)->i32{
            let x =self.usuarios_reg.len();
            x as i32
        }

        #[ink(message)]
        pub fn get_id_posicion(&self, pos:i32)->AccountId{
            self.usuarios_reg[pos as usize].acc_id
        }

        
        #[ink(message)]
        pub fn get_owner_id(&self) -> AccountId {
            self.admin
        }
    }

   
}
