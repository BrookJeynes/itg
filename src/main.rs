pub mod controls;
pub mod ui;

use anyhow::Result;
use controls::run_app;
use core::fmt;
use std::{io, slice::Iter};

use crossterm::{
    style::Stylize,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use tui::{backend::CrosstermBackend, Terminal};

#[derive(PartialEq)]
pub enum MenuItems {
    Issues,
    PullRequests,
}

impl MenuItems {
    pub fn iterator() -> Iter<'static, MenuItems> {
        static MENU_ITEMS: [MenuItems; 2] = [MenuItems::Issues, MenuItems::PullRequests];
        MENU_ITEMS.iter()
    }
}

impl fmt::Display for MenuItems {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Issues => write!(f, "[I]ssues"),
            Self::PullRequests => write!(f, "[P]ull requests"),
        }
    }
}

pub struct AppState {
    current_menu: MenuItems,
}

impl Default for AppState {
    fn default() -> Self {
        Self {
            current_menu: MenuItems::Issues,
        }
    }
}

fn main() -> Result<()> {
    let mut terminal = init_terminal()?;

    let app_state = AppState::default();
    let res = run_app(&mut terminal, app_state);

    reset_terminal()?;

    if let Err(err) = res {
        eprintln!("{}: {}", "Error".red().bold(), err);
        std::process::exit(1);
    }

    Ok(())
}

fn init_terminal() -> Result<Terminal<CrosstermBackend<io::Stdout>>> {
    crossterm::execute!(io::stdout(), EnterAlternateScreen)?;
    enable_raw_mode()?;

    let backend = CrosstermBackend::new(io::stdout());

    let mut terminal = Terminal::new(backend)?;
    terminal.hide_cursor()?;

    let original_hook = std::panic::take_hook();

    std::panic::set_hook(Box::new(move |panic| {
        reset_terminal().unwrap();
        original_hook(panic);
    }));

    Ok(terminal)
}

fn reset_terminal() -> Result<()> {
    disable_raw_mode()?;
    crossterm::execute!(io::stdout(), LeaveAlternateScreen)?;

    Ok(())
}
