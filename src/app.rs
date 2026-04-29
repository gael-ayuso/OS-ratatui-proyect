use std::time::Duration;

use crossterm::event::{self, KeyCode, KeyEventKind};
use ratatui::{
    DefaultTerminal, Frame,
    layout::{Constraint, Direction, Layout},
    style::{Style, Stylize},
    text::{Line, Span},
    widgets::{Block, Borders, Padding},
};

use crate::{
    action::Action,
    components::{Component, Event, buttons::Buttons, table::TablaProcesos, title::Title},
    core::process::Process,
};

pub struct App {
    pub exit: bool,

    //Declaracion de los componentes que se van a usar en la aplicacion
    table: TablaProcesos, //Tabla de procesos
    btn_next: Buttons,    //Boton Next
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
            table: TablaProcesos::new(procesos),
            btn_next: Buttons::new(String::from("Like si ves esto"), Style::default(), Action::NextStep),
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
            .constraints([
                Constraint::Length(6), // Título
                Constraint::Min(0),    // Tabla de Procesos
                // Constraint::Length(10), // Grafica de Gantt
                Constraint::Length(3), // Botón
            ])
            .spacing(2)
            .split(inner);

        //Titulo "Simulacion del Algoritmo SRTF"
        let title = Title::new(vec![
            String::from("SIMULACION DE LA APLICACION"),
            String::from("DEL ALGORITMO SRTF"),
        ]);
        //Agregar Titulo al frame en la primera chunk
        frame.render_widget(title, chunks[0]);

        //Tabla de procesos
        //Crear un layout para que la tabla se centre
        let table_layout = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([
                Constraint::Fill(1),
                Constraint::Ratio(1, 3),
                Constraint::Fill(1),
            ])
            .split(chunks[1]);

        self.table.render(frame, table_layout[1]);

        //Botón next
        //Crear el layout para el botón next
        let button_layout = Layout::default()
            .direction(Direction::Horizontal)
            //Los constrainsts definen el tamaño de las columnas
            .constraints([
                Constraint::Fill(1),    //Espacio vacio a la izquierda del boton
                Constraint::Length(20), //Tamaño del boton
                Constraint::Fill(1),    //Espacio vacio a la derecha del boton
            ])
            .split(chunks[2]);

        //Agregar botón next al frame en el ultimo chunk
        self.btn_next.render(frame, button_layout[1]);
    }

    fn handle_events(&mut self) -> color_eyre::Result<()> {
        if event::poll(Duration::from_millis(16))? {
            let ev = event::read()?;
            match ev {
                //Evento de teclado
                crossterm::event::Event::Key(key) => {
                    if key.kind != KeyEventKind::Press {
                        return Ok(());
                    }
                    // Teclas globales
                    if matches!(key.code, KeyCode::Char('q') | KeyCode::Esc) {
                        self.exit = true;
                        return Ok(());
                    }
                    // Delegar a cada componente explícitamente
                    let event = Event::Key(key);
                    let act_table = self.table.handle_events(Some(event.clone()));
                    self.table.update(act_table);

                    let act_btn = self.btn_next.handle_events(Some(event.clone()));
                    self.btn_next.update(act_btn);
                }
                //Evento de ratón
                crossterm::event::Event::Mouse(mouse) => {
                    // Delegar el evento de ratón a cada componente explícitamente
                    let event = Event::Mouse(mouse);

                    let act_table = self.table.handle_events(Some(event.clone()));
                    self.table.update(act_table);

                    let act_btn = self.btn_next.handle_events(Some(event.clone()));
                    self.btn_next.update(act_btn);
                }
                _ => {}
            }
        }
        Ok(())
    }
}
