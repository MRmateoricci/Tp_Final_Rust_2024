
#[derive(Debug,PartialEq,Clone)]
pub struct Fecha{
    pub dia:i32,
    pub mes:i32,
    pub anio:i32
}

const DIAS_MES: [i32 ; 13]= [0,31,28,31,30,31,30,31,30,31,30,31,30];

impl Fecha{
    pub fn new(dia:i32, mes:i32, anio:i32)->Fecha{
        return Fecha{
            dia,
            mes,
            anio
        }
    }
    pub fn eq(&self, f:&Fecha)->bool{
        return self.dia == f.dia && self.mes == f.mes && self.anio == f.anio;
    }

    fn es_fecha_valida(&self)->bool{
        if (self.mes>0)&&(self.mes<=12)&&(self.dia>0)&&(self.dia<=31)&&(self.anio>=1000){
            match self.mes {
                2=> {
                    if self.es_bisiesto(){
                        return self.dia<=29
                    }
                    else{
                        return self.dia<=28
                    }
                }
                1..=12=> {
                    return self.dia<= DIAS_MES[self.mes as usize]
                }
                _=> {}
            }
        }
        return false;
    }

    fn es_bisiesto(&self)->bool{
        return self.anio%4==0;
    }

    pub fn sumar_dias(&mut self,dias:i32){
        self.dia+=dias;
        while !self.es_fecha_valida(){
            match self.mes {
                2=>{
                    if self.es_bisiesto(){
                        if self.dia > 29 {
                            self.mes+=1;
                            self.dia-=29;
                        }
                    }
                    else{
                        if self.dia>DIAS_MES[self.mes as usize] {
                            self.dia-=DIAS_MES[self.mes as usize];
                            self.mes+=1;
                        }
                    }
                }
                12=>{
                    if self.dia >30{
                        self.anio+=1;
                        self.mes=0;
                        self.dia-=30;
                    }
                }
                1..=11=>{
                    if self.dia>DIAS_MES[self.mes as usize] {
                        self.dia-=DIAS_MES[self.mes as usize];
                        self.mes+=1;
                        }
                }
                _=>{}
            }         

        }
    }

    fn restar_dias(&mut self,dias:i32){
        self.dia-=dias;
        while !self.es_fecha_valida(){
            match self.mes {
                3=>{
                    if self.es_bisiesto(){
                        if self.dia<=0 {
                            self.mes-=1;
                            self.dia+=29;
                        }
                    }
                    else{
                        if self.dia<=0 {
                            self.dia+=DIAS_MES[self.mes as usize-1];
                            self.mes-=1;
                        }
                    }
                }
                1=>{
                    if self.dia <=0{
                        self.anio-=1;
                        self.mes=12;
                        self.dia+=30;
                    }
                }
                2..=12=>{
                    if self.dia<=0 {
                        self.dia+=DIAS_MES[self.mes as usize-1];
                        self.mes-=1;
                    }   
                }
                _=>{}
            }         
        }
    }

    pub fn es_mayor(&self,f:&Fecha)->bool{
        if self.anio>f.anio{
            return true;
        }
        else{
            if self.anio==f.anio && self.mes>f.mes {
                return true;
            }
            else{
                if (self.anio==f.anio)&&(self.mes==f.mes)&&(self.dia>f.dia){
                    return true;
                }
                else{
                    return false;
                }
            }
        }
    }

}

#[test]

fn test_ej3_1(){
    let mut f:Fecha=Fecha::new(1,1,2024);
    f.sumar_dias(59);
    let aux:Fecha=Fecha::new(29,2,2024);
    assert!(f.eq(&aux));
}

#[test]

fn test_ej3_2(){
    let mut f:Fecha=Fecha::new(1,3,2024);
    f.restar_dias(61);
    let aux:Fecha=Fecha::new(30,12,2023);
    assert!(f.eq(&aux));
}

#[test]
fn test_ej3_3(){
    let f:Fecha=Fecha::new(1,1,2024);
    assert!(f.es_bisiesto())
}

#[test]
fn test_ej3_4(){
    let f:Fecha=Fecha::new(1,1,2023);
    assert!(!f.es_bisiesto())
}

#[test]
fn test_ej3_5(){
    let y:Fecha=Fecha::new(31,12,2023);
    let f:Fecha=Fecha::new(1,1,2024);
    assert!(f.es_mayor(&y))
}
