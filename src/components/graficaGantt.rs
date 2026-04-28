use crate::core::process::Process;

pub struct GraficaGantt {
    pub current_time: usize,
    pub processes: Vec<Process>,
    pub current_process: Option<Process>,
}

impl GraficaGantt {
    pub fn new() -> Self {
        Self {
            current_time: 0,
            processes: Vec::new(),
            current_process: None,
        }
    }
}
