use tui::{
    backend::Backend,
    layout::{Alignment, Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    text::{Span, Spans},
    widgets::{Block, Borders, List, ListItem, Paragraph, Tabs, Wrap},
    Frame,
};

use super::tui::App;

pub fn ui<'a, B: Backend>(frame: &mut Frame<B>, app: &mut App<'a>) {
    let size = frame.size();

    if size.height < 15 || size.width < 50 {
        frame.render_widget(
            Block::default()
                .title("Window too small")
                .title_alignment(Alignment::Center),
            size,
        );
        return;
    }

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

    let titles = app
        .tabs
        .iter()
        .map(|&t| Spans::from(Span::from(t)))
        .collect();
    let tabs = Tabs::new(titles)
        .divider("")
        .highlight_style(
            Style::default()
                .bg(Color::Blue)
                .add_modifier(Modifier::BOLD),
        )
        .select(app.tab_index);
    frame.render_widget(tabs, main[0]);

    let inner = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)].as_ref())
        .split(main[1]);

    let left = Block::default()
        .title("Choose Kana")
        .borders(Borders::all())
        .title_alignment(Alignment::Center);

    let list = List::new(app.item_state.items.clone())
        .block(left)
        .highlight_style(Style::default().bg(Color::Blue));
    frame.render_stateful_widget(list, inner[0], &mut app.item_state.choose_state);

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
        .block(right)
        .highlight_style(Style::default().bg(Color::Blue));
    frame.render_stateful_widget(list, inner[1], &mut app.item_state.selected_state);

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

    match app.tab_index {
        0 => {}
        1 => {}
        _ => {}
    }
}
