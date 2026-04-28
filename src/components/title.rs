use ratatui::{
    buffer::Buffer,
    layout::{Alignment, Constraint, Direction, Flex, Layout, Rect},
    style::{Style, Stylize},
    text::Line,
    widgets::Widget,
};
use tui_big_text::{BigText, PixelSize};

pub struct Title {
    pub lines: Vec<String>,
    style: Option<Style>,
}

impl Title {
    pub fn new(lines: Vec<String>) -> Self {
        Self { lines, style: None }
    }
    pub fn new_with_style(lines: Vec<String>, style: Style) -> Self {
        Self {
            lines,
            style: Some(style),
        }
    }
}

impl Widget for Title {
    fn render(self, area: Rect, buf: &mut Buffer) {
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
            // .style(self.style)
            .lines(formatted_lines)
            .alignment(Alignment::Center)
            .build();

        big_title.render(title_area, buf);
    }
}
