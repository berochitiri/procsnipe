use anyhow::Result;
use clap::Parser;
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode},
};
use ratatui::{
    Frame, Terminal,
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, List, ListItem, ListState, Paragraph},
};
use std::time::{Duration, Instant};
use sysinfo::{Pid, ProcessesToUpdate, System};

#[cfg(feature = "tray")]
mod tray;

/// procsnipe - TUI Process Manager for Windows
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Run in system tray mode (background monitoring)
    #[arg(long)]
    tray: bool,
}

#[derive(PartialEq)]
enum Mode {
    Normal,
    Search,
    Help,
}

#[derive(PartialEq, Clone, Copy, Debug)]
enum SortMode {
    Name,
    Cpu,
    Memory,
}

struct ProcessInfo {
    pid: u32,
    name: String,
    cpu_usage: f32,
    memory: u64,
    is_game: bool,
}

struct App {
    mode: Mode,
    processes: Vec<ProcessInfo>,
    list_state: ListState,
    search_query: String,
    sort_mode: SortMode,
    show_games_only: bool,
    refresh_rate: Duration,
    last_refresh: Instant,
    sys: System,
}

impl App {
    fn new() -> Self {
        let mut list_state = ListState::default();
        list_state.select(Some(0));

        Self {
            mode: Mode::Normal,
            processes: Vec::new(),
            list_state,
            search_query: String::new(),
            sort_mode: SortMode::Cpu,
            show_games_only: false,
            refresh_rate: Duration::from_millis(1000),
            last_refresh: Instant::now(),
            sys: System::new_all(),
        }
    }

    fn refresh_processes(&mut self) {
        if self.last_refresh.elapsed() < self.refresh_rate {
            return;
        }

        self.sys.refresh_processes(ProcessesToUpdate::All, true);

        self.processes = self
            .sys
            .processes()
            .iter()
            .map(|(pid, process)| {
                let name = process.name().to_string_lossy().to_string();
                let is_game = Self::is_game_process(&name);

                ProcessInfo {
                    pid: pid.as_u32(),
                    name,
                    cpu_usage: process.cpu_usage(),
                    memory: process.memory(),
                    is_game,
                }
            })
            .collect();

        // Apply filters
        if self.show_games_only {
            self.processes.retain(|p| p.is_game);
        }

        if !self.search_query.is_empty() {
            let query = self.search_query.to_lowercase();
            self.processes
                .retain(|p| p.name.to_lowercase().contains(&query));
        }

        // Sort
        match self.sort_mode {
            SortMode::Name => self.processes.sort_by(|a, b| a.name.cmp(&b.name)),
            SortMode::Cpu => self
                .processes
                .sort_by(|a, b| b.cpu_usage.partial_cmp(&a.cpu_usage).unwrap()),
            SortMode::Memory => self.processes.sort_by(|a, b| b.memory.cmp(&a.memory)),
        }

        self.last_refresh = Instant::now();
    }

    fn is_game_process(name: &str) -> bool {
        let game_indicators = [
            "game",
            "steam",
            "epic",
            "uplay",
            "origin",
            "riot",
            "valorant",
            "league",
            "csgo",
            "cs2",
            "dota",
            "apex",
            "fortnite",
            "minecraft",
            "roblox",
            "gta",
            "unity",
            "unreal",
            "dx11",
            "dx12",
            "vulkan",
        ];

        let name_lower = name.to_lowercase();
        game_indicators
            .iter()
            .any(|&indicator| name_lower.contains(indicator))
    }

    fn kill_selected(&mut self) -> Result<()> {
        if let Some(selected) = self.list_state.selected() {
            if let Some(process) = self.processes.get(selected) {
                let pid = Pid::from_u32(process.pid);
                if let Some(proc) = self.sys.process(pid) {
                    // Use kill_with for better Windows compatibility
                    // ProcessSignal::Kill is more forceful and works better on Windows
                    if !proc.kill() {
                        // Process might require admin privileges or be protected
                        // Silently fail rather than crashing
                    }
                }
            }
        }
        Ok(())
    }

    fn next(&mut self) {
        if self.processes.is_empty() {
            return;
        }
        let i = match self.list_state.selected() {
            Some(i) => {
                if i >= self.processes.len() - 1 {
                    0
                } else {
                    i + 1
                }
            }
            None => 0,
        };
        self.list_state.select(Some(i));
    }

    fn previous(&mut self) {
        if self.processes.is_empty() {
            return;
        }
        let i = match self.list_state.selected() {
            Some(i) => {
                if i == 0 {
                    self.processes.len() - 1
                } else {
                    i - 1
                }
            }
            None => 0,
        };
        self.list_state.select(Some(i));
    }

    fn toggle_sort(&mut self) {
        self.sort_mode = match self.sort_mode {
            SortMode::Name => SortMode::Cpu,
            SortMode::Cpu => SortMode::Memory,
            SortMode::Memory => SortMode::Name,
        };
    }
}

#[cfg(target_os = "windows")]
fn is_elevated() -> bool {
    // Simple check: elevated processes typically have specific privileges
    // We can check if we can access protected system info
    use std::env;

    // Check if running with admin rights by checking temp path
    // Admin processes have access to system-wide temp
    if let Ok(temp) = env::var("TEMP") {
        // If we can write to system32, we're probably elevated
        // But this is a simple heuristic - elevated sessions often have specific env vars
        return !temp.contains("AppData\\Local");
    }
    false
}

fn main() -> Result<()> {
    // Parse command line arguments
    let args = Args::parse();

    // Launch in tray mode if --tray flag is set
    #[cfg(feature = "tray")]
    if args.tray {
        let mut tray_app = tray::TrayApp::new();
        return tray_app.run();
    }

    // Otherwise, launch normal TUI mode
    // Check if running on Windows and warn about admin privileges
    #[cfg(target_os = "windows")]
    {
        if !is_elevated() {
            eprintln!("⚠️  Warning: Not running as administrator.");
            eprintln!("   Some processes might be protected and can't be killed.");
            eprintln!("   For full functionality, run as admin.\n");
            std::thread::sleep(Duration::from_secs(2));
        }
    }

    // Setup terminal
    enable_raw_mode()?;
    let mut stdout = std::io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // Create app
    let mut app = App::new();
    app.refresh_processes();

    // Main loop
    let res = run_app(&mut terminal, &mut app);

    // Restore terminal
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    if let Err(err) = res {
        println!("Error: {:?}", err);
    }

    Ok(())
}

fn run_app<B: ratatui::backend::Backend>(terminal: &mut Terminal<B>, app: &mut App) -> Result<()> {
    let mut last_key_time = Instant::now();
    let debounce_duration = Duration::from_millis(150); // Prevent double-clicks

    loop {
        app.refresh_processes();
        terminal.draw(|f| ui(f, app))?;

        if event::poll(Duration::from_millis(100))? {
            if let Event::Key(key) = event::read()? {
                // Only process Press events, ignore Release and Repeat to prevent double-clicks
                if key.kind != event::KeyEventKind::Press {
                    continue;
                }

                // Debounce to prevent accidental double-presses (except for navigation)
                let is_navigation = matches!(
                    key.code,
                    KeyCode::Char('j') | KeyCode::Char('k') | KeyCode::Up | KeyCode::Down
                );
                if !is_navigation && last_key_time.elapsed() < debounce_duration {
                    continue;
                }
                last_key_time = Instant::now();

                match app.mode {
                    Mode::Normal => match key.code {
                        KeyCode::Char('q') => return Ok(()),
                        KeyCode::Char('?') => app.mode = Mode::Help,
                        KeyCode::Char('/') => {
                            app.mode = Mode::Search;
                            app.search_query.clear();
                        }
                        KeyCode::Char('j') | KeyCode::Down => app.next(),
                        KeyCode::Char('k') | KeyCode::Up => app.previous(),
                        KeyCode::Char('d') => {
                            app.kill_selected()?;
                        }
                        KeyCode::Char('g') => app.show_games_only = !app.show_games_only,
                        KeyCode::Char('s') => app.toggle_sort(),
                        _ => {}
                    },
                    Mode::Search => match key.code {
                        KeyCode::Esc => {
                            app.mode = Mode::Normal;
                            app.search_query.clear();
                        }
                        KeyCode::Char(c) => app.search_query.push(c),
                        KeyCode::Backspace => {
                            app.search_query.pop();
                        }
                        KeyCode::Enter => app.mode = Mode::Normal,
                        _ => {}
                    },
                    Mode::Help => {
                        if key.code == KeyCode::Esc || key.code == KeyCode::Char('?') {
                            app.mode = Mode::Normal;
                        }
                    }
                }
            }
        }
    }
}

fn ui(f: &mut Frame, app: &mut App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),
            Constraint::Min(0),
            Constraint::Length(3),
        ])
        .split(f.area());

    // Header
    let header = Paragraph::new(vec![Line::from(vec![
        Span::styled(
            "🎯 procsnipe ",
            Style::default().fg(Color::Red).add_modifier(Modifier::BOLD),
        ),
        Span::styled("| ", Style::default().fg(Color::DarkGray)),
        Span::styled(
            format!("processes: {} ", app.processes.len()),
            Style::default().fg(Color::Cyan),
        ),
        Span::styled("| ", Style::default().fg(Color::DarkGray)),
        Span::styled(
            format!("sort: {:?} ", app.sort_mode),
            Style::default().fg(Color::Yellow),
        ),
        Span::styled("| ", Style::default().fg(Color::DarkGray)),
        Span::styled(
            if app.show_games_only {
                "games only"
            } else {
                "all"
            },
            Style::default().fg(Color::Magenta),
        ),
    ])])
    .block(
        Block::default()
            .borders(Borders::ALL)
            .border_style(Style::default().fg(Color::Red)),
    );
    f.render_widget(header, chunks[0]);

    // Process list or help
    if app.mode == Mode::Help {
        let help_text = vec![
            Line::from(""),
            Line::from(Span::styled(
                "🎯 procsnipe controls",
                Style::default().fg(Color::Red).add_modifier(Modifier::BOLD),
            )),
            Line::from(""),
            Line::from(vec![
                Span::styled("  /  ", Style::default().fg(Color::Yellow)),
                Span::raw("- Search/Filter processes"),
            ]),
            Line::from(vec![
                Span::styled("  ESC", Style::default().fg(Color::Yellow)),
                Span::raw("- Exit search/help"),
            ]),
            Line::from(vec![
                Span::styled("  ?  ", Style::default().fg(Color::Yellow)),
                Span::raw("- Toggle this help"),
            ]),
            Line::from(vec![
                Span::styled("  q  ", Style::default().fg(Color::Yellow)),
                Span::raw("- Quit"),
            ]),
            Line::from(vec![
                Span::styled("  j/k", Style::default().fg(Color::Yellow)),
                Span::raw("- Navigate (or ↑/↓)"),
            ]),
            Line::from(vec![
                Span::styled("  d  ", Style::default().fg(Color::Yellow)),
                Span::raw("- Kill selected process"),
            ]),
            Line::from(vec![
                Span::styled("  g  ", Style::default().fg(Color::Yellow)),
                Span::raw("- Toggle game-only view"),
            ]),
            Line::from(vec![
                Span::styled("  s  ", Style::default().fg(Color::Yellow)),
                Span::raw("- Cycle sort (Name/CPU/Memory)"),
            ]),
            Line::from(""),
            Line::from(Span::styled(
                "press ? or ESC to close",
                Style::default().fg(Color::DarkGray),
            )),
        ];
        let help = Paragraph::new(help_text).block(
            Block::default()
                .borders(Borders::ALL)
                .title("Help")
                .border_style(Style::default().fg(Color::Yellow)),
        );
        f.render_widget(help, chunks[1]);
    } else {
        let items: Vec<ListItem> = app
            .processes
            .iter()
            .map(|p| {
                let cpu_bar = format!("{:>5.1}%", p.cpu_usage);
                let mem_mb = p.memory / 1024 / 1024;
                let mem_str = format!("{:>6} MB", mem_mb);

                let style = if p.is_game {
                    Style::default().fg(Color::Green)
                } else if p.cpu_usage > 50.0 {
                    Style::default().fg(Color::Red)
                } else if p.cpu_usage > 20.0 {
                    Style::default().fg(Color::Yellow)
                } else {
                    Style::default().fg(Color::White)
                };

                let content = Line::from(vec![
                    Span::styled(
                        format!("{:<8} ", p.pid),
                        Style::default().fg(Color::DarkGray),
                    ),
                    Span::styled(format!("{:<30} ", p.name), style),
                    Span::styled(cpu_bar, Style::default().fg(Color::Cyan)),
                    Span::raw("  "),
                    Span::styled(mem_str, Style::default().fg(Color::Magenta)),
                ]);

                ListItem::new(content)
            })
            .collect();

        let list = List::new(items)
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .title("Processes")
                    .border_style(Style::default().fg(Color::Cyan)),
            )
            .highlight_style(
                Style::default()
                    .bg(Color::DarkGray)
                    .add_modifier(Modifier::BOLD),
            )
            .highlight_symbol(">> ");

        f.render_stateful_widget(list, chunks[1], &mut app.list_state);
    }

    // Footer
    let footer_text = match app.mode {
        Mode::Normal => "press ? for help | q to quit",
        Mode::Search => &format!("search: {}_", app.search_query),
        Mode::Help => "viewing help",
    };

    let footer = Paragraph::new(footer_text)
        .style(Style::default().fg(Color::DarkGray))
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_style(Style::default().fg(Color::DarkGray)),
        );
    f.render_widget(footer, chunks[2]);
}
