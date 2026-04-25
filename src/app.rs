use ratatui::{
    Frame,
    layout::{Constraint, Direction, Layout},
    style::{Color, Style},
    widgets::{Block, Borders, Paragraph},
};

pub struct App {
    pub exit: bool,
}

impl App {
    pub fn new() -> Self {
        Self { exit: false }
    }

    /// Update function called repeatedly (e.g. for algorithms or animations)
    pub fn tick(&mut self) {
        // Put logic here
    }

    pub fn quit(&mut self) {
        self.exit = true;
    }

    /// Render the UI on the frame
    pub fn ui(&self, frame: &mut Frame) {
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .margin(1)
            .constraints(
                [
                    Constraint::Length(3), // Header
                    Constraint::Min(0),    // Main body
                    Constraint::Length(3), // Footer
                ]
                .as_ref(),
            )
            .split(frame.area());

        // Header
        let title_block = Block::default()
            .borders(Borders::ALL)
            .style(Style::default().fg(Color::Cyan));
        let title = Paragraph::new("OS Simulator - Ratatui").block(title_block);
        frame.render_widget(title, chunks[0]);

        // Main body
        let main_block = Block::default().title("Main Content").borders(Borders::ALL);
        frame.render_widget(main_block, chunks[1]);

        // Footer
        let footer_block = Block::default().borders(Borders::ALL);
        let footer = Paragraph::new("Press 'q' to quit.").block(footer_block);
        frame.render_widget(footer, chunks[2]);
    }
}
