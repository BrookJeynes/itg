use anyhow::Result;
use crossterm::event::{self, Event, KeyCode};
use tui::{backend::Backend, Terminal};

use crate::{ui::ui, AppState, MenuItems};

pub fn run_app<B: Backend>(terminal: &mut Terminal<B>, mut app_state: AppState) -> Result<()> {
    loop {
        terminal.draw(|f| ui(f, &mut app_state))?;

        if let Event::Key(key) = event::read()? {
            match key.code {
                // Menu switcher
                KeyCode::Char('I') => app_state.current_menu = MenuItems::Issues,
                KeyCode::Char('P') => app_state.current_menu = MenuItems::PullRequests,

                // Exit keys
                KeyCode::Char('q') => return Ok(()),

                _ => {}
            }
        }
    }
}
