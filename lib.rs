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


        pub fn get_cant_candidatos_vot(&self)->i32{
            let x = self.candidatos.len();
            x as i32
        }

        pub fn get_cant_votantes_vot(&self)->i32{
            let x = self.votantes.len();
            x as i32
        }
        
        pub fn inicio(&self)->bool{  // trabajar con fechas
            true
        }

        pub fn finalizo(&self)->bool{ // trabajar con fechas
           true 
        }



        pub fn es_votante(&self, acc_id:AccountId)->bool{
            return self.votantes.iter().any(|u| *u == acc_id)
        }

        pub fn es_candidato(&self, acc_id:AccountId)->bool{
            return self.candidatos.iter().any(|u| *u == acc_id)
        }

        pub fn sumar_candidato(&mut self,accid:AccountId){
            self.candidatos.push(accid);
            self.votos.push(0);
        }

        pub fn sumar_votante(&mut self,accid:AccountId){
            self.votantes.push(accid);
        }

        pub fn sumar_voto(&mut self,pos:usize){
            self.votos[pos] = self.votos[pos].wrapping_add(1);
        }

        pub fn ver_votos(&self,pos:i32)->u32{
            self.votos[pos as usize]
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
            let caller = self.env().caller();
            if caller != self.admin{  //el administrador no se puede registrar como un usuario 
                let aux:Usuario = Usuario::new(nom, apellido, dni,edad,false, None, caller);
                if edad >= 18 {
                    if  !self.usuarios_reg.iter().any(|u|u.dni == dni){  // no puede haber dos usuarios con el mismo dni 
                        self.usuarios_reg.push(aux);
                    }
                }
            }
        }

        #[ink(message)]
        pub fn crear_votacion(&mut self, id:i32, puesto:String,fecha_inicio:Fecha,fecha_fin:Fecha){ 
            let caller = self.env().caller();
            if caller == self.admin {  //solo el administrador puede crear votaciones
                if !self.votaciones.iter().any(|v|v.id==id){  //no se tiene que poder crear dos votaciones con el mismo id
                    let v = Votacion::new(id, puesto, fecha_inicio, fecha_fin);
                    self.votaciones.push(v);        
                }
            }
                
        }


        #[ink(message)]
        pub fn postularse_a_votacion(&mut self,rol:Rol, id_de_votacion:i32){
            let caller = self.env().caller();
                if self.usuarios_reg.iter().any(|u| u.acc_id == caller){   // como el administrador no puede registrarse, si se intenta postular aca va a dar falso
                    if let Some(v) = self.votaciones.iter_mut().find(|vot| vot.id == id_de_votacion){  //si existe la votacion a la que se quiere postular 
                        if !v.es_votante(caller) && !v.es_candidato(caller){ // si ya no esta postulado como votante o candidato
                            match rol{ // AGREGAR la verificacion para cada uno de que la votacion este vigente (fechas) como para postularse como votante o que no haya empezado para postularse como candidato 
                                Rol::Candidato=>{ self.espera_candidatos.push((caller,id_de_votacion)); }, 
                                Rol::Votante=> {  self.espera_votantes.push((caller,id_de_votacion)); }
                            }
                        }
                    } 
                }
        }


        #[ink(message)] 
        pub fn validar_candidato(&mut self, aceptar:bool){
            let mut aux: Option<AccountId> = None;
            let mut vot_id=0;
            let caller = self.env().caller();
            if caller == self.admin {  // solo el administrador puede validar candidatos 
                if !self.espera_candidatos.is_empty() {  // checkea si hay candidatos a validar, y si hay se empieza a trabajar el primero
                    let acc_id = self.espera_candidatos[0].0;
                    vot_id = self.espera_candidatos[0].1;
                    aux = Some(acc_id);
                    self.usuarios_reg.iter_mut().for_each(|u| { //si ya estas en la espera de candidatos, si o si vas a estar en el vector de usuarios 
                        if u.acc_id == acc_id {
                            if let Some(vot) = self.votaciones.iter_mut().find(|v| v.id == vot_id){  // va a encontrar la votacion si o si ya que esto se checkea al postularse
                                if aceptar{  // el admin decide si aceptar o rechazar el candidato
                                    vot.sumar_candidato(acc_id);
                                }
                            }
                            self.espera_candidatos.remove(0);  // se elimina de la cola de espera de aprobacion 
                        }
                    })
                }
            }
            if let Some(a) =aux{
                ink::env::debug_println!("Aceptar solicitud de candidato del usuario con id {:?} para la votacion de id {}",a,vot_id);

            } else{
                ink::env::debug_println!("No hay solicitudes para candidatos");

            }
        }

        #[ink(message)] 
        pub fn validar_votante(&mut self, aceptar:bool){
            let mut aux: Option<AccountId> = None;
            let mut vot_id=0;
            let caller = self.env().caller();
            if caller == self.admin {
                if !self.espera_votantes.is_empty() {
                    let acc_id = self.espera_votantes[0].0;
                    vot_id = self.espera_votantes[0].1;
                    aux = Some(acc_id);
                    self.usuarios_reg.iter_mut().for_each(|u| { //si ya estas en la espera de candidatos, si o si vas a estar en el vector de usuarios 
                        if u.acc_id == acc_id {
                            if let Some(vot) = self.votaciones.iter_mut().find(|v| v.id == vot_id){
                                if aceptar{
                                    vot.sumar_votante(acc_id);
                                }
                            }
                            self.espera_votantes.remove(0);
                        }
                    })
                }
            }
            if let Some(a) =aux{
                ink::env::debug_println!("Aceptar solicitud de votante del usuario con id {:?} para la votacion de id {}",a,vot_id);

            } else{
                ink::env::debug_println!("No hay solicitudes para votante");

            }
        }


        #[ink(message)]
        pub fn votar(&mut self,id:i32,opcion:i32){
            let caller = self.env().caller();
            let mut x: i32  = 0;
            if caller != self.admin{
                if self.usuarios_reg.iter().any(|u| u.acc_id == caller){
                    if let Some(v) = self.votaciones.iter_mut().find(|vot| vot.id == id){
                        if v.es_votante(caller){
                            ink::env::debug_println!("Candidatos");
                            v.candidatos.iter().for_each(|c|{
                                x = x.wrapping_add(1);
                                if let Some(us) =self.usuarios_reg.iter().find(|u|u.acc_id==*c){  //siempre va a entrar ya que si esta como candidato en la votacion si o si esta registrado 
                                    ink::env::debug_println!("Opcion {}: {} {}",x,us.nombre,us.apellido);
                                }
                            });

                            if let Some(op) = opcion.checked_sub(1) {
                                if op as usize <= v.candidatos.len() {
                                    v.sumar_voto(op as usize);
                                }
                            }
                            
                        }
                    }
                }
            }

        }

        #[ink(message)]
        pub fn ver_votos(&mut self,id:i32){
            
            
            let mut x: i32=0;
            if let Some(v) = self.votaciones.iter_mut().find(|vot| vot.id == id){
                    ink::env::debug_println!("Candidatos y sus votos actuales");
                    v.candidatos.iter().for_each(|c|{
                        x = x.wrapping_add(1);
                        if let Some(us) =self.usuarios_reg.iter().find(|u|u.acc_id==*c){  //siempre va a entrar ya que si esta como candidato en la votacion si o si esta registrado 
                            if let Some(op) = x.checked_sub(1) {
                                    ink::env::debug_println!("Candidato nro {}: {} {}, cant votos: {}",x,us.nombre,us.apellido,v.ver_votos(op));

                            }
                        }
                    });
            }
        }


        #[ink(message)]
        pub fn get_cant_usuarios(&self)->i32{
            let x =self.usuarios_reg.len();
            x as i32
        }

        #[ink(message)]
        pub fn get_cant_espera_candidatos(&self)->i32{
            let x =self.espera_candidatos.len();
            x as i32
        }

        #[ink(message)]
        pub fn get_cant_espera_votantes(&self)->i32{
            let x =self.espera_votantes.len();
            x as i32
        }

        #[ink(message)]
        pub fn get_cant_candidatos_vot(&self,id:i32)->i32{
            if let Some(vot) =self.votaciones.iter().find(|v| v.id == id){
                return vot.get_cant_candidatos_vot();
            }
            0
        }

        #[ink(message)]
        pub fn get_cant_votantes_vot(&self,id:i32)->i32{
            if let Some(vot) =self.votaciones.iter().find(|v| v.id == id){
                return vot.get_cant_votantes_vot();
            }
            0
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
