#![cfg_attr(not(feature = "std"), no_std, no_main)]

#[ink::contract]
mod sistema {
    use ink::prelude::vec::Vec;
    use ink::prelude::string::String;
    use ink::prelude::collections::BTreeMap;


    #[ink::scale_derive(Encode, Decode, TypeInfo)]
    #[cfg_attr(
        feature = "std",
        derive(ink::storage::traits::StorageLayout)
    )]
    #[derive(Debug,Clone)]
    pub enum Rol{
        Votante,
        Candidato,
    }

    #[ink::scale_derive(Encode, Decode, TypeInfo)]
    #[cfg_attr(
        feature = "std",
        derive(ink::storage::traits::StorageLayout)
    )]
    #[derive(Debug,Clone)]
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
        pub dia:u32,
        pub mes:u32,
        pub anio:i32
    }
    impl Fecha {

        fn es_fecha_valida(&self)->bool{
            if (self.mes>0)&&(self.mes<=12)&&(self.dia>0)&&(self.dia<=31)&&(self.anio>=1970){
                match self.mes {
                    2=> {
                        if self.is_leap_year(self.anio){
                            return self.dia<=29
                        }
                        else{
                            return self.dia<=28
                        }
                    }
                    1..=12=> {
                        return self.dia<= self.days_in_month(self.mes)
                    }
                    _=> {}
                }
            }
            return false;
        }

        pub fn to_timestamp(&self) -> Timestamp {
            let days_since_epoch = self.days_since_epoch() as i64;
            let millis_per_day: i64 = 24 * 60 * 60 * 1000;
            days_since_epoch.wrapping_mul(millis_per_day)  as Timestamp
        }
    
        fn days_since_epoch(&self) -> u32 {
            let mut days: u32 = 0;
    
            // Calcular los días desde el Epoch hasta el año actual
            for year in 1970..self.anio {
                days = days.wrapping_add(if self.is_leap_year(year) { 366 } else { 365 });
            }
    
            // Sumar los días de los meses previos en el año actual
            for month in 1..self.mes {
                days = days.wrapping_add(self.days_in_month(month));
            }
    
            // Sumar los días del mes actual
            days = days.wrapping_add(self.dia);
    
            days
        }
    
        fn is_leap_year(&self, year: i32) -> bool {
            (year % 4 == 0 && year % 100 != 0) || (year % 400 == 0)
        }
    
        fn days_in_month(&self, month: u32) -> u32 {
            match month {
                1 | 3 | 5 | 7 | 8 | 10 | 12 => 31,

                4 | 6 | 9 | 11 => 30,

                2 => {
                    if self.is_leap_year(self.anio) {
                        29
                    } else {
                        28
                    }
                },

                _ => panic!("MES INVALIDO"),
            }
        }
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
        votos: BTreeMap<AccountId,u32>,    // hashmap con accountid de candidato
        votaron: Vec<AccountId>,
        fecha_inicio:Timestamp,
        fecha_fin:Timestamp,
    }
    impl Votacion{
        pub fn new(id:i32,puesto:String, fecha_inicio:Timestamp, fecha_fin:Timestamp)-> Votacion{
            Votacion {
                id, puesto, candidatos:Vec::new(),votantes:Vec::new(), votos:BTreeMap::new(), votaron:Vec::new(),fecha_inicio, fecha_fin
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
        
        pub fn inicio(&self, momento:Timestamp)->bool{  // trabajar con fechas
            return momento > self.fecha_inicio
        }

        pub fn finalizo(&self, momento:Timestamp)->bool{ // trabajar con fechas
           return momento > self.fecha_fin
        }



        pub fn es_votante(&self, acc_id:AccountId)->bool{
            return self.votantes.iter().any(|u| *u == acc_id)
        }

        pub fn es_candidato(&self, acc_id:AccountId)->bool{
            return self.candidatos.iter().any(|u| *u == acc_id)
        }

        pub fn sumar_candidato(&mut self,accid:AccountId){
            self.candidatos.push(accid);
            self.votos.insert(accid, 0);
        }

        pub fn sumar_votante(&mut self,accid:AccountId){
            self.votantes.push(accid);
        }

        pub fn sumar_voto(&mut self,pos:usize){
            self.votos.entry(self.candidatos[pos]).and_modify(|c|* c = c.wrapping_add(1));
        }

        pub fn ver_votos(&self,pos:i32)->u32{
            if let Some(x)=self.votos.get(&self.candidatos[pos as usize]){
                return *x
            }
            0
        }


    }


    
    #[ink(storage)]
    pub struct Sistema {
        nombre_administrador:String,
        espera_usuarios: Vec<Usuario>,
        usuarios_reg: Vec<Usuario>, // hashmap con account id
        espera_candidatos:Vec<(AccountId,i32)>,
        espera_votantes:Vec<(AccountId,i32)>,
        votaciones:Vec<Votacion>,  // hashmap con id de votacion
        admin:AccountId,
    }
    

    impl Sistema {
        
        #[ink(constructor)]
        pub fn new(nombre_administrador: String) -> Self {
            Self { nombre_administrador,espera_usuarios:Vec::new(),espera_candidatos:Vec::new(),espera_votantes:Vec::new(), usuarios_reg:Vec::new(),votaciones: Vec::new(), admin: Self::env().caller() }
        }



        #[ink(message)]
        pub fn registrar_usuario(&mut self, nom:String,apellido:String,edad:i32, dni:i32){
            let caller = self.env().caller();
            if caller != self.admin{  //el administrador no se puede registrar como un usuario 
                let aux:Usuario = Usuario::new(nom, apellido, dni,edad,false, None, caller);
                if edad >= 18 {
                    if  !self.usuarios_reg.iter().any(|u| u.dni == dni || u.acc_id == caller){  // no puede haber dos usuarios con el mismo dni 
                        self.espera_usuarios.push(aux);
                    }
                }
            }
        }

        #[ink(message)] 
        pub fn validar_usuario(&mut self, aceptar:bool){
            let mut aux: Option<String> = None;
            let caller = self.env().caller();
            if caller == self.admin {  // solo el administrador puede validar candidatos 
                if !self.espera_usuarios.is_empty() {  // checkea si hay candidatos a validar, y si hay se empieza a trabajar el primero
                    let us = self.espera_usuarios[0].clone();
                    let mut s1 = us.nombre.clone();
                    let s2 = String::from(" ");
                    let s3 = us.apellido.clone();
                    s1.push_str(&s2);
                    s1.push_str(&s3);
                    aux = Some(s1);   
                    if aceptar{  // el admin decide si aceptar o rechazar el candidato
                        self.usuarios_reg.push(us)
                    }
                    self.espera_usuarios.remove(0);  // se elimina de la cola de espera de aprobacion 
                }
            }
            if let Some(a) =aux{
                ink::env::debug_println!("Aceptar solicitud de registro del usuario  {:?}",a);
            } else{
                ink::env::debug_println!("No hay solicitudes de registro");
            }
        }

        #[ink(message)]
        pub fn crear_votacion(&mut self, id:i32, puesto:String,fecha_inicio:Fecha,fecha_fin:Fecha){ 
            let caller = self.env().caller();
            if !fecha_inicio.es_fecha_valida() | !fecha_fin.es_fecha_valida(){
                panic!("FECHA INVALIDA");
            }
            if caller == self.admin {  //solo el administrador puede crear votaciones
                if !self.votaciones.iter().any(|v|v.id==id){  //no se tiene que poder crear dos votaciones con el mismo id
                    let v = Votacion::new(id, puesto, fecha_inicio.to_timestamp(),fecha_fin.to_timestamp());
                    self.votaciones.push(v);       
                    ink::env::debug_println!("fecha inicio: {:?} timestamp: {}",fecha_inicio,fecha_inicio.to_timestamp().wrapping_sub(86_400_000)); //asi comienza ese dia a las 00:00
                    ink::env::debug_println!("fecha fin: {:?} timestamp: {}",fecha_fin,fecha_fin.to_timestamp().wrapping_sub(1));  //asi termina ese dia a las 23:59:59.999
                }

            }
                
        }


        #[ink(message)]
        pub fn postularse_a_votacion(&mut self,rol:Rol, id_de_votacion:i32){
            let caller = self.env().caller();
            let momento = self.env().block_timestamp();
                if self.usuarios_reg.iter().any(|u| u.acc_id == caller){   // como el administrador no puede registrarse, si se intenta postular aca va a dar falso
                    if let Some(v) = self.votaciones.iter_mut().find(|vot| vot.id == id_de_votacion){  //si existe la votacion a la que se quiere postular 
                        if v.inicio(momento){
                            panic!("LA VOTACION YA INICIO timestamp vot:{}",v.fecha_inicio);
                        }
                        if !v.es_votante(caller) && !v.es_candidato(caller){ // si ya no esta postulado como votante o candidato
                            match rol{ // AGREGAR la verificacion para cada uno de que la votacion este vigente (fechas) como para postularse como votante o que no haya empezado para postularse como candidato 
                                Rol::Candidato=>{ self.espera_candidatos.push((caller,id_de_votacion)); }, 
                                Rol::Votante=> {  self.espera_votantes.push((caller,id_de_votacion)); }
                            }
                        }
                    }else{
                        panic!("NO EXISTE VOTACION DE ID: {}",id_de_votacion);
                    } 
                }
            ink::env::debug_println!("timestamp actual: {}",momento);

        }


        #[ink(message)] 
        pub fn validar_candidato(&mut self, aceptar:bool){
            let mut aux: Option<String> = None;
            let mut vot_id=0;
            let caller = self.env().caller();
            let momento = self.env().block_timestamp();
            if caller == self.admin {  // solo el administrador puede validar candidatos 
                if !self.espera_candidatos.is_empty() {  // checkea si hay candidatos a validar, y si hay se empieza a trabajar el primero
                    let acc_id = self.espera_candidatos[0].0;
                    vot_id = self.espera_candidatos[0].1;
                    
                    self.usuarios_reg.iter_mut().for_each(|u| { //si ya estas en la espera de candidatos, si o si vas a estar en el vector de usuarios 
                        if u.acc_id == acc_id {
                            let mut s1 = u.nombre.clone();
                            let s2 = String::from(" ");
                            let s3 = u.apellido.clone();
                            s1.push_str(&s2);
                            s1.push_str(&s3);
                            aux = Some(s1); 
                            if let Some(vot) = self.votaciones.iter_mut().find(|v| v.id == vot_id){  // va a encontrar la votacion si o si ya que esto se checkea al postularse
                                if vot.inicio(momento){
                                    panic!("LA VOTACION YA INICIO");
                                }
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
                ink::env::debug_println!("Aceptar solicitud de candidato del usuario {:?} para la votacion de id {}",a,vot_id);

            } else{
                ink::env::debug_println!("No hay solicitudes para candidatos");

            }
        }

        #[ink(message)] 
        pub fn validar_votante(&mut self, aceptar:bool){
            let mut aux: Option<String> = None;
            let mut vot_id=0;
            let caller = self.env().caller();
            let momento = self.env().block_timestamp();
            if caller == self.admin {
                if !self.espera_votantes.is_empty() {
                    let acc_id = self.espera_votantes[0].0;
                    vot_id = self.espera_votantes[0].1;
                    
                    self.usuarios_reg.iter_mut().for_each(|u| { //si ya estas en la espera de candidatos, si o si vas a estar en el vector de usuarios 
                        if u.acc_id == acc_id {
                            let mut s1 = u.nombre.clone();
                            let s2 = String::from(" ");
                            let s3 = u.apellido.clone();
                            s1.push_str(&s2);
                            s1.push_str(&s3);
                            aux = Some(s1); 
                            if let Some(vot) = self.votaciones.iter_mut().find(|v| v.id == vot_id){
                                if vot.inicio(momento){
                                    panic!("LA VOTACION YA INICIO");
                                }
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
                ink::env::debug_println!("Aceptar solicitud de votante del usuario {:?} para la votacion de id {}",a,vot_id);

            } else{
                ink::env::debug_println!("No hay solicitudes para votante");

            }
        }


        #[ink(message)]
        pub fn votar(&mut self,id_de_votacion:i32,opcion:i32){
            let caller = self.env().caller();
            let mut x: i32  = 0;
            let momento = self.env().block_timestamp();
            if caller != self.admin{
                if self.usuarios_reg.iter().any(|u| u.acc_id == caller){
                    if let Some(v) = self.votaciones.iter_mut().find(|vot| vot.id == id_de_votacion){
                        if !v.inicio(momento){
                            panic!("LA VOTACION TODAVIA NO INICIO");
                        }
                        if v.finalizo(momento){
                            panic!("LA VOTACION FINALIZO");
                        }
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
                    }else{
                        panic!("NO EXISTE VOTACION DE ID: {}",id_de_votacion);
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
