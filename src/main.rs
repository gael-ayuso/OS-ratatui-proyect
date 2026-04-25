mod app;
pub mod components;
pub mod core;

use std::{error::Error, io, time::Duration};

use crossterm::{
    event::{self, Event, KeyCode},
    execute,
    terminal::{EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode},
};
use ratatui::{Terminal, backend::CrosstermBackend};

use app::App;

fn main() -> Result<(), Box<dyn Error>> {
    // Setup the terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // Create the app state
    let mut app = App::new();

    // Run the application
    let res = run_app(&mut terminal, &mut app);

    // Restore the terminal
    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen,)?;
    terminal.show_cursor()?;

    // Handle any potential errors during execution
    if let Err(err) = res {
        println!("{:?}", err)
    }

    Ok(())
}

fn run_app(terminal: &mut Terminal<CrosstermBackend<io::Stdout>>, app: &mut App) -> io::Result<()> {
    loop {
        // Render the UI
        terminal.draw(|f| app.ui(f))?;

        // Handle events
        if event::poll(Duration::from_millis(250))? {
            if let Event::Key(key) = event::read()? {
                match key.code {
                    KeyCode::Char('q') => app.quit(),
                    _ => {}
                }
            }
        } else {
            // Tick for the application state updates (e.g., animations, running algorithms)
            app.tick();
        }

        // Exit if quit was requested
        if app.exit {
            return Ok(());
        }
    }
}
