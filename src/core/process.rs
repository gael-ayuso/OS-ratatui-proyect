#[derive(Debug, Clone, Copy)]
pub struct Process {
    pub num: u32,
    pub tiempo_llegada: u32,
    pub tiempo_rafaga: u32,
    pub tiempo_restante: u32,
}

impl Process {
    pub fn new(num: u32, tiempo_llegada: u32, tiempo_rafaga: u32) -> Process {
        Process {
            num,
            tiempo_llegada,
            tiempo_rafaga,
            tiempo_restante: tiempo_rafaga,
        }
    }
}
