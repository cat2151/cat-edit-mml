use anyhow::Result;
use crossterm::{
    event::{self, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{
    backend::CrosstermBackend,
    layout::{Constraint, Layout},
    style::Style,
    widgets::{Block, Borders},
    Terminal,
};
use std::io;
use tui_textarea::TextArea;

fn main() -> Result<()> {
    // Setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // Create text area
    let mut textarea = TextArea::default();
    textarea.set_block(
        Block::default()
            .borders(Borders::ALL)
            .title("MML Editor - Press ESC to exit"),
    );
    textarea.set_cursor_line_style(Style::default());

    // Main event loop
    loop {
        // Draw UI
        terminal.draw(|f| {
            let chunks = Layout::default()
                .constraints([Constraint::Percentage(100)])
                .split(f.area());
            f.render_widget(&textarea, chunks[0]);
        })?;

        // Handle events
        if event::poll(std::time::Duration::from_millis(100))? {
            if let Event::Key(key) = event::read()? {
                // ESC to exit
                if key.code == KeyCode::Esc {
                    break;
                }

                // Pass key to textarea
                textarea.input(key);
            }
        }
    }

    // Cleanup terminal
    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    terminal.show_cursor()?;

    Ok(())
}
