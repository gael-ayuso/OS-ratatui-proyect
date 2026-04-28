use crossterm::event::MouseButton;
use ratatui::{
    layout::{Alignment, Constraint, Direction, Flex, Layout, Rect},
    style::Style,
    widgets::{Block, Borders, Cell, Paragraph, Row},
};

use crate::{
    action::{self, Action},
    components::Component,
};

pub struct Buttons {
    label: String,
    is_pressed: bool,
    style: Style,
    #[allow(unused)]
    pressed_style: Option<Style>,
    pub action: Action,
    area: Rect,
}

impl Buttons {
    pub fn new(label: String, style: Style, action: Action) -> Self {
        Self {
            label,
            is_pressed: false,
            style,
            pressed_style: None,
            action,
            area: Rect::default(),
        }
    }
}

impl Component for Buttons {
    fn handle_mouse_events(
        &mut self,
        mouse: crossterm::event::MouseEvent,
    ) -> crate::action::Action {
        let is_inside = self.area.left() <= mouse.column
            && mouse.column < self.area.right()
            && self.area.top() <= mouse.row
            && mouse.row < self.area.bottom();

        if !is_inside {
            return crate::action::Action::Noop;
        }

        match mouse.kind {
            crossterm::event::MouseEventKind::Down(MouseButton::Left) => {
                self.is_pressed = true;
                Action::NextStep
            }
            crossterm::event::MouseEventKind::Up(MouseButton::Left) => {
                self.is_pressed = false;
                crate::action::Action::Noop
            }
            _ => crate::action::Action::Noop,
        }
    }

    fn update(&mut self, action: crate::action::Action) -> crate::action::Action {
        match action {
            Action::NextStep => self.action.clone(),
            _ => crate::action::Action::Noop,
        }
    }

    fn render(&mut self, frame: &mut ratatui::Frame, area: ratatui::prelude::Rect) {
        self.area = area;

        let style = if self.is_pressed {
            self.style.add_modifier(ratatui::style::Modifier::REVERSED)
        } else {
            self.style
        };

        let button = Paragraph::new(self.label.clone())
            .style(style)
            .alignment(Alignment::Center)
            .block(Block::default().borders(Borders::ALL));

        frame.render_widget(button, self.area);
    }
}
