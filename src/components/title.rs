use ratatui::{
    Frame,
    layout::{Alignment, Constraint, Direction, Flex, Layout, Rect},
    style::{Style, Stylize},
    text::Line,
};
use tui_big_text::{BigText, PixelSize};

use crate::components::Component;

pub struct Title {
    pub lines: Vec<String>,
    style: Style,
}

impl Title {
    pub fn new(lines: Vec<String>, style: Style) -> Self {
        Self { lines, style }
    }
}

impl Component for Title {
    fn render(&mut self, frame: &mut Frame, area: Rect) {
        let centered_layout = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Length(6)])
            .flex(Flex::Center)
            .split(area);

        let title_area = centered_layout[0];

        let formatted_lines: Vec<Line> = self
            .lines
            .iter()
            .map(|l| Line::from(l.clone()).white())
            .collect();

        let big_title = BigText::builder()
            .pixel_size(PixelSize::Octant)
            .style(self.style)
            .lines(formatted_lines)
            .alignment(Alignment::Center)
            .build();

        frame.render_widget(big_title, title_area);
    }
}
