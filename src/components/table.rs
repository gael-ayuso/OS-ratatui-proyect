use crossterm::event::{KeyCode, KeyEvent};
use ratatui::{
    Frame,
    layout::{Constraint, Rect},
    style::Color,
    widgets::{Block, BorderType, Borders, Padding, Row, Table, TableState},
};

use crate::{action::Action, components::Component, core::process::Process};

pub struct TablaProcesos {
    pub lista: Vec<Process>,
    table_state: TableState,
    area: Rect,
}

impl TablaProcesos {
    pub fn new(lista: Vec<Process>) -> Self {
        let mut table_state = TableState::default();
        table_state.select_first();

        Self {
            lista,
            table_state,
            area: Rect::default(),
        }
    }
}

impl Component for TablaProcesos {
    //Maneja los eventos del teclado
    fn handle_key_events(&mut self, key: KeyEvent) -> Action {
        match key.code {
            KeyCode::Up => Action::SelectPrevious,
            KeyCode::Down => Action::SelectNext,
            KeyCode::Left => Action::SelectPreviousColumn,
            KeyCode::Right => Action::SelectNextColumn,
            _ => Action::Noop,
        }
    }

    fn update(&mut self, action: Action) -> Action {
        match action {
            Action::SelectPrevious => self.table_state.select_previous(),
            Action::SelectNext => self.table_state.select_next(),
            Action::SelectPreviousColumn => self.table_state.select_previous_column(),
            Action::SelectNextColumn => self.table_state.select_next_column(),
            _ => {}
        }
        Action::Noop
    }

    fn render(&mut self, frame: &mut Frame, area: Rect) {
        self.area = area;

        let header = Row::new(["PROCESO", "TIEMPO DE LLEGADA", "TIEMPO DE RAFAGA"])
            .style(ratatui::style::Style::new().bold())
            .bottom_margin(1);

        let rows: Vec<Row> = self
            .lista
            .iter()
            .map(|p| {
                Row::new([
                    p.num.to_string(),
                    p.tiempo_llegada.to_string(),
                    p.tiempo_rafaga.to_string(),
                ])
            })
            .collect();

        //Tamanio de las columnas
        let widths = [
            Constraint::Percentage(20),
            Constraint::Percentage(40),
            Constraint::Percentage(40),
        ];

        let table_block = Block::default()
            .borders(Borders::ALL)
            .title("Procesos")
            .title_style(ratatui::style::Style::new().bold())
            .border_style(ratatui::style::Style::new().bold())
            .border_type(BorderType::Rounded)
            .padding(Padding::uniform(1));

        let table = Table::new(rows, widths)
            .block(table_block)
            .header(header)
            .column_spacing(4)
            .style(Color::White)
            .row_highlight_style(ratatui::style::Style::new().black().on_dark_gray().bold())
            .column_highlight_style(Color::Gray)
            .cell_highlight_style(ratatui::style::Style::new().reversed().light_magenta())
            .highlight_symbol(">>");

        frame.render_stateful_widget(table, self.area, &mut self.table_state);
    }
}
