use anyhow::Result;
use crossterm::event::{self, Event, KeyCode};
use termimad::crossterm::style::Stylize;
use tui::{backend::Backend, Terminal};

use crate::{reset_terminal, ui::ui, AppState, MenuItems};

pub fn run_app<B: Backend>(terminal: &mut Terminal<B>, mut app_state: AppState) -> Result<()> {
    loop {
        terminal.draw(|f| ui(f, &mut app_state))?;

        if let Event::Key(key) = event::read()? {
            match key.code {
                // Menu switcher
                KeyCode::Char('I') => app_state.current_menu = MenuItems::Issues,
                KeyCode::Char('P') => app_state.current_menu = MenuItems::PullRequests,

                // Issue controls
                KeyCode::Up | KeyCode::Char('k') => app_state.issues.previous(),
                KeyCode::Down | KeyCode::Char('j') => app_state.issues.next(),
                KeyCode::Enter => {
                    if let Some(index) = app_state.issues.selected() {
                        if let Some(issue) = app_state.issues.items.get(index) {
                            webbrowser::open(issue.html_url.as_str()).unwrap_or_else(|err| {
                                eprintln!("{}: {}", "Error".red().bold(), err);
                                reset_terminal()
                                    .unwrap_or_else(|_| panic!("Failed to reset terminal"));
                                std::process::exit(1);
                            });
                        }
                    }
                }

                // Exit keys
                KeyCode::Char('q') => return Ok(()),

                _ => {}
            }
        }
    }
}
