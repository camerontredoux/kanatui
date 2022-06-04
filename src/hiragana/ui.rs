use tui::{
    backend::Backend,
    layout::{Alignment, Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    text::{Span, Spans},
    widgets::{Block, BorderType, Borders, List, ListItem, ListState, Paragraph, Wrap},
    Frame,
};

use super::tui::App;

pub fn ui<'a, B: Backend>(frame: &mut Frame<B>, app: &mut App<'a>) {
    let size = frame.size();

    let title = Span::styled(
        " [ Hiragana Practice ] ",
        Style::default().fg(Color::LightBlue),
    );

    let outer = Block::default()
        .borders(Borders::TOP)
        .title(title)
        .title_alignment(Alignment::Center);
    frame.render_widget(outer, size);

    let main = Layout::default()
        .direction(Direction::Vertical)
        .constraints(
            [
                Constraint::Length(2),
                Constraint::Min(2),
                Constraint::Length(3),
            ]
            .as_ref(),
        )
        .split(size);

    let inner = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)].as_ref())
        .split(main[1]);

    let left = Block::default()
        .title("Choose Kana")
        .borders(Borders::all())
        .title_alignment(Alignment::Center);

    let list = List::new(app.item_state.items.clone())
        .highlight_symbol(">")
        .block(left)
        .highlight_style(Style::default().bg(Color::Cyan));
    frame.render_stateful_widget(list, inner[0], &mut app.item_state.state);

    let right = Block::default()
        .title("Selected Kana")
        .borders(Borders::all())
        .title_alignment(Alignment::Center);

    let selected_items: Vec<ListItem> = app
        .item_state
        .selected
        .iter()
        .map(|(i, _)| app.item_state.items[*i].clone())
        .collect();
    let list = List::new(selected_items)
        .highlight_symbol(">")
        .block(right)
        .highlight_style(Style::default().bg(Color::Cyan));
    frame.render_stateful_widget(list, inner[1], &mut app.item_state.state);

    let text = vec![Spans::from(vec![
        Span::from("Navigate with the "),
        Span::styled("'Up', 'Down'", Style::default().fg(Color::Green)),
        Span::from(" arrow keys and press "),
        Span::styled("'enter'", Style::default().fg(Color::Green)),
        Span::from(" to select an option. Press "),
        Span::styled("'c'", Style::default().fg(Color::Green)),
        Span::from(" to continue to the next section."),
    ])];

    let help = Block::default().borders(Borders::all()).title("Help");
    let description = Paragraph::new(text).block(help).wrap(Wrap { trim: false });
    frame.render_widget(description, main[2]);
}
