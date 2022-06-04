use std::{
    collections::{BTreeMap, HashMap},
    io,
    time::{Duration, Instant},
};

use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use tui::{
    backend::CrosstermBackend,
    text::Spans,
    widgets::{ListItem, ListState},
    Terminal,
};

use super::{ui, Hiragana};

pub struct HiraganaListState<T> {
    pub state: ListState,
    pub items: Vec<T>,
    pub selected: BTreeMap<usize, bool>,
}

impl<T> HiraganaListState<T> {
    fn new(items: Vec<T>) -> Self {
        Self {
            state: ListState::default(),
            items,
            selected: BTreeMap::new(),
        }
    }

    fn up(&mut self) {
        let i = match self.state.selected() {
            Some(i) => {
                if i == 0 {
                    self.items.len() - 1
                } else {
                    i - 1
                }
            }
            None => 0,
        };

        self.state.select(Some(i))
    }

    fn down(&mut self) {
        let i = match self.state.selected() {
            Some(i) => {
                if i >= self.items.len() - 1 {
                    0
                } else {
                    i + 1
                }
            }
            None => 0,
        };

        self.state.select(Some(i))
    }

    fn select(&mut self) {
        match self.state.selected() {
            Some(i) => {
                if let None = self.selected.get(&i) {
                    self.selected.insert(i, true);
                }
            }
            None => {}
        }
    }

    fn remove(&mut self) {
        match self.state.selected() {
            Some(i) => {
                if let Some(idx) = self.selected.get(&i) {
                    self.selected.remove(&i);
                }
            }
            None => {}
        }
    }
}

pub struct App<'a> {
    pub item_state: HiraganaListState<ListItem<'a>>,
}

impl<'a> App<'a> {
    fn new(items: Vec<ListItem<'a>>) -> Self {
        Self {
            item_state: HiraganaListState::new(items),
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

        let timeout = Duration::from_millis(1000)
            .checked_sub(last_tick.elapsed())
            .unwrap_or_else(|| Duration::from_secs(0));

        if crossterm::event::poll(timeout)? {
            if let Event::Key(key) = event::read()? {
                match key.code {
                    KeyCode::Char('q') => break,
                    KeyCode::Up => app.item_state.up(),
                    KeyCode::Down => app.item_state.down(),
                    KeyCode::Enter => app.item_state.select(),
                    KeyCode::Backspace | KeyCode::Delete => app.item_state.remove(),
                    _ => {}
                }
                if let KeyCode::Char('q') = key.code {
                    break;
                }
            }
        }
        if last_tick.elapsed() >= Duration::from_millis(1000) {
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
