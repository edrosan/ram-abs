use crate::est;



pub fn crear_obj(pid: u32, tam: u8, extra: u8) -> est::Proceso {
    est::Proceso{pid:pid, tam:tam, extra:extra, sub:false}
}

pub fn porcentaje(cantidad: u8, porcentaje: u8) -> u8 {
    let mut extra: f64 = (cantidad as f64 / 100.0) * porcentaje as f64;
    extra = extra.ceil();
    return extra as u8;
}

pub fn comprobar_espacio(lista: &Vec<est::Nodo>, tam_proceso: u8) -> (bool, u8) {
    let mut espacio_disponible:u8;
    for nodo in lista {
        espacio_disponible = (nodo.pos_final - nodo.pos_inicial) + 1;
        if (nodo.tipo == "H") & (tam_proceso <= espacio_disponible) {
            return (true, nodo.pos_inicial);
        }        
    }
    return (false, 0);
}

pub fn crear_lista(mem_ram: &[i32]) -> Vec<est::Nodo> {
    let mut lista_ligada: Vec<est::Nodo> = Vec::new();
    let mut cont_hueco: u8 = 0;
    let mut cont_proceso: u8 = 0;
    let mut pos_inicial: u8 = 0;
    let mut pos_final: u8;
    let mut proceso_act: i32 = 0;

    for (indice, elemento) in mem_ram.iter().enumerate() {
        if elemento == &0 {
            if cont_proceso != 0 {
                pos_final = (indice as u8) - 1;
                lista_ligada.push(est::Nodo { tipo: String::from("P"), pos_inicial: pos_inicial, pos_final: pos_final});
                cont_proceso = 0;
            }
            if cont_hueco == 0 {
                pos_inicial = indice as u8;
            }
            cont_hueco = cont_hueco + 1;
        } else if elemento != &0 {
            if cont_hueco != 0 {
                pos_final = (indice as u8) - 1;
                lista_ligada.push(est::Nodo { tipo: String::from("H"), pos_inicial: pos_inicial, pos_final: pos_final});
                cont_hueco = 0;
            }
            if cont_proceso == 0 {
                proceso_act = *elemento;
                pos_inicial = indice as u8;
                cont_proceso = cont_proceso + 1;
            } else if elemento == &proceso_act {
                cont_proceso = cont_proceso + 1;
            } else if elemento != &proceso_act {
                pos_final = (indice as u8) - 1;
                lista_ligada.push(est::Nodo {tipo: String::from("P"), pos_inicial: pos_inicial, pos_final: pos_final});
                proceso_act = *elemento;
                pos_inicial = indice as u8;
                cont_proceso = 1;
            }
        }
    }

    if cont_proceso != 0 {
        pos_final = pos_inicial + cont_proceso - 1;
        lista_ligada.push(est::Nodo {tipo: String::from("P"), pos_inicial: pos_inicial, pos_final: pos_final});
    }
    if cont_hueco != 0 {
        pos_final = pos_inicial + cont_hueco - 1;
        lista_ligada.push(est::Nodo {tipo: String::from("H"), pos_inicial: pos_inicial, pos_final: pos_final});
    }
    
    return lista_ligada;
}

pub fn crear_mapa(ram: &[i32], mapa: &Vec<[i32; 8]>) -> Vec<[i32; 8]> {
    let mut mapa_bits = mapa.to_vec();

    for (indice, proceso) in ram.iter().enumerate() {
        let fila = indice as i32 / 8;
        let columna = indice as i32 % 8;
        if *proceso == 0 {
            mapa_bits[fila as usize][columna as usize] = 0;
        }else {
            mapa_bits[fila as usize][columna as usize] = 1;
        }
    }
    
    return mapa_bits;
}