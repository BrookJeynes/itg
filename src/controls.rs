use anyhow::Result;
use crossterm::event::{self, Event, KeyCode};
use itertools::Itertools;
use termimad::crossterm::style::Stylize;
use tui::{backend::Backend, Terminal};

use crate::{
    api_requests::fetch_issues_repo,
    models::{
        errors::Errors, input_mode::InputMode, repository::Repository, screen::Screen,
        stateful_list::StatefulList,
    },
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
            match app_state.input_mode {
                InputMode::Normal => {
                    match key.code {
                        // Menu switcher
                        KeyCode::Char('I') => app_state.current_menu = MenuItems::Issues,

                        // Focus switcher
                        KeyCode::Tab => app_state.change_focus(),

                        // Issue controls
                        KeyCode::Up | KeyCode::Char('k') => match app_state.screen {
                            Screen::Issues => app_state.issues.previous(),
                            Screen::Repositories => app_state.repositories.previous(),
                            Screen::Error => {}
                        },
                        KeyCode::Down | KeyCode::Char('j') => match app_state.screen {
                            Screen::Issues => app_state.issues.next(),
                            Screen::Repositories => app_state.repositories.next(),
                            Screen::Error => {}
                        },
                        KeyCode::Enter => match app_state.screen {
                            Screen::Issues => {
                                if let Some(issue) = app_state.issues.selected_value() {
                                    // Open issue in browser
                                    webbrowser::open(issue.html_url.as_str()).unwrap_or_else(
                                        |err| {
                                            eprintln!("{}: {}", "Error".red().bold(), err);
                                            reset_terminal().unwrap_or_else(|_| {
                                                panic!("Failed to reset terminal")
                                            });
                                            std::process::exit(1);
                                        },
                                    );
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
                                                StatefulList::with_items(issues.clone());
                                        }
                                        // Fetch issues for repo and add to cache
                                        None => {
                                            // This blocks input
                                            match fetch_issues_repo(
                                                &app_state.config,
                                                repo.full_name.as_str(),
                                            )
                                            .await
                                            {
                                                Ok(issues) => {
                                                    app_state.issues =
                                                        StatefulList::with_items(issues.clone());
                                                    app_state.cache_issues(repo.full_name, issues);
                                                }
                                                Err(_) => app_state.show_error(
                                                    Errors::FetchRequestError.to_string(),
                                                ),
                                            }
                                        }
                                    };
                                }
                            }
                            Screen::Error => app_state.close_error(),
                        },

                        // Search repo
                        KeyCode::Char('S') => app_state.show_search(),

                        // Exit keys
                        KeyCode::Char('q') => return Ok(()),

                        _ => {}
                    }
                }
                InputMode::Editing => match key.code {
                    KeyCode::Enter => {
                        let search = app_state.search_string.trim();
                        let (_, repo) = app_state
                            .search_string
                            .split("/")
                            .next_tuple()
                            .unwrap_or(("", ""));

                        let repo = Repository {
                            full_name: String::from(search),
                            name: String::from(repo),
                            open_issues_count: 0,
                        };

                        // Check cache for issues
                        match app_state.issue_cache.get(&repo.full_name) {
                            Some(issues) => {
                                app_state.issues = StatefulList::with_items(issues.clone());
                                app_state.search_string = String::new();
                                app_state.hide_search();
                            }
                            // Fetch issues for repo and add to cache
                            None => {
                                // This blocks input
                                match fetch_issues_repo(&app_state.config, search).await {
                                    Ok(issues) => {
                                        app_state.issues = StatefulList::with_items(issues.clone());
                                        app_state.selected_repo = Some(repo.clone());
                                        app_state.repositories.items.insert(0, repo.clone());
                                        app_state.cache_issues(repo.full_name, issues);

                                        app_state.search_string = String::new();
                                        app_state.hide_search();
                                    }
                                    Err(_) => {
                                        app_state.show_error(Errors::FetchRequestError.to_string())
                                    }
                                }
                            }
                        };
                    }
                    KeyCode::Char(c) => {
                        app_state.search_string.push(c);
                    }
                    KeyCode::Backspace => {
                        app_state.search_string.pop();
                    }
                    KeyCode::Esc => app_state.hide_search(),

                    _ => {}
                },
            }
        }
    }
}
