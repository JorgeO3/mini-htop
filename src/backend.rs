use crossterm::event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode};
use crossterm::execute;
use crossterm::terminal::{
    disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen,
};

use ratatui::prelude::*;
use std::time::{Duration, Instant};
use sysinfo::{System, SystemExt};

use crate::prelude::*;
use crate::system_info::SystemInfo;
use crate::ui;

pub fn run(duration: Duration, enhanced_graphics: bool) -> Result<()> {
    // setup terminal
    enable_raw_mode()?;
    let mut stdout = std::io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // Start app
    let sys = System::new_all();
    let sys_resources = SystemInfo::new(sys, enhanced_graphics);
    let res = start_monitor(&mut terminal, sys_resources, duration);

    // restore terminal
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    if let Err(err) = res {
        println!("{err:?}");
    }

    Ok(())
}

pub fn start_monitor<B>(
    terminal: &mut Terminal<B>,
    mut sys: SystemInfo,
    tick_rate: Duration,
) -> Result<()>
where
    B: Backend,
{
    let mut last_tick = Instant::now();
    loop {
        terminal.draw(|f| ui::draw(f, &sys))?;

        let timeout = tick_rate
            .checked_sub(last_tick.elapsed())
            .unwrap_or_else(|| Duration::from_secs(0));

        if crossterm::event::poll(timeout)? {
            if let Event::Key(key) = event::read()? {
                if let KeyCode::Char('q') = key.code {
                    return Ok(());
                }
            }
        }

        if last_tick.elapsed() >= tick_rate {
            sys.update_info();
            last_tick = Instant::now();
        }
    }
}
