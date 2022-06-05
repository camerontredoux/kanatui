use std::{
    collections::BTreeMap,
    io,
    time::{Duration, Instant},
};

use crossterm::{
    event::{
        self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode, KeyEvent, KeyModifiers,
    },
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use tui::{
    backend::CrosstermBackend,
    style::{Color, Style},
    text::Spans,
    widgets::{ListItem, ListState},
    Terminal,
};

use super::{ui, Hiragana};

pub enum Side {
    Left,
    Right,
}

pub struct HiraganaListState<'a> {
    pub choose_state: ListState,
    pub selected_state: ListState,
    pub items: Vec<ListItem<'a>>,
    pub selected: BTreeMap<usize, usize>,
    pub side: Side,
    pub last_chosen: usize,
    pub last_selected: usize,
}

impl<'a> HiraganaListState<'a> {
    fn new(items: Vec<ListItem<'a>>) -> Self {
        Self {
            choose_state: ListState::default(),
            selected_state: ListState::default(),
            items,
            selected: BTreeMap::new(),
            side: Side::Left,
            last_chosen: 0,
            last_selected: 0,
        }
    }

    fn up(&mut self) {
        match self.side {
            Side::Left => {
                let i = match self.choose_state.selected() {
                    Some(i) => {
                        if i == 0 {
                            self.items.len() - 1
                        } else {
                            i - 1
                        }
                    }
                    None => 0,
                };

                self.choose_state.select(Some(i))
            }
            Side::Right => {
                if self.selected.is_empty() {
                    return;
                }
                let i = match self.selected_state.selected() {
                    Some(i) => {
                        if i == 0 {
                            self.selected.len() - 1
                        } else {
                            i - 1
                        }
                    }
                    None => 0,
                };

                self.selected_state.select(Some(i))
            }
        }
    }

    fn down(&mut self) {
        match self.side {
            Side::Left => {
                let i = match self.choose_state.selected() {
                    Some(i) => {
                        if i >= self.items.len() - 1 {
                            0
                        } else {
                            i + 1
                        }
                    }
                    None => 0,
                };

                self.choose_state.select(Some(i))
            }
            Side::Right => {
                if self.selected.is_empty() {
                    return;
                }

                let i = match self.selected_state.selected() {
                    Some(i) => {
                        if i >= self.selected.len() - 1 {
                            0
                        } else {
                            i + 1
                        }
                    }
                    None => 0,
                };

                self.selected_state.select(Some(i))
            }
        }
    }

    fn right(&mut self) {
        if self.selected.is_empty() {
            return;
        }
        match self.choose_state.selected() {
            Some(i) => self.last_chosen = i,
            None => {}
        }
        match self.side {
            Side::Left => self.selected_state.select(Some(self.last_selected)),
            Side::Right => {}
        }
        self.side = Side::Right;
        self.choose_state.select(None);
    }

    fn left(&mut self) {
        match self.selected_state.selected() {
            Some(i) => self.last_selected = i,
            None => {}
        }
        match self.side {
            Side::Left => {}
            Side::Right => self.choose_state.select(Some(self.last_chosen)),
        }
        self.side = Side::Left;
        self.selected_state.select(None)
    }

    fn choose(&mut self) {
        match self.choose_state.selected() {
            Some(i) => {
                if let None = self.selected.get(&i) {
                    self.selected.insert(i, i);
                }
            }
            None => {}
        }
    }

    fn remove(&mut self) {
        match self.selected_state.selected() {
            Some(i) => {
                let mut vec_selected: Vec<(&usize, &usize)> = self.selected.iter().collect();

                if let Some(_) = vec_selected.get(i) {
                    vec_selected.remove(i);
                }

                self.selected = vec_selected.iter().map(|e| (*e.0, *e.1)).collect();
            }
            None => {}
        }
    }
}

pub struct App<'a> {
    pub item_state: HiraganaListState<'a>,
    pub tabs: Vec<&'a str>,
    pub tab_index: usize,
}

impl<'a> App<'a> {
    fn new(items: Vec<ListItem<'a>>) -> Self {
        Self {
            item_state: HiraganaListState::new(items),
            tabs: vec!["Choose", "Practice"],
            tab_index: 0,
        }
    }

    fn next_tab(&mut self) {
        if self.tab_index == 0 {
            self.tab_index = 1
        }
    }

    fn prev_tab(&mut self) {
        if self.tab_index == 1 {
            self.tab_index = 0
        }
    }
}

pub fn terminal() -> Result<(), io::Error> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let mut last_tick = Instant::now();

    let hiragana = Hiragana::new().unwrap();
    let items: Vec<ListItem> = hiragana
        .iter()
        .map(|item| ListItem::new(Spans::from(format!("{} - {}", item.0, item.1))))
        .collect();
    let mut app = App::new(items);

    loop {
        terminal.draw(|f| ui::ui(f, &mut app))?;

        let timeout = Duration::from_millis(250)
            .checked_sub(last_tick.elapsed())
            .unwrap_or_else(|| Duration::from_secs(0));

        if crossterm::event::poll(timeout)? {
            if let Event::Key(event) = event::read()? {
                match event {
                    KeyEvent { code, modifiers } => {
                        if modifiers == KeyModifiers::SHIFT {
                            match code {
                                KeyCode::Right => app.next_tab(),
                                KeyCode::Left => app.prev_tab(),
                                _ => {}
                            }
                        } else {
                            match code {
                                KeyCode::Char('q') => break,
                                KeyCode::Up => app.item_state.up(),
                                KeyCode::Down => app.item_state.down(),
                                KeyCode::Right => app.item_state.right(),
                                KeyCode::Left => app.item_state.left(),
                                KeyCode::Enter => app.item_state.choose(),
                                KeyCode::Backspace | KeyCode::Delete => app.item_state.remove(),
                                _ => {}
                            }
                        }
                    }
                }
            }
        }
        if last_tick.elapsed() >= Duration::from_millis(250) {
            last_tick = Instant::now();
        }
    }

    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;
    Ok(())
}
