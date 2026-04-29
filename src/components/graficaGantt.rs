use std::process;

use ratatui::{Frame, layout::Rect};

use crate::{components::Component, core::process::Process};

pub struct GraficaGantt {
    current_time: u32,
    processes: Vec<Process>,
    current_process: Option<Process>,
    area: Rect,
    total_time: u32,
}

impl GraficaGantt {
    pub fn new(processes: Vec<Process>) -> Self {
        let total_time = processes.iter().map(|p| p.tiempo_rafaga).sum();
        Self {
            current_time: 0,
            processes,
            current_process: None,
            area: Rect::default(),
            total_time,
        }
    }

    fn next_step(&mut self) {
        //Obtener el proceso que llego en el tiempo actual
        let next_process = self
            .processes
            .iter()
            .find(|p| p.tiempo_llegada == self.current_time);

        //Obtener el tiempo que falta para que termine el proceso actual
        let time_left = self.current_process.map_or(0, |p| p.tiempo_rafaga);
    }
}

impl Component for GraficaGantt {
    fn update(&mut self, _action: crate::action::Action) -> crate::action::Action {
        todo!()
    }

    fn render(&mut self, frame: &mut Frame, area: Rect) {
        self.area = area;
    }
}
