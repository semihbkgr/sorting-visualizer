use anyhow::{anyhow, Result};
use crossterm::{
    event::{self, Event, KeyCode, KeyEvent, KeyEventKind},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    ExecutableCommand,
};
use ratatui::{
    prelude::*,
    widgets::{self, Block, BorderType::Rounded, Borders, ListItem, ListState, Paragraph},
};
use sorting_visualizer::{
    init_vec, shuffle,
    sorting::{bubble_sort, AlgorithmContext, Operation},
};
use std::{
    fmt::Display,
    io::{self, stdout},
    sync::{Arc, Mutex},
    thread,
    time::{Duration, Instant},
    vec,
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
    algorithm: Option<AlgorithmUI>,
}

impl<'a> App<'a> {
    fn new(list_items: Vec<&'a str>) -> App<'a> {
        App {
            list: List::new(list_items),
            algorithm: Option::None,
        }
    }
}

const BLOCK_FULL: char = '\u{2588}';
const BLOCK_HALF_QUARTER: char = '\u{2586}';
const BLOCK_HALF: char = '\u{2584}';
const BLOCK_QUARTER: char = '\u{2582}';

struct AlgorithmUI {
    status: Arc<AlgorithmStatus>,
    blocks: Vec<String>,
    size: (u16, u16),
}

impl AlgorithmUI {
    fn new(name: String, size: Rect) -> Result<AlgorithmUI> {
        let blocks_size = blocks_size(size)?;
        Ok(AlgorithmUI {
            status: Arc::new(AlgorithmStatus::new(name, blocks_size.0 as usize)),
            blocks: block_strings(blocks_size.0 as usize),
            size: blocks_size,
        })
    }

    // todo: optimize
    fn display_text(&self) -> Text {
        let mut lines = Vec::new();
        let guard = self.status.operations.lock().unwrap();
        let (operation, nums) = guard.last().unwrap();

        for i in nums.iter() {
            lines.push(self.blocks[(*i as usize) - 1].clone());
        }

        let max_height = self.blocks.last().unwrap().chars().count();
        let mut result = String::new();
        for i in 0..max_height {
            for j in lines.iter() {
                let c = (*j).chars().nth(max_height - i - 1).unwrap_or(' ');
                result.push(c);
            }
            result.push('\n');
        }
        let mut text = Text::raw(result);
        match operation {
            Operation::Compare(a, b) => {
                for line in text.lines.iter_mut() {
                    let line_content = line.spans[0].content.clone();
                    let line_chars = line_content.chars().into_iter().collect::<Vec<char>>();
                    let pre = Span::raw(line_chars[..*a].iter().collect::<String>());
                    let a_span = Span::raw(line_chars[*a..*a + 1].iter().collect::<String>())
                        .fg(Color::Green);
                    let mid = Span::raw(line_chars[*a + 1..*b].iter().collect::<String>());
                    let b_span = Span::raw(line_chars[*b..*b + 1].iter().collect::<String>())
                        .fg(Color::Green);
                    let last = Span::raw(line_chars[*b + 1..].iter().collect::<String>());

                    line.spans = vec![pre, a_span, mid, b_span, last];
                }
            }
            _ => {}
        }
        return text;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_block_strings() {
        let blocks = block_strings(10);
        println!("{:?}", blocks);
    }
}

fn block_strings(n: usize) -> Vec<String> {
    let mut v = Vec::new();
    for i in 1..n + 1 {
        let mut s = String::new();
        for _ in 0..i / 4 {
            s.push(BLOCK_FULL);
        }
        match i % 4 {
            1 => s.push(BLOCK_QUARTER),
            2 => s.push(BLOCK_HALF),
            3 => s.push(BLOCK_HALF_QUARTER),
            _ => {}
        }
        v.push(s);
    }
    return v;
}

const WIDTH: u16 = 32;
const HEIGHT: u16 = WIDTH / 4;

fn blocks_size(s: Rect) -> anyhow::Result<(u16, u16)> {
    if s.width < WIDTH {
        return Err(anyhow!("width is too small".to_string()));
    }
    if s.height < HEIGHT {
        return Err(anyhow!("height is too small".to_string()));
    }
    return Ok((WIDTH, HEIGHT));
}

struct AlgorithmStatus {
    nums: Vec<i32>,
    operations: Mutex<Vec<(Operation, Vec<i32>)>>,
    name: String,
    proceed: Mutex<bool>,
}

impl AlgorithmStatus {
    fn new(name: String, size: usize) -> AlgorithmStatus {
        let mut v = init_vec(size);
        shuffle(&mut v);
        let operations = Vec::from(vec![(Operation::Noop(), v.clone())]);
        return AlgorithmStatus {
            nums: v,
            operations: Mutex::new(operations),
            name,
            proceed: Mutex::new(false),
        };
    }

    fn proceed(&self) {
        *self.proceed.lock().unwrap() = true;
    }
}

impl AlgorithmContext for AlgorithmStatus {
    fn next(&self, operation: Operation, nums: Vec<i32>) {
        {
            let mut proceed = self.proceed.lock().unwrap();
            *proceed = false;
            self.operations.lock().unwrap().push((operation, nums));
        }

        loop {
            {
                if *self.proceed.lock().unwrap() {
                    break;
                }
            }
            thread::yield_now();
        }
    }
}

fn main() -> io::Result<()> {
    enable_raw_mode()?;
    stdout().execute(EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout());
    let mut terminal = Terminal::new(backend)?;

    let app = App::new(vec![
        "item1",
        "item2",
        "bubble sort",
        "item4",
        "item5",
        "item6",
        "item7",
        "item8",
        "item9",
    ]);
    let tick_rate = Duration::from_millis(50);
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
                if handle_key_events(key, &mut app, terminal.size()?)? {
                    return Ok(());
                }
            }
        }
        if last_tick.elapsed() >= tick_rate {
            last_tick = Instant::now();
        }
    }
}

fn ui(frame: &mut Frame, app: &mut App) {
    match &mut app.algorithm {
        None => {
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
        Some(algorithm_ui) => {
            /*
            let len;
            let operation;
            {
                let steps = algorithm_ui.status.operations.lock().unwrap();
                len = steps.len();
                operation = steps.last().unwrap().0;
            }
            */

            let blocks_width = algorithm_ui.size.0 + 2;
            let blocks_height = algorithm_ui.size.1 + 2;
            let x_position = (frame.size().width - blocks_width) / 2;
            let y_position = (frame.size().height - blocks_height) / 2;
            let rect = Rect::new(x_position, y_position, blocks_width, blocks_height);

            let text = algorithm_ui.display_text();
            let paragraph = Paragraph::new(text)
                .alignment(Alignment::Center)
                .block(Block::default().border_type(Rounded).borders(Borders::ALL));
            frame.render_widget(paragraph, rect);
            algorithm_ui.status.as_ref().proceed();
            return;
        }
    }
}

fn handle_key_events(key: KeyEvent, app: &mut App, size: Rect) -> io::Result<bool> {
    if key.kind == KeyEventKind::Press {
        match app.algorithm {
            None => match key.code {
                KeyCode::Char('q') => return Ok(true),
                KeyCode::Left | KeyCode::Char('h') => app.list.unselect(),
                KeyCode::Down | KeyCode::Char('j') => app.list.next(),
                KeyCode::Up | KeyCode::Char('k') => app.list.previous(),
                KeyCode::Enter => {
                    if let Some(i) = app.list.state.selected() {
                        let name = app.list.items[i].to_string();
                        let algorithm = AlgorithmUI::new(name.clone(), size).unwrap();
                        let status = algorithm.status.clone();
                        let algorithm_func = get_algorithm_func(name.clone());
                        thread::spawn(move || {
                            algorithm_func(
                                status.as_ref().nums.clone().as_mut_slice(),
                                status.as_ref(),
                            );
                        });
                        app.algorithm = Some(algorithm);
                    }
                }
                _ => {}
            },
            Some(_) => match key.code {
                KeyCode::Esc => app.algorithm = None,
                _ => {}
            },
        }
    }
    Ok(false)
}

fn get_algorithm_func<'a>(s: String) -> impl FnOnce(&mut [i32], &dyn AlgorithmContext) {
    match s.as_str() {
        "bubble sort" => bubble_sort::sort,
        _ => panic!("algorithm not found"),
    }
}
