mod app;
use app::App;

fn main() -> std::io::Result<()> {
    let terminal = ratatui::init();
    let mut buffer = [String::new(), String::new()];
    let mut app = App::new(&mut buffer);
    let result = app.run(terminal);
    ratatui::restore();

    result
}
