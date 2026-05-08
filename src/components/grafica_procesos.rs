use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Style},
    widgets::{Block, Borders, Paragraph},
};

use crate::components::Component;

pub struct GraficaProcesos {
    pub name: String,
    pub inicio_ejecucion: f32,
    pub fin_ejecucion: f32,
    pub tiempo_restante: f32,
    pub color: Color,
    pub area: Rect,
}

impl GraficaProcesos {
    pub fn new(name: String, inicio_ejecucion: f32, fin_ejecucion: f32, color: Color) -> Self {
        Self {
            name,
            inicio_ejecucion,
            fin_ejecucion,
            tiempo_restante: 0.0,
            color,
            area: Rect::default(),
        }
    }
}

impl Component for GraficaProcesos {
    fn render(&mut self, frame: &mut ratatui::Frame, area: Rect) {
        self.area = area;

        // Dividir el área en dos: la barra superior y la etiqueta inferior para el tiempo
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Min(3),    // Espacio para la barra
                Constraint::Length(1), // Espacio para el tiempo
            ])
            .split(self.area);

        // El texto del proceso
        let text = if self.name.is_empty() {
            "P?".to_string()
        } else {
            self.name.clone()
        };

        // Crear el bloque de la gráfica de Gantt (barra llena con color)
        let bar = Paragraph::new(text)
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .border_style(Style::default().bg(Color::Reset)),
            )
            // Color de fondo para la barra
            .style(Style::default().bg(self.color).fg(Color::White))
            .alignment(Alignment::Center);

        frame.render_widget(bar, chunks[0]);

        // Crear la etiqueta del tick final (alineada a la derecha para que quede bajo el borde derecho)
        let tick = Paragraph::new(self.fin_ejecucion.to_string()).alignment(Alignment::Right);

        frame.render_widget(tick, chunks[1]);
    }
}
