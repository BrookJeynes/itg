use ansi_to_tui::IntoText;
use tui::{
    backend::Backend,
    layout::{Alignment, Constraint, Corner, Direction, Layout, Rect},
    style::{Color, Style},
    text::{Span, Spans, Text},
    widgets::{Block, BorderType, Borders, Clear, List, ListItem, Paragraph, Wrap},
    Frame,
};

use crate::{
    models::{screen::Screen, stateful_list::StatefulList},
    AppState, MenuItems,
};

pub fn ui<B: Backend>(f: &mut Frame<B>, app_state: &mut AppState) {
    let size = f.size();

    let repo_name = match &app_state.selected_repo {
        Some(repo) => repo.full_name.clone(),
        None => String::new(),
    };

    // A helper closure to create blocks
    let create_block = |title: &str| {
        Block::default()
            .borders(Borders::ALL)
            .title(title.to_string())
            .border_type(BorderType::Rounded)
    };

    // The main canvas
    let main = Layout::default()
        .horizontal_margin(2)
        .constraints([
            Constraint::Percentage(7),
            Constraint::Percentage(90),
            Constraint::Percentage(5),
        ])
        .split(size);

    // Inner canvas handling the list and preview
    let inner = Layout::default()
        .direction(tui::layout::Direction::Horizontal)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
        .split(main[1]);

    // Canvas split between showing issues and repos
    let issues_repos = Layout::default()
        .constraints([Constraint::Percentage(70), Constraint::Percentage(30)])
        .split(inner[0]);

    f.render_widget(render_menu_bar(&app_state), main[0]);

    if app_state.issues.items.is_empty() {
        f.render_widget(
            Paragraph::new("No issues found..").block(
                create_block(format!("Issues - {}", repo_name).as_str()).border_style(
                    Style::default().fg(if app_state.screen == Screen::Issues {
                        Color::Yellow
                    } else {
                        Color::White
                    }),
                ),
            ),
            issues_repos[0],
        )
    } else {
        f.render_stateful_widget(
            render_list(&app_state.issues).block(
                create_block(format!("Issues - {}", repo_name).as_str()).border_style(
                    Style::default().fg(if app_state.screen == Screen::Issues {
                        Color::Yellow
                    } else {
                        Color::White
                    }),
                ),
            ),
            issues_repos[0],
            &mut app_state.issues.state,
        );
    }

    f.render_stateful_widget(
        render_list(&app_state.repositories).block(create_block("Repositories").border_style(
            Style::default().fg(if app_state.screen == Screen::Repositories {
                Color::Yellow
            } else {
                Color::White
            }),
        )),
        issues_repos[1],
        &mut app_state.repositories.state,
    );
    f.render_widget(
        render_markdown(match app_state.issues.selected_value() {
            Some(issue) => issue.body.as_str(),
            None => "",
        }),
        inner[1],
    );
    f.render_widget(render_controls(), main[2]);

    if app_state.show_search {
        let area = render_centered_rect(70, 7, size);
        f.render_widget(Clear, area); //this clears out the background
        f.render_widget(render_search_box(app_state.search_string.as_str()), area)
    }
}

fn render_search_box<'a>(search_text: &'a str) -> Paragraph<'a> {
    Paragraph::new(search_text)
        .alignment(Alignment::Left)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded)
                .title("Search - {user_name}/{repo_name}"),
        )
}

fn parse_markdown_headers(content: &str) -> String {
    content
        .lines()
        .map(|line| {
            let mut parsed_content = String::new();

            // Check if line starts with '#'
            if line.chars().nth(0).unwrap_or(' ') != '#' {
                return format!("{}\n", line);
            }

            // Parse markdown headings
            match String::from_iter(line.chars().take(6).filter(|char| char == &'#')).as_str() {
                "#" => {
                    parsed_content
                        .push_str(format!("\x1b[1;32m██ {}\x1b[0m\n", line[1..].trim()).as_str());
                }
                "##" => {
                    parsed_content
                        .push_str(format!("\x1b[1;36m▓▓▓ {}\x1b[0m\n", line[2..].trim()).as_str());
                }
                "###" => {
                    parsed_content
                        .push_str(format!("\x1b[1;33m▒▒▒▒ {}\x1b[0m\n", line[3..].trim()).as_str());
                }
                "####" => {
                    parsed_content.push_str(
                        format!("\x1b[1;35m░░░░░ {}\x1b[0m\n", line[4..].trim()).as_str(),
                    );
                }
                "#####" => {
                    parsed_content.push_str(format!("{}\n", line[5..].trim()).as_str());
                }
                "######" => {
                    parsed_content.push_str(format!("{}\n", line[6..].trim()).as_str());
                }
                _ => parsed_content.push_str(format!("{}\n", line).as_str()),
            }

            parsed_content
        })
        .collect::<Vec<String>>()
        .join("")
}

fn render_list<'a, T: std::fmt::Display>(items: &StatefulList<T>) -> List<'a> {
    let items: Vec<ListItem> = items
        .items
        .iter()
        .map(|item| ListItem::new(item.to_string()))
        .collect();

    List::new(items)
        .highlight_style(Style::default().fg(Color::LightGreen))
        .start_corner(Corner::TopLeft)
}

fn render_markdown<'a>(content: &'a str) -> Paragraph<'a> {
    let parsed_content = parse_markdown_headers(content);

    // Convert md content to ansi string
    let output = termimad::text(&parsed_content.as_str())
        .to_string()
        // Convert ansi string to tui::text::Text
        .into_text()
        .unwrap_or(Text::from(content));

    Paragraph::new(output)
        .wrap(Wrap { trim: false })
        .alignment(Alignment::Left)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title("Issue preview")
                .border_type(BorderType::Rounded),
        )
}

fn render_controls<'a>() -> Paragraph<'a> {
    Paragraph::new("q: quit, Up / k && Down / j: scroll list, Enter: open/select issue/repository, Tab: switch focus, S: search repo")
        .wrap(Wrap { trim: false })
        .alignment(Alignment::Left)
}

fn render_menu_bar<'a>(app_state: &AppState) -> Paragraph<'a> {
    let items = vec![Spans::from(
        MenuItems::iterator()
            .enumerate()
            .flat_map(|(index, menu_item)| {
                let mut items = vec![Span::styled(
                    menu_item.to_string(),
                    Style::default().fg(if app_state.current_menu == *menu_item {
                        Color::Blue
                    } else {
                        Color::White
                    }),
                )];

                if index != (MenuItems::iterator().count() - 1) {
                    items.push(Span::raw(" | "));
                }

                items
            })
            .collect::<Vec<Span>>(),
    )];

    Paragraph::new(items).alignment(Alignment::Left).block(
        Block::default()
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded),
    )
}

/// helper function to create a centered rect using up certain percentage of the available rect `r`
fn render_centered_rect(percent_x: u16, percent_y: u16, r: Rect) -> Rect {
    let popup_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints(
            [
                Constraint::Percentage((100 - percent_y) / 2),
                Constraint::Percentage(percent_y),
                Constraint::Percentage((100 - percent_y) / 2),
            ]
            .as_ref(),
        )
        .split(r);

    Layout::default()
        .direction(Direction::Horizontal)
        .constraints(
            [
                Constraint::Percentage((100 - percent_x) / 2),
                Constraint::Percentage(percent_x),
                Constraint::Percentage((100 - percent_x) / 2),
            ]
            .as_ref(),
        )
        .split(popup_layout[1])[1]
}
