pub mod app;

use app::{ui, App, ConnectionType, File, HostInfo};
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use std::{
    error::Error,
    io::{self, Stdout},
    time::{Duration, Instant},
};
use tui::{backend::CrosstermBackend, terminal::Terminal};

type Result<T> = std::result::Result<T, Box<dyn Error>>;

pub fn init(
    tick_rate: Duration,
    connection_type: ConnectionType,
    host_info: HostInfo,
    files: Vec<File>,
) -> Result<()> {
    let mut terminal = setup_crossterm()?;
    let app = create_app(connection_type, host_info, files);

    app_loop(&mut terminal, app, tick_rate)?;

    restore_terminal(terminal)?;

    Ok(())
}

pub fn setup_crossterm() -> Result<Terminal<CrosstermBackend<Stdout>>> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let terminal = Terminal::new(backend)?;

    Ok(terminal)
}

pub fn create_app<'a>(
    connection_type: ConnectionType,
    host_info: HostInfo,
    files: Vec<File>,
) -> App<'a> {
    App::new("leightbox", connection_type, host_info, files)
}

pub fn app_loop(
    terminal: &mut Terminal<CrosstermBackend<Stdout>>,
    mut app: App,
    tick_rate: Duration,
) -> Result<()> {
    let mut last_tick = Instant::now();

    loop {
        // Render
        terminal.draw(|f| ui::draw(f, &mut app))?;

        // Handle inputs
        let timeout = tick_rate
            .checked_sub(last_tick.elapsed())
            .unwrap_or_else(|| Duration::from_secs(0));
        if event::poll(timeout)? {
            if let Event::Key(key) = event::read()? {
                app.match_keycode(key.code);
            }
        }

        if last_tick.elapsed() >= tick_rate {
            //app.on_tick()
            last_tick = Instant::now();
        }

        if app.should_quit {
            break;
        }
    }

    Ok(())
}

pub fn restore_terminal(mut terminal: Terminal<CrosstermBackend<Stdout>>) -> Result<()> {
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    Ok(())
}
