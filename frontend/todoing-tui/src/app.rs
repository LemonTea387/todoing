use crossterm::event::{self, KeyCode, KeyEventKind};
use ratatui::{
    prelude::{Backend, Constraint, Direction, Layout},
    style::{Style, Stylize, Color},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph},
    Terminal,
};

use crate::theme::{BLACK, DARK_GRAY};

pub struct App {
    quit: bool,
    index_selected: usize,
}

impl App {
    pub fn new() -> Self {
        Self { quit: false, index_selected:0 }
    }
    pub fn run<B: Backend>(&mut self, terminal: &mut Terminal<B>) -> anyhow::Result<()> {
        while !self.quit {
            terminal.draw(|frame| {
                let area = frame.size();
                let layout = Layout::default()
                    .direction(Direction::Vertical)
                    .constraints(vec![
                        Constraint::Percentage(35),
                        Constraint::Percentage(65),
                        Constraint::Length(1),
                    ])
                    .split(area);

                // Selection
                let task_view_layout = Layout::default()
                    .direction(Direction::Horizontal)
                    .constraints(vec![Constraint::Percentage(75), Constraint::Percentage(25)])
                    .split(layout[0]);
                frame.render_widget(
                    Paragraph::new("Hello Ratatui! (press 'q' to quit)")
                        .block(Block::default().borders(Borders::ALL))
                        .white(),
                    task_view_layout[0],
                );
                frame.render_widget(
                    Paragraph::new("Hello Ratatui! (press 'q' to quit)")
                        .block(Block::default().borders(Borders::ALL))
                        .white(),
                    task_view_layout[1],
                );

                // Main Area
                frame.render_widget(
                    Paragraph::new("Hello Ratatui! (press 'q' to quit)")
                        .block(Block::default().borders(Borders::ALL))
                        .white(),
                    layout[1],
                );

                // Keybindings
                let keys = [
                    ("H/←", "Left"),
                    ("L/→", "Right"),
                    ("K/↑", "Up"),
                    ("J/↓", "Down"),
                    ("/", "Mark Complete/Unmark"),
                    ("Q", "Quit"),
                ];
                let spans = keys
                    .iter()
                    .flat_map(|(key, desc)| {
                        let key = Span::styled(
                            format!(" {} ", key),
                            Style::new().fg(BLACK).bg(DARK_GRAY),
                        );
                        let desc = Span::styled(
                            format!(" {} ", desc),
                            Style::new().fg(DARK_GRAY).bg(BLACK),
                        );
                        [key, desc]
                    })
                    .collect::<Vec<_>>();
                frame.render_widget(
                    Line::from(spans)
                        .centered()
                        .style((Color::Indexed(236), Color::Indexed(232))),
                    layout[2],
                );
            })?;

            if event::poll(std::time::Duration::from_millis(16))? {
                if let event::Event::Key(key) = event::read()? {
                    if key.kind == KeyEventKind::Press && key.code == KeyCode::Char('q') {
                        self.quit = true;
                    }
                }
            }
        }
        Ok(())
    }
}
