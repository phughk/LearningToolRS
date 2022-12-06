extern crate text_io;
mod cli_args;
mod error;
mod module_browser;
mod ui;

use crossterm::{
  event::{DisableMouseCapture, EnableMouseCapture},
  execute,
  terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use std::io::Stdout;
use std::{io, thread, time::Duration};
use tracing::info;
use tui::layout::Direction::Horizontal;
use tui::layout::{Constraint, Direction, Layout};
use tui::style::{Color, Style};
use tui::text::Span;
use tui::widgets::{Axis, Chart, Dataset, GraphType, Paragraph};
use tui::{
  backend::CrosstermBackend,
  symbols,
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
  ui::terminal_loop(&modules)
}
