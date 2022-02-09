//*  editan la memoria
use crate::est;

pub fn eliminar(ram: &[i32], pid: i32) -> Vec<i32> {
    let mut mem = ram.to_vec();
    for (indice, _proceso) in ram.iter().enumerate() {
        let a =  ram[indice];
        if a == pid {
            mem[indice] = 0;
        } else{
            mem[indice] = a;
        }
    }
    return mem;
}

pub fn eliminar_tabla(pid: i32, tabla_proceso: &Vec<est::Proceso>) -> Vec<est::Proceso> {
    let mut tabla = vec![];
    for proceso in tabla_proceso.iter() {
        if proceso.pid != pid as u32 {
            tabla.push(*proceso);
        }
    }
    return tabla;
}

pub fn reducir_ram(ram: &Vec<i32>, pid: i32, tam: u8, extra: u8) -> Vec<i32> {
    let mut mem: Vec<i32> = vec![];
    let mut cont:u8 = 0;
    while cont < ram.len() as u8 {
        if ram[cont as usize] == pid {
            for i in 0..tam+extra {
                mem.push(ram[(cont + i) as usize]);
            }
            mem.push(0);
            cont = cont + tam + extra;
        }else {
            mem.push(ram[cont as usize]);
        }
        cont = cont + 1;
    }
    return mem;
}

pub fn decrecer(ram: &Vec<i32>, pid: i32, tabla_proceso: &Vec<est::Proceso>) -> (Vec<i32>, Vec<est::Proceso>) {
    let mut mem = ram.to_vec();
    let mut tabla = tabla_proceso.to_vec();

    for (indice, proceso) in tabla_proceso.iter().enumerate() {
        if proceso.pid == pid as u32 {
            if !proceso.sub {
                tabla[indice].extra = tabla[indice].extra - 1;
                tabla[indice].sub = true;
                mem = reducir_ram(ram, pid, tabla[indice].tam, tabla[indice].extra);
                println!("\x1b[38;5;28mSe redujo el tamaño del proceso.\x1b[0m");
            }else {
                println!("\x1b[38;5;124mNo se puede reducir mas el tamaño del proceso.\x1b[0m");
            }
        }
    }
    return (mem, tabla);
}