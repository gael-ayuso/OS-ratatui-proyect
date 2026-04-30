use ratatui::{
    Frame,
    layout::{Constraint, Direction, Layout, Rect},
    style::Color,
};

use crate::{
    components::{Component, grafica_procesos::GraficaProcesos},
    core::process::Process,
};

pub struct GraficaGantt {
    pub procesos: Vec<GraficaProcesos>,
    pub total_time: u32,
}

impl GraficaGantt {
    pub fn new(processes: Vec<Process>) -> Self {
        let mut graficas = Vec::new();
        let mut current_time = 0;
        let colors = [
            Color::Blue,
            Color::Green,
            Color::Yellow,
            Color::Red,
            Color::Magenta,
            Color::Cyan,
        ];

        for (i, p) in processes.iter().enumerate() {
            let color = colors[i % colors.len()];
            let name = format!("P{}", p.num);
            let inicio = current_time;
            let fin = current_time + p.tiempo_rafaga;

            graficas.push(GraficaProcesos::new(name, inicio, fin, color));
            current_time = fin;
        }

        Self {
            procesos: graficas,
            total_time: current_time,
        }
    }
}

impl Component for GraficaGantt {
    fn update(&mut self, action: crate::action::Action) -> crate::action::Action {
        action // Just pass the action through or handle it
    }

    fn render(&mut self, frame: &mut Frame, area: Rect) {
        if self.total_time == 0 || self.procesos.is_empty() {
            return;
        }

        // Creamos un constraint proporcional para cada proceso usando Fill(duración)
        let constraints: Vec<Constraint> = self
            .procesos
            .iter()
            .map(|p| {
                let duration = p.fin_ejecucion - p.inicio_ejecucion;
                // Constraint::Fill(peso) divide el espacio basado en la relación de pesos
                Constraint::Fill(if duration > 0 { duration as u16 } else { 1 })
            })
            .collect();

        let chunks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints(constraints)
            .split(area);

        // Renderizamos cada proceso en su respectivo chunk
        for (i, proceso) in self.procesos.iter_mut().enumerate() {
            proceso.render(frame, chunks[i]);
        }
    }
}
