use std::time::Duration;

use crossterm::event::{self, KeyCode, KeyEventKind};
use ratatui::{
    DefaultTerminal, Frame,
    layout::{Constraint, Direction, Layout},
    style::{Color, Style, Stylize},
    text::{Line, Span},
    widgets::{Block, Borders, Padding},
};

use crate::{
    components::{Component, Event, table::TablaProcesos, title::Title},
    core::process::Process,
};

pub struct App {
    pub exit: bool,
    components: Vec<Box<dyn Component>>,
}

impl App {
    pub fn new() -> Self {
        let procesos = vec![
            Process::new(1, 0, 6),
            Process::new(2, 1, 4),
            Process::new(3, 2, 2),
            Process::new(4, 3, 3),
        ];

        Self {
            exit: false,
            components: vec![
                Box::new(Title::new(
                    vec![
                        String::from("SIMULACION DE LA APLICACION"),
                        String::from("DEL ALGORITMO SRTF"),
                    ],
                    Style::default().fg(Color::White),
                )),
                Box::new(TablaProcesos::new(procesos)),
            ],
        }
    }

    pub fn run(&mut self, terminal: &mut DefaultTerminal) -> color_eyre::Result<()> {
        while !self.exit {
            terminal.draw(|frame| self.render_ui(frame))?;
            self.handle_events()?;
        }
        Ok(())
    }

    fn render_ui(&mut self, frame: &mut Frame) {
        let area = frame.area();

        let title = Line::from_iter([
            Span::from("SIMULACIÓN DEL ALGORITMO SRTF").bold(),
            Span::from(" (q: salir)"),
        ]);

        let block = Block::default()
            .title(title)
            .padding(Padding::new(3, 3, 3, 3))
            .borders(Borders::ALL);
        let inner = block.inner(area);
        frame.render_widget(block, area);

        // Un área por componente (vertical)
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Length(4), Constraint::Min(0)])
            .spacing(2)
            .split(inner);

        for (i, component) in self.components.iter_mut().enumerate() {
            if let Some(&chunk) = chunks.get(i) {
                component.render(frame, chunk);
            }
        }
    }

    fn handle_events(&mut self) -> color_eyre::Result<()> {
        if event::poll(Duration::from_millis(16))? {
            let ev = event::read()?;
            if let crossterm::event::Event::Key(key) = ev {
                if key.kind != KeyEventKind::Press {
                    return Ok(());
                }
                // Teclas globales
                if matches!(key.code, KeyCode::Char('q') | KeyCode::Esc) {
                    self.exit = true;
                    return Ok(());
                }
                // Delegar a cada componente
                let event = Event::Key(key);
                for component in &mut self.components {
                    let action = component.handle_events(Some(event.clone()));
                    component.update(action);
                }
            }
        }
        Ok(())
    }
}
