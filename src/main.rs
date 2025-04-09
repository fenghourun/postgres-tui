mod app;
mod postgres;
mod ui;
mod widgets;

use crate::app::App;
use crate::ui::draw;
use cli_log::init_cli_log;

use crossterm::{
    event::{DisableMouseCapture, EnableMouseCapture},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{backend::CrosstermBackend, Terminal};
use std::{error::Error, io};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    run().await
}

pub async fn run() -> Result<(), Box<dyn Error>> {
    init_cli_log!();

    // setup terminal
    enable_raw_mode()?;

    let mut stdout = io::stdout();

    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;

    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let mut app = App::new().await?;

    let res = run_loop(&mut terminal, &mut app).await;

    if let Err(err) = res {
        disable_raw_mode()?;
        println!("{:?}", err)
    }

    // restore terminal
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;

    terminal.show_cursor()?;

    Ok(())
}

async fn run_loop(
    terminal: &mut Terminal<CrosstermBackend<io::Stdout>>,
    app: &mut App,
) -> io::Result<()> {
    loop {
        terminal.draw(|f| draw(f, app)).expect("Failed to draw");

        app.register_keybinds().await?;

        if app.should_quit {
            return Ok(());
        }
    }
}
