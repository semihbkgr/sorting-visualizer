use anyhow::{anyhow, Result};
use crossterm::{
    event::{self, Event, KeyCode, KeyEvent, KeyEventKind, KeyModifiers},
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
    sorting::{get_algorithm_func, get_algorithms, AlgorithmContext, Operation},
};
use std::{
    fmt::Display,
    io::{self, stdout},
    ops::{DerefMut, Index},
    sync::{Arc, Mutex},
    thread,
    time::{Duration, Instant, SystemTime, UNIX_EPOCH},
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
    auto_next: bool,
    tick_rate: Duration,
    last_tick: Duration,
}

impl AlgorithmUI {
    fn new(name: String, size: Rect, tick_rate: Duration) -> Result<AlgorithmUI> {
        let blocks_size = blocks_size(size)?;
        Ok(AlgorithmUI {
            status: Arc::new(AlgorithmStatus::new(name, blocks_size.0 as usize)),
            blocks: block_strings(blocks_size.0 as usize),
            size: blocks_size,
            auto_next: true,
            tick_rate,
            last_tick: Duration::ZERO,
        })
    }

    // todo: optimize
    fn display_text(&self) -> Text {
        let mut lines = Vec::new();
        let index = self.status.index.lock().unwrap();
        let operations = self.status.operations.lock().unwrap();
        let (operation, nums) = operations.index(*index);

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
        match operation.adjusted() {
            Operation::Compare(a, b) => {
                for line in text.lines.iter_mut() {
                    let line_content = line.spans[0].content.clone();
                    let line_chars = line_content.chars().into_iter().collect::<Vec<char>>();
                    let pre = Span::raw(line_chars[..a].iter().collect::<String>());
                    let a_span = Span::raw(line_chars[a..a + 1].iter().collect::<String>())
                        .fg(Color::LightCyan);
                    let mid = Span::raw(line_chars[a + 1..b].iter().collect::<String>());
                    let b_span = Span::raw(line_chars[b..b + 1].iter().collect::<String>())
                        .fg(Color::LightCyan);
                    let last = Span::raw(line_chars[b + 1..].iter().collect::<String>());

                    line.spans = vec![pre, a_span, mid, b_span, last];
                }
            }
            Operation::Swap(a, b) => {
                for line in text.lines.iter_mut() {
                    let line_content = line.spans[0].content.clone();
                    let line_chars = line_content.chars().into_iter().collect::<Vec<char>>();
                    let pre = Span::raw(line_chars[..a].iter().collect::<String>());
                    let a_span = Span::raw(line_chars[a..a + 1].iter().collect::<String>())
                        .fg(Color::LightGreen);
                    let mid = Span::raw(line_chars[a + 1..b].iter().collect::<String>());
                    let b_span = Span::raw(line_chars[b..b + 1].iter().collect::<String>())
                        .fg(Color::LightGreen);
                    let last = Span::raw(line_chars[b + 1..].iter().collect::<String>());

                    line.spans = vec![pre, a_span, mid, b_span, last];
                }
            }
            Operation::Insert(i) => {
                for line in text.lines.iter_mut() {
                    let line_content = line.spans[0].content.clone();
                    let line_chars = line_content.chars().into_iter().collect::<Vec<char>>();
                    let pre = Span::raw(line_chars[..i].iter().collect::<String>());
                    let span = Span::raw(line_chars[i..i + 1].iter().collect::<String>())
                        .fg(Color::LightYellow);
                    let last = Span::raw(line_chars[i + 1..].iter().collect::<String>());

                    line.spans = vec![pre, span, last];
                }
            }
            _ => {}
        }
        return text;
    }

    fn tick(&mut self) {
        if self.auto_next {
            let current_duration = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
            if self.last_tick + self.tick_rate < current_duration {
                self.last_tick = current_duration;
                self.status.as_ref().step_next();
            }
        }
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
    index: Mutex<usize>,
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
            index: Mutex::new(0),
        };
    }

    fn step_next(&self) {
        let operations_len = self.operations.lock().unwrap().len();
        if operations_len == 0 {
            return;
        }
        let mut index = self.index.lock().unwrap();
        if *index < operations_len - 1 {
            *index.deref_mut() = *index + 1;
        }
    }

    fn step_prev(&self) {
        let mut index = self.index.lock().unwrap();
        if *index > 0 {
            *index.deref_mut() = *index - 1;
        }
    }

    fn step_info(&self) -> (usize, Operation) {
        let index = self.index.lock().unwrap();
        let (operation, _) = self.operations.lock().unwrap().index(*index).clone();
        return (*index, operation);
    }
}

impl AlgorithmContext for AlgorithmStatus {
    fn next(&self, operation: Operation, nums: Vec<i32>) {
        self.operations.lock().unwrap().push((operation, nums));
    }
}

fn main() -> io::Result<()> {
    enable_raw_mode()?;
    stdout().execute(EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout());
    let mut terminal = Terminal::new(backend)?;

    let app = App::new(get_algorithms());
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
                let action = handle_key_events(key, &mut app, terminal.size()?);
                match action {
                    Action::Quit => {
                        return io::Result::Ok(());
                    }
                    _ => {}
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
                    ListItem::new(lines).style(Style::default().fg(Color::White))
                })
                .collect();

            let width = WIDTH + 2;
            let height = HEIGHT * 2 + 2;
            let area_option = center_area(width, height, frame.size());
            if area_option.is_none() {
                return;
            }
            let area = area_option.unwrap();

            let list = widgets::List::new(list_items)
                .block(Block::default().borders(Borders::ALL).border_type(Rounded))
                .highlight_style(
                    Style::default()
                        .bg(Color::DarkGray)
                        .add_modifier(Modifier::BOLD),
                );

            frame.render_stateful_widget(list, area, &mut app.list.state);
        }
        Some(algorithm) => {
            algorithm.tick();

            let blocks_width = algorithm.size.0 + 2;
            let blocks_height = algorithm.size.1 + 2;
            let area_option = center_area(blocks_width, blocks_height, frame.size());
            if area_option.is_none() {
                return;
            }
            let area = area_option.unwrap();

            let text = algorithm.display_text();
            let paragraph = Paragraph::new(text).alignment(Alignment::Center).block(
                Block::default()
                    .border_type(Rounded)
                    .borders(Borders::ALL)
                    .title(algorithm.status.name.clone())
                    .title_alignment(Alignment::Left),
            );
            frame.render_widget(paragraph, area);

            if !algorithm.auto_next {
                let (step, operation) = algorithm.status.step_info();
                let info = format!("step: {}\n{}", step, operation.adjusted());
                let text_info = Text::from(info);
                let paragraph_info = Paragraph::new(text_info).alignment(Alignment::Left);
                let next_area = next_area_vertical(area, 2, 1);
                frame.render_widget(paragraph_info, next_area);

                return;
            }
        }
    }
}

enum Action {
    Tick,
    Quit,
}

fn handle_key_events(key: KeyEvent, app: &mut App, size: Rect) -> Action {
    if key.kind == KeyEventKind::Press {
        match &mut app.algorithm {
            None => match key.code {
                KeyCode::Char('c') if key.modifiers.contains(KeyModifiers::CONTROL) => {
                    return Action::Quit
                }
                KeyCode::Left | KeyCode::Char('h') => app.list.unselect(),
                KeyCode::Down | KeyCode::Char('j') => app.list.next(),
                KeyCode::Up | KeyCode::Char('k') => app.list.previous(),
                KeyCode::Enter => {
                    if let Some(i) = app.list.state.selected() {
                        let name = app.list.items[i];
                        let algorithm =
                            AlgorithmUI::new(name.to_string(), size, Duration::from_millis(200))
                                .unwrap();
                        let status = algorithm.status.clone();
                        let algorithm_func = get_algorithm_func(name);
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
            Some(algorithm_ui) => match key.code {
                KeyCode::Char('c') if key.modifiers.contains(KeyModifiers::CONTROL) => {
                    return Action::Quit
                }
                KeyCode::Esc => app.algorithm = None,
                KeyCode::Right => algorithm_ui.status.as_ref().step_next(),
                KeyCode::Left => {
                    if !algorithm_ui.auto_next {
                        algorithm_ui.status.as_ref().step_prev()
                    }
                }
                KeyCode::Char(' ') => algorithm_ui.auto_next = !algorithm_ui.auto_next,
                _ => {}
            },
        }
    }
    return Action::Tick;
}

fn center_area(width: u16, height: u16, s: Rect) -> Option<Rect> {
    if s.width < width || s.height < height {
        return None;
    }
    let x_position = (s.width - width) / 2;
    let y_position = (s.height - height) / 2;
    Some(Rect::new(x_position, y_position, width, height))
}

fn next_area_vertical(s: Rect, height: u16, width_padding: u16) -> Rect {
    Rect::new(
        s.x + width_padding,
        s.y + s.height,
        s.width - width_padding * 2,
        height,
    )
}

#[cfg(test)]
mod tests {
    use std::borrow::Borrow;

    use super::*;

    #[test]
    fn test_block_strings() {
        let blocks = block_strings(10);
        assert!(blocks.contains(BLOCK_QUARTER.to_string().borrow()));
        assert!(blocks.contains(BLOCK_HALF.to_string().borrow()));
        assert!(blocks.contains(BLOCK_HALF_QUARTER.to_string().borrow()));
        assert!(blocks.contains(BLOCK_FULL.to_string().borrow()));
        assert!(blocks.contains(
            format!("{}{}", BLOCK_FULL, BLOCK_QUARTER)
                .to_string()
                .borrow()
        ));
        assert!(blocks.contains(format!("{}{}", BLOCK_FULL, BLOCK_HALF).to_string().borrow()));
        assert!(blocks.contains(
            format!("{}{}", BLOCK_FULL, BLOCK_HALF_QUARTER)
                .to_string()
                .borrow()
        ));
        assert!(blocks.contains(format!("{}{}", BLOCK_FULL, BLOCK_FULL).to_string().borrow()));
        assert!(blocks.contains(
            format!("{}{}{}", BLOCK_FULL, BLOCK_FULL, BLOCK_QUARTER)
                .to_string()
                .borrow()
        ));
        assert!(blocks.contains(
            format!("{}{}{}", BLOCK_FULL, BLOCK_FULL, BLOCK_HALF)
                .to_string()
                .borrow()
        ));
    }

    #[test]
    fn test_blocks_size() {
        let (w, h) = blocks_size(Rect::new(0, 0, 64, 64)).unwrap();
        assert!(w == 32);
        assert!(h == 8);
    }

    #[test]
    #[should_panic(expected = "width is too small")]
    fn test_blocks_size_width_too_small() {
        blocks_size(Rect::new(0, 0, 8, 64)).unwrap();
    }

    #[test]
    #[should_panic(expected = "height is too small")]
    fn test_blocks_size_height_too_small() {
        blocks_size(Rect::new(0, 0, 64, 4)).unwrap();
    }

    #[test]
    fn test_center_area() {
        let area_option = center_area(32, 8, Rect::new(0, 0, 128, 32));
        assert!(area_option.is_some());
        let area = area_option.unwrap();
        assert_eq!(area.x, 48);
        assert_eq!(area.y, 12);
        assert_eq!(area.width, 32);
        assert_eq!(area.height, 8);
    }
}
