use crossterm::event::MouseButton;
use ratatui::{
    layout::{Constraint, Direction, Flex, Layout},
    style::Style,
    widgets::{Block, Borders, Cell, Paragraph, Row},
};

use crate::{action::Action, components::Component};

pub struct Buttons {
    label: String,
    is_pressed: bool,
    style: Style,
    pressed_style: Option<Style>,
}

impl Buttons {
    pub fn new(label: String, style: Style) -> Self {
        Self {
            label,
            is_pressed: false,
            style,
            pressed_style: None,
        }
    }
}

impl Component for Buttons {
    fn handle_mouse_events(
        &mut self,
        mouse: crossterm::event::MouseEvent,
    ) -> crate::action::Action {
        match mouse.kind {
            crossterm::event::MouseEventKind::Down(MouseButton::Left) => {
                self.is_pressed = true;
                crate::action::Action::NextStep
            }
            _ => crate::action::Action::Noop,
        }
    }

    fn update(&mut self, action: crate::action::Action) -> crate::action::Action {
        match action {
            Action::NextStep => todo!(),
            _ => todo!(),
        }
    }

    fn render(&mut self, frame: &mut ratatui::Frame, area: ratatui::prelude::Rect) {
        let style = if self.is_pressed {
            Style::default().fg(ratatui::style::Color::Red)
        } else {
            Style::default().fg(ratatui::style::Color::White)
        };

        let button = Paragraph::new(self.label.clone())
            .style(style)
            .block(Block::default().borders(Borders::ALL));

        frame.render_widget(button, area);

        self.is_pressed = false;
    }
}
