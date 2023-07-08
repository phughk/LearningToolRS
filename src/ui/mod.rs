mod module_browser;

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
use crate::ui::module_browser::draw_module_browser;

const MENU_BG: Color = Color::Yellow;
const MENU_FG: Color = Color::Black;
const SELECT_BG: Color = Color::Black;
const SELECT_FG: Color = Color::Cyan;

pub enum UIState {
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

