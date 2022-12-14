use crate::error::Error;
use crate::module_browser::{LearningModule, LearningModuleEntries, LearningModuleMetadata, Version};
use crossterm::event::{Event, KeyCode, KeyEvent};
use crossterm::{
  event::{DisableMouseCapture, EnableMouseCapture},
  execute,
  terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use std::borrow::BorrowMut;
use std::io::Stdout;
use std::{io, thread, time::Duration, vec};
use tracing::info;
use tui::layout::Direction::Horizontal;
use tui::layout::{Constraint, Direction, Layout};
use tui::style::{Color, Style};
use tui::text::Span;
use tui::widgets::{Axis, Cell, Chart, Dataset, GraphType, Paragraph, Row, Table};
use tui::{
  backend::CrosstermBackend,
  symbols,
  widgets::{Block, Borders},
  Frame, Terminal,
};

const MENU_BG: Color = Color::Yellow;
const MENU_FG: Color = Color::Black;
const SELECT_BG: Color = Color::Black;
const SELECT_FG: Color = Color::Cyan;

enum UIState {
  ModuleBrowser { selected_item: u16 },
  QuizSetup,
  QuizScore,
  Quiz,
}

pub fn terminal_loop(modules: &Vec<LearningModule>) -> Result<(), Error> {
  // setup terminal
  enable_raw_mode()?;
  let mut stdout = io::stdout();
  execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
  let backend = CrosstermBackend::new(stdout);
  let mut terminal = Terminal::new(backend)?;

  let mut app_window_state = UIState::ModuleBrowser { selected_item: 0 };
  let poll_rate = Duration::from_millis(200);
  let mut refresh = true;
  loop {
    if refresh {
      match app_window_state {
        UIState::ModuleBrowser { selected_item } => {
          terminal.draw(draw_module_browser(modules, UIState::ModuleBrowser { selected_item }))?;
        }
        _ => return Err(Error::StateError("unrecognised state".to_string())), // TODO does this break the terminal since reset is not handled?
      }
      refresh = false
    }
    if crossterm::event::poll(poll_rate)? {
      match crossterm::event::read()? {
        Event::FocusGained => {}
        Event::FocusLost => {}
        Event::Key(k) => {
          if k.code == KeyCode::Char('q') {
            break;
          }
          if k.code == KeyCode::Down {
            match app_window_state {
              UIState::ModuleBrowser { selected_item } => {
                let updated = (selected_item + 1).clamp(0, modules.len() as u16 - 1);
                if updated != selected_item {
                  app_window_state = UIState::ModuleBrowser { selected_item: updated };
                  refresh = true;
                }
              }
              _ => {}
            }
          }
          if k.code == KeyCode::Up {
            match app_window_state {
              UIState::ModuleBrowser { selected_item } => {
                let updated = (selected_item - 1).clamp(0, modules.len() as u16 - 1);
                if updated != selected_item {
                  app_window_state = UIState::ModuleBrowser { selected_item: updated };
                  refresh = true;
                }
              }
              _ => {}
            }
          }
        }
        Event::Mouse(_) => {}
        Event::Paste(_) => {}
        Event::Resize(_, _) => {}
      }
    }
  }

  // restore terminal
  disable_raw_mode()?;
  execute!(terminal.backend_mut(), LeaveAlternateScreen, DisableMouseCapture)?;
  terminal.show_cursor()?;

  Ok(())
}

fn draw_module_browser<'a>(modules: &'a Vec<LearningModule>, state: UIState) -> impl FnOnce(&'_ mut Frame<'_, CrosstermBackend<Stdout>>) -> () + 'a {
  return |f| {
    // let mut f = f.borrow_mut();
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

    f.render_widget(module_selector_component(modules, state), browser_layout);

    let paragraph = Paragraph::new("This is where module info goes.\nTesting new lines as well.\n\tTabs don't work.").block(
      Block::default()
        .title("Module info window")
        .borders(Borders::ALL),
    );
    f.render_widget(paragraph, module_info_layout);
  };
}

fn module_selector_component<'a>(modules: &Vec<LearningModule>, state: UIState) -> Table<'a> {
  let mut rows = vec![Row::new(vec![Cell::from("Name"), Cell::from("Description"), Cell::from("Author")]).style(Style::default().bg(MENU_BG).fg(MENU_FG))];
  for i in 0..modules.len() {
    let m = modules[i].clone();
    let cells = vec![Cell::from(m.metadata.name), Cell::from(m.metadata.description), Cell::from(m.metadata.author)];
    let mut row = Row::new(cells);
    match state {
      UIState::ModuleBrowser { selected_item } => {
        if i == selected_item as usize {
          row = row.style(
            Style::default()
              .bg(SELECT_BG)
              .fg(SELECT_FG),
          );
        }
      }
      _ => {
        // TODO this should throw or log
      }
    }
    rows.push(row);
  }
  let table = Table::new(rows)
    .block(
      Block::default()
        .title("Module browsing window")
        .borders(Borders::ALL),
    )
    .widths(&[Constraint::Percentage(20), Constraint::Percentage(60), Constraint::Percentage(20)]);
  return table;
}
