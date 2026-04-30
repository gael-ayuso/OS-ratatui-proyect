use color_eyre::eyre::Result;
use crossterm::event::{KeyEvent, MouseEvent};
use ratatui::{Frame, layout::Rect};

use crate::action::Action;

/// Evento unificado que puede recibir cualquier componente.
#[derive(Clone, Debug)]
pub enum Event {
    Key(KeyEvent),
    Mouse(MouseEvent),
    Resize(u16, u16),
    Tick,
    Quit,
}

/// Trait principal de componente.
/// Cada componente encapsula su propio estado, eventos y renderizado.
pub trait Component {
    /// Inicialización única al arrancar (opcional).
    fn init(&mut self) -> Result<()> {
        Ok(())
    }

    /// Dispatcher de eventos — enruta al handler específico.
    fn handle_events(&mut self, event: Option<Event>) -> Action {
        match event {
            Some(Event::Quit) => Action::Quit,
            Some(Event::Tick) => Action::Tick,
            Some(Event::Key(key_event)) => self.handle_key_events(key_event),
            Some(Event::Mouse(mouse_event)) => self.handle_mouse_events(mouse_event),
            Some(_) | None => Action::Noop,
        }
    }

    /// Handler de teclado — hacer override en componentes que lo necesiten.
    fn handle_key_events(&mut self, _key: KeyEvent) -> Action {
        Action::Noop
    }

    /// Handler de mouse — hacer override en componentes que lo necesiten.
    fn handle_mouse_events(&mut self, _mouse: MouseEvent) -> Action {
        Action::Noop
    }

    /// Actualización de estado interno a partir de una acción.
    fn update(&mut self, _action: Action) -> Action {
        Action::Noop
    }

    /// Renderizado del componente — OBLIGATORIO implementar.
    fn render(&mut self, frame: &mut Frame, area: Rect);
}

pub mod buttons;
pub mod grafica_gantt;
pub mod grafica_procesos;
pub mod table;
pub mod title;
