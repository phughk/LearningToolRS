use crate::error::Error;
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

enum AppWindow {
  ModuleBrowser,
  QuizSetup,
  QuizScore,
  Quiz,
}

pub fn terminal_loop() -> Result<(), Error> {
  // setup terminal
  enable_raw_mode()?;
  let mut stdout = io::stdout();
  execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
  let backend = CrosstermBackend::new(stdout);
  let mut terminal = Terminal::new(backend)?;

  let app_window_state = AppWindow::ModuleBrowser;

  match app_window_state {
    AppWindow::ModuleBrowser => {
      terminal.draw(draw_module_browser)?;
    }
    _ => return Err(Error::StateError("unrecognised state".to_string())),
  }

  thread::sleep(Duration::from_millis(10_000));

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

  let chunks = Layout::default()
    .direction(Direction::Horizontal)
    .constraints([Constraint::Percentage(50), Constraint::Percentage(50)].as_ref())
    .split(non_banner_layout);
  let (browser_layout, module_info_layout) = (chunks[0], chunks[1]);

  let block = Block::default()
    .title("ModuleBrowser")
    .borders(Borders::ALL);
  f.render_widget(block, banner_layout);

  let block = Block::default()
    .title("Module browsing window")
    .borders(Borders::ALL);
  f.render_widget(block, browser_layout);

  let paragraph = Paragraph::new("This is where module info goes.\nTesting new lines as well.\n\tTabs don't work.").block(
    Block::default()
      .title("Module info window")
      .borders(Borders::ALL),
  );
  f.render_widget(paragraph, module_info_layout);
}
