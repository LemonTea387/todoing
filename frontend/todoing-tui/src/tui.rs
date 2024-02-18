use std::{
    io::{stdout, Stdout},
    time::Duration,
};

use crossterm::{
    event::{DisableMouseCapture, EnableMouseCapture},
    terminal::{disable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{prelude::CrosstermBackend, Terminal};
use tokio::sync;

pub struct Tui {
    pub terminal: Terminal<CrosstermBackend<Stdout>>,
}

impl Tui {
    pub fn new() -> anyhow::Result<Self> {
        let terminal = Terminal::new(CrosstermBackend::new(stdout()))?;
        Ok(Self { terminal })
    }

    pub fn enter(&self) -> anyhow::Result<()> {
        crossterm::terminal::enable_raw_mode()?;
        crossterm::execute!(std::io::stdout(), EnterAlternateScreen, EnableMouseCapture)?;
        Ok(())
    }

    pub fn exit(&self) -> anyhow::Result<()> {
        crossterm::execute!(stdout(), LeaveAlternateScreen, DisableMouseCapture)?;
        disable_raw_mode()?;
        Ok(())
    }
}

enum Event {
    Key(crossterm::event::KeyEvent),
}

struct EventHandler {
    rx: sync::mpsc::UnboundedReceiver<Event>,
}

impl EventHandler {
    fn new(tick_rate: Duration) -> Self {
        let (tx, rx) = sync::mpsc::unbounded_channel();
        tokio::spawn(async move {
            loop {
                if crossterm::event::poll(tick_rate)? {
                    match crossterm::event::read()? {
                        crossterm::event::Event::Key(e) => tx.send(Event::Key(e)),
                        _ => unimplemented!(),
                    };
                }
            }
            #[allow(unreachable_code)]
            Ok::<(), anyhow::Error>(())
        });
        Self { rx }
    }

    async fn next(&self) -> Option<Event> {
        Ok(self.rx.recv().await)
    }
}
