mod action;
mod app;
pub mod components;
pub mod core;

use crate::app::App;

fn main() -> color_eyre::Result<()> {
    //Instala el manejador de errores
    color_eyre::install()?;
    //Habilita la captura de eventos del mouse
    crossterm::execute!(std::io::stdout(), crossterm::event::EnableMouseCapture)?;
    //Inicializa el terminal
    let mut terminal = ratatui::init();

    //Crea la aplicacion y ejecuta el bucle principal
    let app_result = App::new().run(&mut terminal);

    ratatui::restore();
    crossterm::execute!(std::io::stdout(), crossterm::event::DisableMouseCapture)?;
    app_result
}
