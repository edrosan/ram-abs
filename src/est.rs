//* Estructuras

#[derive(Copy, Clone)]
#[derive(Debug)]
pub struct Proceso {
    pub pid: u32,
    pub tam: u8,
    pub extra: u8,
    pub sub: bool,
}

#[derive(Debug)]
pub struct Nodo {
    pub tipo: String,
    pub pos_inicial: u8,
    pub pos_final: u8,
}