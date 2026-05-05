#[derive(Debug, Clone, Copy)]
pub struct Process {
    pub num: u32,
    pub tiempo_llegada: f32,
    pub tiempo_rafaga: f32,
    pub tiempo_restante: f32,
    pub tiempo_de_espera: f32,
    pub turnaround_time: f32,
    pub tiempo_finalizacion: f32,
}

impl Process {
    pub fn new(num: u32, tiempo_llegada: f32, tiempo_rafaga: f32) -> Process {
        Process {
            num,
            tiempo_llegada,
            tiempo_rafaga,
            tiempo_restante: tiempo_rafaga,
            tiempo_de_espera: 0.0,
            turnaround_time: 0.0,
            tiempo_finalizacion: 0.0,
        }
    }
}
