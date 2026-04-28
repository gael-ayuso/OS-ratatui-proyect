mod app;
mod action;
pub mod components;
pub mod core;

use crate::app::App;

fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;
    let mut terminal = ratatui::init();
    let app_result = App::new().run(&mut terminal);
    ratatui::restore();
    app_result
}
