
//*clasic */
#[allow(dead_code)]
#[derive(Debug)]
pub struct Nodo {
    pub tipo: String,
    pub pos_inicial: u8,
    pub pos_final: u8,
}
// #[allow(dead_code)]
pub struct proceso {
    nombre: String,
    tam: u8,
    extra: u8,
}


pub fn crear_lista(mem_ram: &[i32]) -> Vec<Nodo> {
    let mut lista_ligada: Vec<Nodo> = Vec::new();

    let mut cont_hueco: u8 = 0;
    let mut cont_proceso: u8 = 0;
    let mut pos_inicial: u8 = 0;
    let mut pos_final: u8;
    let mut proceso_act: i32 = 0;

    for (indice, elemento) in mem_ram.iter().enumerate() {
        if elemento == &0 {
            if cont_proceso != 0 {
                pos_final = (indice as u8) - 1;
                lista_ligada.push(Nodo {
                    tipo: String::from("P"),
                    pos_inicial: pos_inicial,
                    pos_final: pos_final,
                });
                cont_proceso = 0;
            }
            if cont_hueco == 0 {
                pos_inicial = indice as u8;
            }
            cont_hueco = cont_hueco + 1;
        } else if elemento != &0 {
            if cont_hueco != 0 {
                pos_final = (indice as u8) - 1;
                lista_ligada.push(Nodo {
                    tipo: String::from("H"),
                    pos_inicial: pos_inicial,
                    pos_final: pos_final,
                });
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
                lista_ligada.push(Nodo {
                    tipo: String::from("P"),
                    pos_inicial: pos_inicial,
                    pos_final: pos_final,
                });
                proceso_act = *elemento;
                pos_inicial = indice as u8;
                cont_proceso = 1;
            }
        }
    }

    if cont_proceso != 0 {
        pos_final = pos_inicial + cont_proceso - 1;
        lista_ligada.push(Nodo {
            tipo: String::from("P"),
            pos_inicial: pos_inicial,
            pos_final: pos_final,
        });
    }
    if cont_hueco != 0 {
        pos_final = pos_inicial + cont_hueco - 1;
        lista_ligada.push(Nodo {
            tipo: String::from("H"),
            pos_inicial: pos_inicial,
            pos_final: pos_final,
        });
    }

    return lista_ligada;
}
