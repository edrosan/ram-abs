use std::io;

mod print;
mod fnc;




fn main() {
    let mut run: bool = true;
    let mut opcion = String::new();
    let mut opcion_proceso = String::new();
    let mut tam_proceso: u16;

    const TAM: u8 = 16;
    const N: u8 = TAM / 8;

    //* Arrays's */
    let mut ram = [0; TAM as usize];
    let  mapa_bits = [[0; 8]; N as usize];

    ram[5] = 1;
    let  lista_ligada = fnc::crear_lista(&ram);
    println!("{:?}", lista_ligada);

    print::ram(&ram);
    print::lista( lista_ligada);
    // println!("{:?}", lista_ligada);



    for proceso in mapa_bits.iter() {
        println!(" {:?}", proceso);
    }

    while run {
        println!("Menu");
        println!("1.Agregar Proceso");
        println!("2.Mostrar Proceso");
        println!("3.Salir");

        io::stdin()
            .read_line(&mut opcion)
            .expect("Ha ocurrido un error");

        match opcion.trim().parse::<i32>().unwrap() {
            1 => {
                println!("Ingrese el tamaño del proceso");
                io::stdin()
                    .read_line(&mut opcion_proceso)
                    .expect("Ha ocurrido un error");
                tam_proceso = opcion_proceso.trim().parse::<u16>().unwrap();
                println!("{}", tam_proceso);
            }
            3 => run = false,
            _ => println!("Opcion no valida"),
        }

        opcion = String::from("");
    }

    // print!("[");
    // for proceso in ram.iter() {
    //     print!(" {}", proceso);
    // }
    // print!(" ]");

    // print!("{:?}", ram);
}
