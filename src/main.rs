use std::{
    fmt::Display,
    io::{self, stdout},
    time::{Duration, Instant},
};

use crossterm::{
    event::{self, Event, KeyCode, KeyEventKind},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    ExecutableCommand,
};
use ratatui::{
    prelude::*,
    widgets::{self, Block, Borders, ListItem, ListState},
};

struct List<T: Display> {
    state: ListState,
    items: Vec<T>,
    last_selected: Option<usize>,
}

impl<T: Display> List<T> {
    fn new(items: Vec<T>) -> List<T> {
        List {
            state: ListState::default(),
            items,
            last_selected: None,
        }
    }

    fn next(&mut self) {
        let i = match self.state.selected() {
            Some(i) => {
                if i >= self.items.len() - 1 {
                    0
                } else {
                    i + 1
                }
            }
            None => self.last_selected.unwrap_or(0),
        };
        self.state.select(Some(i));
    }

    fn previous(&mut self) {
        let i = match self.state.selected() {
            Some(i) => {
                if i == 0 {
                    self.items.len() - 1
                } else {
                    i - 1
                }
            }
            None => self.last_selected.unwrap_or(0),
        };
        self.state.select(Some(i));
    }

    fn unselect(&mut self) {
        let offset = self.state.offset();
        self.last_selected = self.state.selected();
        self.state.select(None);
        *self.state.offset_mut() = offset;
    }
}

struct App<'a> {
    list: List<&'a str>,
}

impl<'a> App<'a> {
    fn new(list_items: Vec<&'a str>) -> App<'a> {
        App {
            list: List::new(list_items),
        }
    }
}

fn main() -> io::Result<()> {
    enable_raw_mode()?;
    stdout().execute(EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout());
    let mut terminal = Terminal::new(backend)?;

    let app = App::new(vec![
        "item1", "item2", "item3", "item4", "item5", "item6", "item7", "item8", "item9",
    ]);
    let tick_rate = Duration::from_millis(250);
    let res = run_app(&mut terminal, app, tick_rate);

    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    terminal.show_cursor()?;

    if let Err(err) = res {
        return Err(err);
    }

    Ok(())
}

fn run_app<B: Backend>(
    terminal: &mut Terminal<B>,
    mut app: App,
    tick_rate: Duration,
) -> io::Result<()> {
    let mut last_tick = Instant::now();
    loop {
        terminal.draw(|f| ui(f, &mut app))?;
        let timeout = tick_rate.saturating_sub(last_tick.elapsed());
        if crossterm::event::poll(timeout)? {
            if let Event::Key(key) = event::read()? {
                if key.kind == KeyEventKind::Press {
                    match key.code {
                        KeyCode::Char('q') => return Ok(()),
                        KeyCode::Left | KeyCode::Char('h') => app.list.unselect(),
                        KeyCode::Down | KeyCode::Char('j') => app.list.next(),
                        KeyCode::Up | KeyCode::Char('k') => app.list.previous(),
                        _ => {}
                    }
                }
            }
        }
        if last_tick.elapsed() >= tick_rate {
            last_tick = Instant::now();
        }
    }
}

fn ui(frame: &mut Frame, app: &mut App) {
    let list_items: Vec<widgets::ListItem> = app
        .list
        .items
        .iter()
        .map(|i| {
            let lines = vec![Line::from(i.bold()).alignment(Alignment::Center)];
            ListItem::new(lines).style(Style::default().fg(Color::Black).bg(Color::White))
        })
        .collect();

    let list = widgets::List::new(list_items)
        .block(Block::default().borders(Borders::ALL).title("List"))
        .highlight_style(
            Style::default()
                .bg(Color::LightGreen)
                .add_modifier(Modifier::BOLD),
        )
        .highlight_symbol(">> ");

    frame.render_stateful_widget(list, frame.size(), &mut app.list.state);
}
