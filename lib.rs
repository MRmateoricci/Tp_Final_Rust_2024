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
        Candidato,
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
        acc_id:AccountId
    }
    impl Usuario{

        pub fn new(nombre:String,apellido:String,dni:i32,edad:i32,verificado:bool,rol:Option<Rol>,acc_id:AccountId)->Self{
            Self{nombre,apellido,dni,edad,verificado,rol,acc_id}
        }
    }

    #[ink::scale_derive(Encode, Decode, TypeInfo)]
    #[cfg_attr(
        feature = "std",
        derive(ink::storage::traits::StorageLayout)
    )]
    #[derive(Debug)]
    pub struct Fecha{
        pub dia:i32,
        pub mes:i32,
        pub anio:i32
    }


    #[ink::scale_derive(Encode, Decode, TypeInfo)]
    #[cfg_attr(
        feature = "std",
        derive(ink::storage::traits::StorageLayout)
    )]
    #[derive(Debug)]
    struct Votacion{
        id:i32,
        puesto:String,
        candidatos:Vec<AccountId>,
        votantes: Vec<AccountId>,
        votos: Vec<u32>,    // hashmap con accountid de candidato
        votaron: Vec<AccountId>,
        fecha_inicio:Fecha,
        fecha_fin:Fecha,
    }
    impl Votacion{
        pub fn new(id:i32,puesto:String, fecha_inicio:Fecha, fecha_fin:Fecha)-> Votacion{
            Votacion {
                id, puesto, candidatos:Vec::new(),votantes:Vec::new(), votos:Vec::new(), votaron:Vec::new(),fecha_inicio, fecha_fin
            }
        }

        pub fn inicio(&self)->bool{
            true
        }

        pub fn finalizo(&self)->bool{
           true 
        }

        pub fn existe(&self, acc_id:AccountId)->bool{
            return self.candidatos.iter().any(|u| *u == acc_id) | self.votantes.iter().any(|u| *u == acc_id)
        }

    }


    
    #[ink(storage)]
    pub struct Sistema {
        nombre_administrador:String,
        usuarios_reg: Vec<Usuario>, // hashmap con account id
        espera_candidatos:Vec<(AccountId,i32)>,
        espera_votantes:Vec<(AccountId,i32)>,
        votaciones:Vec<Votacion>,  // hashmap con id de votacion
        admin:AccountId,
    }
    

    impl Sistema {
        
        #[ink(constructor)]
        pub fn new(nombre_administrador: String) -> Self {
            Self { nombre_administrador,espera_candidatos:Vec::new(),espera_votantes:Vec::new(), usuarios_reg:Vec::new(),votaciones: Vec::new(), admin: Self::env().caller() }
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
        pub fn crear_votacion(&mut self, id:i32, puesto:String,fecha_inicio:Fecha,fecha_fin:Fecha){
            let caller = self.env().caller();
            if caller == self.admin {
                let v = Votacion::new(id, puesto, fecha_inicio, fecha_fin);
                self.votaciones.push(v);
            }
        }


        #[ink(message)]
        pub fn postularse_a_votacion(&mut self,rol:Rol, id_de_votacion:i32){
            let caller = self.env().caller();
            if let Some(us) = self.usuarios_reg.iter_mut().find(|u| u.acc_id == caller){
                if let Some(v) = self.votaciones.iter_mut().find(|vot| vot.id == id_de_votacion){
                    if !v.existe(caller) {  
                        match rol{// agregar la verificacion para cada uno de que la votacion este vigente como para postularse como votante o que no haya empezado para postularse como candidato 
                            Rol::Candidato=>{ self.espera_candidatos.push((caller,id_de_votacion)); }, 
                            Rol::Votante=> {  self.espera_votantes.push((caller,id_de_votacion)); }
                        }
                    }
                } 
            }
        }


        #[ink(message)] 
        pub fn validar_candidato(&mut self, aceptar:bool)->Option<AccountId>{
            let mut aux: Option<AccountId> = None;
            let caller = self.env().caller();
            if caller == self.admin {
                if !self.espera_candidatos.is_empty() {
                    let acc_id = self.espera_candidatos[0].0;
                    aux = Some(acc_id);
                    self.usuarios_reg.iter_mut().for_each(|u| {
                        if u.acc_id == acc_id {
                            if let Some(vot) = self.votaciones.iter_mut().find(|v| v.id == self.espera_candidatos[0].1){
                                if aceptar{
                                    vot.candidatos.push(acc_id);
                                }
                            }
                            self.espera_candidatos.remove(0);
                        }
                    })
                }
            }
            ink::env::debug_println!("HOLAAAAAAAAAAAAAAAA");
            aux
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
