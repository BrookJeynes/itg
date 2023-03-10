use ansi_to_tui::IntoText;
use tui::{
    backend::Backend,
    layout::{Alignment, Constraint, Corner, Layout},
    style::{Color, Style},
    text::{Span, Spans, Text},
    widgets::{Block, Borders, List, ListItem, Paragraph, Wrap},
    Frame,
};

use crate::{models::stateful_list::StatefulList, AppState, MenuItems};

pub fn ui<B: Backend>(f: &mut Frame<B>, app_state: &mut AppState) {
    let size = f.size();

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

    f.render_widget(render_menu_bar(&app_state), main[0]);
    f.render_stateful_widget(
        render_list(&app_state.issues),
        inner[0],
        &mut app_state.issues.state,
    );
    f.render_widget(
        render_markdown(if let Some(index) = app_state.issues.selected() {
            match app_state.issues.items.get(index) {
                Some(issue) => issue.body.as_str(),
                None => "",
            }
        } else {
            ""
        }),
        inner[1],
    );
    f.render_widget(render_controls(), main[2]);
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
        .block(Block::default().borders(Borders::ALL))
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
        .block(Block::default().borders(Borders::ALL))
}

fn render_controls<'a>() -> Paragraph<'a> {
    Paragraph::new("q: quit, Up / k && Down / j: scroll list, Enter: open issue")
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

    Paragraph::new(items)
        .alignment(Alignment::Left)
        .block(Block::default().borders(Borders::ALL))
}
