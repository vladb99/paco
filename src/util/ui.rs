use std::io;
use termion::event::Key;
use termion::input::TermRead;
use termion::{input::MouseTerminal, raw::IntoRawMode, screen::AlternateScreen};
use tui::{
    backend::TermionBackend,
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    symbols,
    widgets::{Axis, Block, Borders, Chart, Dataset, GraphType, Paragraph, Text},
    Terminal,
};
#[derive(Clone)]
pub struct PlotData {
    pub chart_title: String,
    pub chart_x_title: String,
    pub chart_y_title: String,
    pub paragraph_title: String,
    pub graphdata: Vec<GraphData>,
    pub layout: Vec<Rect>,
}
#[derive(Clone)]
pub struct GraphData {
    pub label: String,
    pub color: Color,
    pub data: Vec<(f64, f64)>,
}
pub struct UI {
    stdin: io::Stdin,
    terminal: tui::terminal::Terminal<
        TermionBackend<
            termion::screen::AlternateScreen<
                termion::input::MouseTerminal<termion::raw::RawTerminal<std::io::Stdout>>,
            >,
        >,
    >,
    pub layout_core_affinity: Vec<Rect>,
    pub layout_runtime: Vec<Rect>,
}
impl UI {
    pub fn new() -> UI {
        let stdin = io::stdin();
        let stdout = io::stdout().into_raw_mode().unwrap();
        let stdout = MouseTerminal::from(stdout);
        let stdout = AlternateScreen::from(stdout);
        let backend = TermionBackend::new(stdout);
        let mut terminal = Terminal::new(backend).unwrap();
        terminal.hide_cursor().unwrap();

        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Ratio(1, 3), Constraint::Ratio(2, 3)].as_ref())
            .split(terminal.size().unwrap());
        let chunks_core_affinity = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Percentage(75), Constraint::Percentage(25)].as_ref())
            .split(chunks[0]);
        let chunks_runtime = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Percentage(75), Constraint::Percentage(25)].as_ref())
            .split(chunks[1]);

        UI {
            stdin: stdin,
            terminal: terminal,
            layout_core_affinity: chunks_core_affinity,
            layout_runtime: chunks_runtime,
        }
    }
    pub fn plot(mut self, vec_plotdata: Vec<PlotData>) {
        let mut scroll: usize = 0;
        let mut once = true;
        self.terminal.clear().unwrap();
        println!("press q to quit");
        println!("up and down to scroll");
        for c in self.stdin.keys() {
            if once {
                self.terminal.clear().unwrap();
                once = false;
            }
            self.terminal
                .draw(|mut f| {
                    for plotdata in vec_plotdata.clone() {
                        let mut datasets: Vec<Dataset> = Vec::new();
                        let mut labels: Vec<Text> = Vec::new();
                        let mut xy_min_max = ((100000., 0.), (100000., 0.));
                        for i in 0..plotdata.graphdata.len() {
                            let mut color = plotdata.graphdata[i].color;
                            if i == scroll % plotdata.graphdata.len(){
                                color = Color::Red;
                            }
                            datasets.push(
                                Dataset::default()
                                    .name(plotdata.graphdata[i].label.clone())
                                    .marker(symbols::Marker::Braille)
                                    .style(Style::default().fg(color))
                                    .graph_type(GraphType::Line)
                                    .data(&plotdata.graphdata[i].data),
                            );
                            xy_min_max =
                                axes_bounding(xy_min_max, plotdata.graphdata[i].data.clone());
                            labels.push(Text::styled(
                                plotdata.graphdata[i].label.clone() + "\n",
                                Style::default().fg(color),
                            ));
                        }
                        let x_axes_label = vec![
                            ((xy_min_max.0).0).to_string(),
                            "".to_string(),
                            ((xy_min_max.0).1).to_string(),
                        ];
                        let y_axes_label = vec![
                            ((xy_min_max.1).0).to_string(),
                            "".to_string(),
                            ((xy_min_max.1).1).to_string(),
                        ];
                        let chart = Chart::default()
                            .block(
                                Block::default()
                                    .title(plotdata.chart_title.as_str())
                                    .title_style(
                                        Style::default().fg(Color::Cyan).modifier(Modifier::BOLD),
                                    )
                                    .borders(Borders::ALL),
                            )
                            .x_axis(
                                Axis::default()
                                    .title(plotdata.chart_x_title.as_str())
                                    .style(Style::default().fg(Color::Gray))
                                    .labels_style(Style::default().modifier(Modifier::ITALIC))
                                    .bounds([(xy_min_max.0).0, (xy_min_max.0).1])
                                    .labels(&x_axes_label),
                            )
                            .y_axis(
                                Axis::default()
                                    .title(plotdata.chart_y_title.as_str())
                                    .style(Style::default().fg(Color::Gray))
                                    .labels_style(Style::default().modifier(Modifier::ITALIC))
                                    .bounds([(xy_min_max.1).0, (xy_min_max.1).1])
                                    .labels(&y_axes_label),
                            )
                            .datasets(&datasets);
                        f.render_widget(chart, plotdata.layout[0]);
                        let block = Block::default()
                            .borders(Borders::ALL)
                            .title_style(Style::default().modifier(Modifier::BOLD));
                        let paragraph = Paragraph::new(labels.iter())
                            .block(block.title(plotdata.paragraph_title.as_str()))
                            .alignment(Alignment::Left)
                            .scroll(scroll as u16 % 1000)
                            .wrap(true);
                        f.render_widget(paragraph, plotdata.layout[1]);
                    }
                })
                .unwrap();
            match c.unwrap() {
                Key::Char('q') => break,
                Key::Esc => break,
                Key::Up => {
                    if scroll != 0 {
                        scroll -= 1;
                    }
                }
                Key::Down => scroll += 1,
                _ => {}
            }
        }
    }
}
fn axes_bounding(
    mut xy_min_max: ((f64, f64), (f64, f64)),
    points: Vec<(f64, f64)>,
) -> ((f64, f64), (f64, f64)) {
    for (px, py) in points {
        if px < (xy_min_max.0).0 {
            (xy_min_max.0).0 = px
        };
        if px > (xy_min_max.0).1 {
            (xy_min_max.0).1 = px
        };
        if py < (xy_min_max.1).0 {
            (xy_min_max.1).0 = py
        };
        if py > (xy_min_max.1).1 {
            (xy_min_max.1).1 = py
        };
    }
    xy_min_max
}
