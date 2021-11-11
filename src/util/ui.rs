use super::board::*;
use std::io;
use termion::{input::MouseTerminal, raw::IntoRawMode, screen::AlternateScreen};
use tui::{
    backend::TermionBackend,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Style},
    widgets::{Block, Borders},
    Terminal,
};
#[allow(dead_code)]
pub struct UI {
    stdin: Option<io::Stdin>,
    terminal: Option<
        tui::terminal::Terminal<
            TermionBackend<
                termion::screen::AlternateScreen<
                    termion::input::MouseTerminal<termion::raw::RawTerminal<std::io::Stdout>>,
                >,
            >,
        >,
    >,
    cells: Vec<Rect>,
}
impl UI {
    pub fn disabled() -> UI {
        UI {
            stdin: None,
            terminal: None,
            cells: vec![],
        }
    }
    pub fn new() -> UI {
        let stdin = io::stdin();
        let stdout = io::stdout().into_raw_mode().unwrap();
        let stdout = MouseTerminal::from(stdout);
        let stdout = AlternateScreen::from(stdout);
        let backend = TermionBackend::new(stdout);
        let mut terminal = Terminal::new(backend).unwrap();
        terminal.hide_cursor().unwrap();
        let constraints = [Constraint::Ratio(1, 13); 13];
        let colums = Layout::default()
            .margin(1)
            .horizontal_margin(1)
            .vertical_margin(1)
            .direction(Direction::Horizontal)
            .constraints(constraints.as_ref())
            .split(terminal.size().unwrap());

        let mut cells: Vec<Rect> = Vec::new();
        for c in colums {
            cells.extend(
                Layout::default()
                    .margin(1)
                    .horizontal_margin(1)
                    .vertical_margin(1)
                    .direction(Direction::Vertical)
                    .constraints(constraints.as_ref())
                    .split(c),
            );
        }

        UI {
            stdin: Some(stdin),
            terminal: Some(terminal),
            cells: cells,
        }
    }
    pub fn plot(&mut self, board: Board) {
        let cells = self.cells.clone();
        match self.terminal {
            Some(ref mut t) => {
                t.draw(|f| {
                    let dame = Style::default().bg(Color::Green).fg(Color::Black);
                    let even = Style::default().bg(Color::Gray).fg(Color::Black);
                    let odd = Style::default().bg(Color::DarkGray).fg(Color::Black);
                    for (i, cell) in cells.iter().enumerate() {
                        let s = match board.get_index(i).occupied {
                            false => (i + i / 13) % 2,
                            true => 3,
                        };

                        let block = match s {
                            0 => Block::default().style(even).borders(Borders::ALL),
                            1 => Block::default().style(odd).borders(Borders::ALL),
                            _ => Block::default().style(dame).borders(Borders::ALL),
                        };
                        f.render_widget(block, *cell);
                    }
                })
                .unwrap();
            }
            None => return,
        }
    }
}
