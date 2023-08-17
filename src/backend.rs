use crossterm::{
    event::{DisableMouseCapture, EnableMouseCapture},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::prelude::*;
use std::time::Duration;
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
    loop {
        sys.update_info();
        terminal.draw(|f| ui::draw(f, &sys))?;
        std::thread::sleep(tick_rate);
    }
}
