use tui::{
    backend::Backend,
    layout::{Alignment, Constraint, Layout},
    style::{Color, Style},
    text::{Span, Spans},
    widgets::{Block, Borders, Paragraph, Wrap},
    Frame,
};

use crate::{AppState, MenuItems};

pub fn ui<B: Backend>(f: &mut Frame<B>, app_state: &mut AppState) {
    let size = f.size();

    // A helper closure to create blocks
    let create_block = |title: &str| {
        Block::default()
            .borders(Borders::ALL)
            .title(title.to_string())
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

    let inner = Layout::default()
        .direction(tui::layout::Direction::Horizontal)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
        .split(main[1]);

    f.render_widget(render_menu_bar(&app_state), main[0]);
    f.render_widget(create_block(""), inner[0]);
    f.render_widget(create_block(""), inner[1]);
    f.render_widget(render_controls(), main[2]);
}

fn render_controls<'a>() -> Paragraph<'a> {
    Paragraph::new("q: quit")
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
