use anyhow::Result;
use crossterm::event::{self, Event, KeyCode};
use termimad::crossterm::style::Stylize;
use tui::{backend::Backend, Terminal};

use crate::{
    api_requests::fetch_issues_repo,
    models::{screen::Screen, stateful_list::StatefulList},
    reset_terminal,
    ui::ui,
    AppState, MenuItems,
};

pub async fn run_app<B: Backend>(
    terminal: &mut Terminal<B>,
    mut app_state: AppState,
) -> Result<()> {
    loop {
        terminal.draw(|f| ui(f, &mut app_state))?;

        if let Event::Key(key) = event::read()? {
            match key.code {
                // Menu switcher
                KeyCode::Char('I') => app_state.current_menu = MenuItems::Issues,
                KeyCode::Char('P') => app_state.current_menu = MenuItems::PullRequests,

                // Focus switcher
                KeyCode::Tab => app_state.change_focus(),

                // Issue controls
                KeyCode::Up | KeyCode::Char('k') => match app_state.screen {
                    Screen::Issues => app_state.get_issues().previous(),
                    Screen::Repositories => app_state.repositories.previous(),
                },
                KeyCode::Down | KeyCode::Char('j') => match app_state.screen {
                    Screen::Issues => app_state.get_issues().next(),
                    Screen::Repositories => app_state.repositories.next(),
                },
                KeyCode::Enter => match app_state.screen {
                    Screen::Issues => {
                        if let Some(issue) = app_state.get_issues().selected_value() {
                            // Open issue in browser
                            webbrowser::open(issue.html_url.as_str()).unwrap_or_else(|err| {
                                eprintln!("{}: {}", "Error".red().bold(), err);
                                reset_terminal()
                                    .unwrap_or_else(|_| panic!("Failed to reset terminal"));
                                std::process::exit(1);
                            });
                        }
                    }
                    Screen::Repositories => {
                        if let Some(repo) = app_state.repositories.selected_value() {
                            // Maybe a better way than so much cloning here
                            let repo = repo.clone();

                            // and here
                            app_state.select_repo(repo.clone());

                            // Check cache for issues
                            match app_state.issue_cache.get(&repo.full_name) {
                                Some(issues) => {
                                    app_state.issues =
                                        Some(StatefulList::with_items(issues.clone()));
                                }
                                // Fetch issues for repo and add to cache
                                None => {
                                    // This blocks input
                                    let issues = fetch_issues_repo(
                                        &app_state.config,
                                        repo.full_name.as_str(),
                                    )
                                    .await?;

                                    app_state.issues =
                                        Some(StatefulList::with_items(issues.clone()));
                                    app_state.cache_issues(repo.full_name, issues);
                                }
                            };
                        }
                    }
                },

                // Exit keys
                KeyCode::Char('q') => return Ok(()),

                _ => {}
            }
        }
    }
}
