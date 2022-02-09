use std::io;
mod print;
mod fnc;
mod est;
mod edit;

fn main() {
    let mut run: bool = true;
    let mut tam_proceso: u16;
    let mut porcentaje: u8;
    let mut tam_total:u8;
    let mut tabla_proceso:Vec<est::Proceso> = Vec::new();
    let mut pid:u32 = 45;
    const TAM: u8 = 32;
    const N: u8 = TAM / 8;
    let mut opcion_proceso = String::new();

    //* Arrays's */
    let mut ram = vec![0; TAM as usize];
    let mut mapa_bits = vec![[0; 8]; N as usize];
    println!("{:?}", mapa_bits[0][0]);


    let mut lista_ligada = fnc::crear_lista(&ram);


    while run {
        println!("------Menu----------");
        println!("1.Agregar un proceso");
        println!("2.Mostrar los procesos");
        println!("3.Eliminar un proceso");
        println!("4.Modificar tamaño de un proceso");
        println!("5.Salir");
        let mut opcion = String::new();
        io::stdin().read_line(&mut opcion).expect("Ha ocurrido un error");

        match opcion.trim().parse::<i32>().unwrap() {
            1 => {
                println!("--------------------");
                println!("Ingrese el tamaño del proceso");
                io::stdin().read_line(&mut opcion_proceso).expect("Ha ocurrido un error");
                tam_proceso = opcion_proceso.trim().parse::<u16>().unwrap();
                porcentaje = fnc::porcentaje(tam_proceso as u8, 10);
                tam_total = tam_proceso as u8 + porcentaje;
                let respuesta = fnc::comprobar_espacio( &lista_ligada, tam_total);
                if respuesta.0 {
                    print::mess(tam_proceso, porcentaje);
                    tabla_proceso.push(fnc::crear_obj(pid, tam_proceso as u8, porcentaje));
                    for i in 0..(tam_proceso+porcentaje as u16){
                        ram[respuesta.1 as usize  + i as usize] = pid as i32;
                    }
                    pid = pid + 1;
                    mapa_bits = fnc::crear_mapa(&ram, &mapa_bits);
                    lista_ligada = fnc::crear_lista(&ram);
                } else {
                    println!("No hay espacio suficiente");
                }
            }
            2 => {
                println!("--------------------");
                println!("RAM:");
                print::ram_ex(&ram, &tabla_proceso);
                println!("");
                println!("Mapa de bits:");
                print::mapa(&mapa_bits);
                println!("");
                println!("Lista:");
                print::lista( &lista_ligada);
                println!("");
            }
            3 => {
                if tabla_proceso.len() != 0 {
                    println!("--------------------");
                    print::procesos(&tabla_proceso);
                    println!("Ingresa el proceso a eliminar");
                    io::stdin().read_line(&mut opcion_proceso).expect("Ha ocurrido un error");
    
                    println!("\x1b[38;5;136m!!!!Cuidado, va a eliminar el proceso!!!!\x1b[0m");
                    println!("1.Eliminar");
                    println!("2.Cancelar");
                    let mut buf = String::new();
                    io::stdin().read_line(&mut buf).expect("Ha ocurrido un error");
    
                    match buf.trim().parse::<i32>().unwrap() {
                        1 => {
                            let pid_eliminar = opcion_proceso.trim().parse::<i32>().unwrap();
                            ram = edit::eliminar(&ram, pid_eliminar);
                            tabla_proceso = edit::eliminar_tabla(pid_eliminar, &tabla_proceso);
                            lista_ligada = fnc::crear_lista(&ram);
                            mapa_bits = fnc::crear_mapa(&ram, &mapa_bits);
                            println!("\x1b[38;5;28mProceso eliminado\x1b[0m");
                        },
                        2 => println!("Regresando al menu..."),
                        _ => println!("Opcion no valida"),
                    }
                }else {
                    println!("No hay procesos");
                }
            },
            4 => {
                if tabla_proceso.len() != 0 {
                    println!("--------------------");
                    print::procesos(&tabla_proceso);
                    println!("Ingrese el proceso que desea modificar");
                    let mut buf = String::new();
                    io::stdin().read_line(&mut buf).expect("Ha ocurrido un error");
    
                    let proceso_decrecer = buf.trim().parse::<i32>().unwrap();
                    println!("\x1b[38;5;136m!!!!Cuidado, va a reducir el tamaño del proceso!!!!\x1b[0m");
                    println!("1.Decrecer");
                    println!("2.Cancelar");
                    let mut buf = String::new();
                    io::stdin().read_line(&mut buf).expect("Ha ocurrido un error");
    
                    match buf.trim().parse::<i32>().unwrap() {
                        1 => {
                            let (_a, _b) = edit::decrecer(&ram, proceso_decrecer, &tabla_proceso);
                            ram = _a;
                            tabla_proceso = _b;
                            lista_ligada = fnc::crear_lista(&ram);
                            mapa_bits = fnc::crear_mapa(&ram, &mapa_bits);
                        },
                        2 => println!("Regresando a menu...."),
                        _ => println!("Opcion no valida"),
                    }
                } else {
                    println!("No hay procesos");
                }
            },
            5 => run = false,
            _ => println!("Opcion no valida"),
        }
        opcion_proceso=String::from("");
    }
}
