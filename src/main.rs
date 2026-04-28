mod action;
mod app;
pub mod components;
pub mod core;

use crate::app::App;

fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;
    crossterm::execute!(std::io::stdout(), crossterm::event::EnableMouseCapture)?;
    let mut terminal = ratatui::init();

    let app_result = App::new().run(&mut terminal);

    ratatui::restore();
    crossterm::execute!(std::io::stdout(), crossterm::event::DisableMouseCapture)?;
    app_result
}
