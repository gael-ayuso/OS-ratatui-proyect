use ratatui::{
    Frame,
    layout::{Constraint, Direction, Layout, Rect},
    style::Color,
};

use crate::{
    action::Action,
    components::{Component, grafica_procesos::GraficaProcesos},
    core::{process::Process, srtf_Algorithm::Srtf},
};

pub struct GraficaGantt {
    pub procesos: Vec<GraficaProcesos>,
    pub lista_de_procesos: Vec<Process>,
    pub total_time: f32,
    pub srtf_instance: Srtf,
}

const COLORS: [Color; 6] = [
    Color::Blue,
    Color::Green,
    Color::Yellow,
    Color::Red,
    Color::Magenta,
    Color::Cyan,
];

impl GraficaGantt {
    pub fn new(processes: Vec<Process>) -> Self {
        let graficas = Vec::new();

        let srtf_instance = Srtf::new(processes.clone());

        Self {
            procesos: graficas,
            lista_de_procesos: processes,
            total_time: 0.0,
            srtf_instance,
        }
    }

    //Cuando se hace click en NextStep, se ejecuta el siguiente proceso
    fn update_processes(&mut self) {
        //Si no quedan procesos por ejecutar, termina
        if self.srtf_instance.procesos_completados >= self.srtf_instance.num_processes {
            return;
        }

        //Obtiene el siguiente proceso a ejecutar
        let next_process = self.srtf_instance.execute_one_process();
        if next_process.num > 0 {
            let final_process_time = self.total_time + next_process.time_last_execution;
            let inicio = self.total_time;

            //Guarda el proceso y actualiza el tiempo total
            self.procesos.push(GraficaProcesos::new(
                format!("P{}", next_process.num),
                inicio,
                final_process_time,
                COLORS[((next_process.num - 1) as usize) % COLORS.len()],
            ));
            self.total_time = final_process_time;
        } else {
            //Tiempos muertos de la CPU
            self.procesos.push(GraficaProcesos::new(
                "Espera".to_string(),
                self.total_time,
                self.total_time + next_process.tiempo_rafaga,
                Color::Black,
            ));
            self.total_time += next_process.tiempo_rafaga;
        }
    }
}

impl Component for GraficaGantt {
    fn update(&mut self, action: Action) -> Action {
        match action {
            Action::NextStep => {
                //Actualiza los procesos despues de un tick
                self.update_processes();
                Action::Noop
            }
            //Resetear la grafica de gantt
            Action::Reset => {
                self.procesos.clear();
                self.total_time = 0.0;
                Action::Noop
            }
            _ => Action::Noop,
        }
    }

    fn render(&mut self, frame: &mut Frame, area: Rect) {
        let block = ratatui::widgets::Block::default()
            .title("Gráfica de Gantt")
            .borders(ratatui::widgets::Borders::ALL);

        let inner_area = block.inner(area);
        frame.render_widget(block, area);

        if self.total_time == 0.0 || self.procesos.is_empty() {
            return;
        }

        // Creamos un constraint proporcional para cada proceso usando Fill(duración)
        let constraints: Vec<Constraint> = self
            .procesos
            .iter()
            .map(|p| {
                let duration = p.fin_ejecucion - p.inicio_ejecucion;
                // Constraint::Fill(peso) divide el espacio basado en la relación de pesos
                Constraint::Fill(if duration > 0.0 { duration as u16 } else { 1 })
            })
            .collect();

        let chunks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints(constraints)
            .split(inner_area);

        // Renderizamos cada proceso en su respectivo chunk
        for (i, proceso) in self.procesos.iter_mut().enumerate() {
            proceso.render(frame, chunks[i]);
        }
    }
}
