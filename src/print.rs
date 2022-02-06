use crate::fnc;

// 'verde' : '\x1b[48;5;28m','\x1b[0m'
pub fn ram(ram: &[i32]) {
    print!("[");
    for (indice, proceso) in ram.iter().enumerate() {
        if proceso == &0 {
            print!("\x1b[48;5;28m {} \x1b[0m", proceso);
        }else{
            print!("\x1b[48;5;124m {} \x1b[0m", proceso);
        }
    }
    println!("]");
}


pub fn lista(lista: Vec<fnc::Nodo>) {
    for nodo in lista {
        print!("[{} | I:{} F:{}]->", nodo.tipo, nodo.pos_inicial, nodo.pos_final);
    }
    println!("");
}