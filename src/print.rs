//* Mostrar con formato */
use crate::est;



pub fn tam_disponible(lista: &Vec<est::Nodo>){
    for nodo in lista {
        if nodo.tipo == "H" {
            println!("Espacio disponible: {}", nodo.pos_final - nodo.pos_inicial + 1);
        }
    }
}

pub fn mapa(mapa: &Vec<[i32; 8]>) {

    for fila in mapa.iter() {
        for columna in fila.iter() {
            if *columna == 0 {
                print!("\x1b[48;5;28m {} \x1b[0m", columna);
            }else {
                print!("\x1b[48;5;124m {} \x1b[0m", columna);
            }
        }
        println!("");
    }
}

pub fn procesos(tabla: &Vec<est::Proceso>) {
    println!("----Procesos-----------");
    for proceso in tabla.iter() {
        println!("--{}---{}", proceso.pid, !proceso.sub);
    }
    println!("--------------------");
}

pub fn ram_ex(ram: &Vec<i32>, tabla: &Vec<est::Proceso>){
    let mut tam =  0;
    let mut extra = 0;
    let mut cont = 0;
    let mut cont2 = 0;
    let mut pro_actual;

    while cont < ram.len() {
        pro_actual = ram[cont];

        while cont2 < tabla.len(){
            if pro_actual == tabla[cont2].pid as i32 {

                tam = tabla[cont2].tam;
                extra = tabla[cont2].extra;
                cont2 = cont2 + tabla.len();
            }else {
                cont2 = cont2 + 1;
            }
        }
        if tam != 0{
            for i in 0..tam {
                //color rojo
                print!("\x1b[48;5;124m {} \x1b[0m", ram[(cont + i as usize)]);
            }
            //color ambar
            for i in 0..extra {
                print!("\x1b[48;5;136m {} \x1b[0m", ram[(cont + i as usize)]);
            }
            tam = tam - 1;
            cont = cont + (tam + extra) as usize;
        }else{
            //color verde
            print!("\x1b[48;5;28m {} \x1b[0m", ram[cont]);
        }
        tam = 0;
        extra = 0;
        cont2 = 0;
        cont = cont + 1;
    }
    println!("");
}

pub fn lista(lista: &Vec<est::Nodo>) {
    for nodo in lista {
        print!("[{} | I:{} F:{}]-> ", nodo.tipo, nodo.pos_inicial, nodo.pos_final);
    }
    println!("");
}

pub fn mess(proceso: u16, porcentaje: u8) {
    println!("Tamaño del proceso: {}", proceso);
    println!("Porcentaje: {}", porcentaje);
    println!("Tamaño final: {}", porcentaje+proceso as u8);
}