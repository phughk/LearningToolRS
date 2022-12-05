extern crate text_io;
mod cli_args;
mod error;
mod module_browser;

use crossterm::{
  event::{DisableMouseCapture, EnableMouseCapture},
  execute,
  terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use std::io::Stdout;
use std::{io, thread, time::Duration};
use tracing::info;
use tui::layout::{Constraint, Direction, Layout};
use tui::{
  backend::CrosstermBackend,
  widgets::{Block, Borders},
  Frame, Terminal,
};

fn main() -> Result<(), error::Error> {
  tracing_subscriber::fmt::init();
  let app_args = cli_args::process_args();
  let app_args_str = format!("{app_args:?}");
  info!(app_args = app_args_str, "The app args are logged");
  let modules = module_browser::list_modules("./data")?;
  let modules_str = format!("{modules:?}");
  info!(modules_str, "processed modules");
  terminal_loop()
}

enum AppWindow {
  ModuleBrowser,
  QuizSetup,
  QuizScore,
  Quiz,
}

fn terminal_loop() -> Result<(), error::Error> {
  // setup terminal
  enable_raw_mode()?;
  let mut stdout = io::stdout();
  execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
  let backend = CrosstermBackend::new(stdout);
  let mut terminal = Terminal::new(backend)?;

  let appWindowState = AppWindow::ModuleBrowser;

  match appWindowState {
    AppWindow::ModuleBrowser => {
      terminal.draw(draw_module_browser)?;
    }
    _ => return Err(error::Error::StateError("unrecognised state".to_string())),
  }

  thread::sleep(Duration::from_millis(2000));

  // restore terminal
  disable_raw_mode()?;
  execute!(terminal.backend_mut(), LeaveAlternateScreen, DisableMouseCapture)?;
  terminal.show_cursor()?;

  Ok(())
}

fn draw_module_browser(f: &mut Frame<CrosstermBackend<Stdout>>) {
  let size = f.size();

  let chunks = Layout::default()
    .direction(Direction::Vertical)
    .constraints(
      [
        Constraint::Length(3), // The height of the title bar
        Constraint::Min(3),    // the height of the rest of content
      ]
      .as_ref(),
    )
    .split(size);
  let (banner_layout, non_banner_layout) = (chunks[0], chunks[1]);

  let block = Block::default()
    .title("ModuleBrowser")
    .borders(Borders::ALL);
  f.render_widget(block, banner_layout);

  let block = Block::default()
    .title("Non banner")
    .borders(Borders::ALL);
  f.render_widget(block, non_banner_layout);
}
