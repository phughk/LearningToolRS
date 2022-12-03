mod cli_args;
mod error;
mod module_browser;

use crossterm::{
  event::{DisableMouseCapture, EnableMouseCapture},
  execute,
  terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use std::{io, thread, time::Duration};
use tracing::info;
use tui::{
  backend::CrosstermBackend,
  widgets::{Block, Borders},
  Terminal,
};

fn main() -> Result<(), error::Error> {
  tracing_subscriber::fmt::init();
  let app_args = cli_args::process_args();
  let app_args_str = format!("{app_args:?}");
  info!(app_args = app_args_str, "The app args are logged");
  let modules = module_browser::list_modules("./data")?;
  let modules_str = format!("{modules:?}");
  info!(modules_str, "processed modules");
  // setup terminal
  enable_raw_mode()?;
  let mut stdout = io::stdout();
  execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
  let backend = CrosstermBackend::new(stdout);
  let mut terminal = Terminal::new(backend)?;

  terminal.draw(|f| {
    let size = f.size();
    let block = Block::default()
      .title("Block")
      .borders(Borders::ALL);
    f.render_widget(block, size);
  })?;

  thread::sleep(Duration::from_millis(500));

  // restore terminal
  disable_raw_mode()?;
  execute!(terminal.backend_mut(), LeaveAlternateScreen, DisableMouseCapture)?;
  terminal.show_cursor()?;

  Ok(())
}
