use crossterm::event::{KeyCode, KeyEvent};
use ratatui::{
    Frame,
    layout::{Constraint, Rect},
    style::{Color, Style},
    widgets::{Block, BorderType, Borders, Padding, Row, Table, TableState},
};

use crate::{action::Action, components::Component, core::process::Process};

#[derive(PartialEq)]
//Enum para definir el modo de edicion de la tabla
pub enum InputMode {
    Normal,
    Editing,
}

pub struct TablaProcesos {
    pub lista: Vec<Process>,
    table_state: TableState,
    area: Rect,
    actual_row: usize,
    actual_colum: usize,
    mode: InputMode,
    input: String,
}

impl TablaProcesos {
    pub fn new(lista: Vec<Process>) -> Self {
        let mut table_state = TableState::default();
        table_state.select_first();

        Self {
            lista,
            table_state,
            area: Rect::default(),
            actual_row: 0,
            actual_colum: 0,
            mode: InputMode::Normal,
            input: String::new(),
        }
    }

    fn apply_edit(&mut self) -> Action {
        //Si la tabla esta vacia
        if self.lista.is_empty() {
            return Action::Noop;
        }
        //Obtiene el valor del input
        let input_val = self.input.trim();

        //Actualiza el valor de la columna correspondiente
        match self.actual_colum {
            0 => {
                if let Ok(num) = input_val.parse::<u32>() {
                    //Comprueba si el numero ya existe en la tabla
                    let existing = self.lista.iter().any(|p| p.num == num);
                    if existing {
                        let next_num = self.lista.iter().map(|p| p.num).max().unwrap_or(0) + 1;
                        self.lista[self.actual_row].num = next_num;
                    } else {
                        self.lista[self.actual_row].num = num;
                    }
                }
            }
            1 => {
                if let Ok(llegada) = input_val.parse::<f32>() {
                    self.lista[self.actual_row].tiempo_llegada = llegada;
                }
            }
            2 => {
                if let Ok(rafaga) = input_val.parse::<f32>() {
                    self.lista[self.actual_row].tiempo_rafaga = rafaga;
                    self.lista[self.actual_row].tiempo_restante = rafaga;
                }
            }
            _ => {}
        }
        Action::Edit
    }

    fn add(&mut self) -> Action {
        let max_num = self.lista.iter().map(|p| p.num).max().unwrap_or(0);
        let new_process = Process::new(max_num + 1, 0.0, 1.0);
        self.lista.push(new_process);
        Action::Add
    }

    fn delete(&mut self) -> Action {
        if !self.lista.is_empty() && self.actual_row < self.lista.len() {
            self.lista.remove(self.actual_row);
            if self.actual_row >= self.lista.len() && self.actual_row > 0 {
                self.actual_row -= 1;
            }
        }
        Action::Delete
    }

    fn save(&mut self) -> Action {
        Action::Save
    }
}

impl Component for TablaProcesos {
    fn handle_key_events(&mut self, key: KeyEvent) -> Action {
        match self.mode {
            InputMode::Normal => match key.code {
                KeyCode::Up => Action::SelectPrevious,
                KeyCode::Down => Action::SelectNext,
                KeyCode::Left => Action::SelectPreviousColumn,
                KeyCode::Right => Action::SelectNextColumn,
                KeyCode::Enter => {
                    if !self.lista.is_empty() {
                        self.mode = InputMode::Editing;
                        let p = &self.lista[self.actual_row];
                        //Pasa el valor de la columna a editar al input
                        self.input = match self.actual_colum {
                            0 => p.num.to_string(),
                            1 => p.tiempo_llegada.to_string(),
                            2 => p.tiempo_rafaga.to_string(),
                            _ => return Action::Noop,
                        };
                    }
                    Action::Noop
                }
                KeyCode::Char('a') | KeyCode::Char('A') => self.add(),
                KeyCode::Char('d') | KeyCode::Char('D') => self.delete(),
                KeyCode::Char('s') | KeyCode::Char('S') => self.save(),
                _ => Action::Noop,
            },
            InputMode::Editing => match key.code {
                KeyCode::Enter => {
                    //Aplica la edicion
                    let action = self.apply_edit();
                    self.mode = InputMode::Normal;
                    self.input.clear();
                    action
                }
                KeyCode::Char(c) => {
                    //Agrega el caracter al input
                    self.input.push(c);
                    Action::Noop
                }
                KeyCode::Backspace => {
                    //Elimina el ultimo caracter del input
                    self.input.pop();
                    Action::Noop
                }
                KeyCode::Esc => {
                    //Sale del modo edicion
                    self.mode = InputMode::Normal;
                    self.input.clear();
                    Action::Noop
                }
                _ => Action::Noop,
            },
        }
    }

    fn update(&mut self, action: Action) -> Action {
        match action {
            Action::SelectPrevious => {
                if self.actual_row > 0 {
                    self.actual_row -= 1;
                }
                self.table_state.select_previous()
            }
            Action::SelectNext => {
                if self.actual_row < self.lista.len().saturating_sub(1) {
                    self.actual_row += 1;
                }
                self.table_state.select_next()
            }
            Action::SelectPreviousColumn => {
                if self.actual_colum > 0 {
                    self.actual_colum -= 1;
                }
                self.table_state.select_previous_column()
            }
            Action::SelectNextColumn => {
                if self.actual_colum < 2 {
                    self.actual_colum += 1;
                }
                self.table_state.select_next_column()
            }
            _ => {}
        }
        Action::Noop
    }

    fn render(&mut self, frame: &mut Frame, area: Rect) {
        self.area = area;
        self.table_state.select(Some(self.actual_row));
        self.table_state.select_column(Some(self.actual_colum));

        let title_text = if self.mode == InputMode::Editing {
            "Procesos [MODO EDICION] (Enter: Confirmar, Esc: Cancelar)"
        } else {
            "Procesos [a: Anadir, d: Eliminar, s: Guardar, Enter: Editar]"
        };

        let header = Row::new(["PROCESO", "TIEMPO DE LLEGADA", "TIEMPO DE RAFAGA"])
            .style(Style::new().bold())
            .bottom_margin(1);

        let rows: Vec<Row> = self
            .lista
            .iter()
            .enumerate()
            .map(|(i, p)| {
                let is_editing = self.mode == InputMode::Editing && i == self.actual_row;

                let num_str = if is_editing && self.actual_colum == 0 {
                    format!("{}|", self.input)
                } else {
                    p.num.to_string()
                };

                let llegada_str = if is_editing && self.actual_colum == 1 {
                    format!("{}|", self.input)
                } else {
                    p.tiempo_llegada.to_string()
                };

                let rafaga_str = if is_editing && self.actual_colum == 2 {
                    format!("{}|", self.input)
                } else {
                    p.tiempo_rafaga.to_string()
                };

                Row::new([num_str, llegada_str, rafaga_str])
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
            .title(title_text)
            .title_style(Style::new().bold())
            .border_style(Style::new().bold())
            .border_type(BorderType::Rounded)
            .padding(Padding::uniform(1));

        let table = Table::new(rows, widths)
            .block(table_block)
            .header(header)
            .column_spacing(4)
            .style(Color::White)
            .row_highlight_style(Style::new().black().on_dark_gray().bold())
            .column_highlight_style(Color::Gray)
            .cell_highlight_style(Style::new().reversed().light_magenta())
            .highlight_symbol(">>");

        frame.render_stateful_widget(table, self.area, &mut self.table_state);
    }
}
