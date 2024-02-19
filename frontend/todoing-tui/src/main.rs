use app::App;
use tui::Tui;

mod tui;
mod app;
mod theme;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let mut tui = Tui::new()?;
    tui.enter()?;
    App::new().run(&mut tui.terminal)?;
    tui.exit()
}
