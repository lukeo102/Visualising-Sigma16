use std::io;
use tui;

struct Tui {
    terminal: tui::Terminal<tui::backend::CrosstermBackend<io::Stdout>>,
}

impl Tui {
    pub fn new() -> Option<Tui> {
        let stdout = io::stdout();
        let backend = tui::backend::CrosstermBackend::new(stdout);
        if let Ok(terminal) = tui::Terminal::new(backend) {
            Some(Tui { terminal })
        } else {
            None
        }
    }

    pub fn draw() {}
}
