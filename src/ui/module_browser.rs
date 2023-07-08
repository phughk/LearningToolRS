use std::io::Stdout;
use tui::backend::CrosstermBackend;
use tui::Frame;
use tui::layout::{Constraint, Direction, Layout};
use tui::style::Style;
use tui::widgets::{Block, Borders, Paragraph, Row, Cell, Table};
use crate::module_browser::LearningModule;
use crate::ui::{MENU_BG, MENU_FG, SELECT_BG, SELECT_FG, UIState};

pub(crate) fn draw_module_browser<'a>(modules: &'a Vec<LearningModule>, state: UIState) -> impl FnOnce(&'_ mut Frame<'_, CrosstermBackend<Stdout>>) -> () + 'a {
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
