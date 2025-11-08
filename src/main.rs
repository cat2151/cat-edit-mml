use anyhow::Result;
use crossterm::{
    event::{self, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{
    backend::CrosstermBackend,
    style::Style,
    widgets::{Block, Borders},
    Terminal,
};
use std::io;
use std::process::{Command, Stdio};
use std::time::Duration;
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

    // Playback state
    let mut last_content = String::new();

    // Main event loop
    loop {
        // Draw UI
        terminal.draw(|f| {
            f.render_widget(&textarea, f.area());
        })?;

        // Handle events
        if event::poll(Duration::from_millis(100))? {
            if let Event::Key(key) = event::read()? {
                // ESC to exit
                if key.code == KeyCode::Esc {
                    break;
                }

                // Pass key to textarea
                textarea.input(key);

                // Trigger playback immediately on content change
                let current_content = textarea.lines().join("\n");
                if current_content != last_content && !current_content.trim().is_empty() {
                    // Check if content contains MML notes (cdefgab)
                    if contains_mml_notes(&current_content) {
                        play_mml(&current_content);
                    }
                    last_content = current_content;
                }
            }
        }
    }

    // Cleanup terminal
    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    terminal.show_cursor()?;

    Ok(())
}

/// Check if the content contains MML notes (c, d, e, f, g, a, b)
fn contains_mml_notes(content: &str) -> bool {
    let lowercase = content.to_lowercase();
    lowercase
        .chars()
        .any(|c| matches!(c, 'c' | 'd' | 'e' | 'f' | 'g' | 'a' | 'b'))
}

/// Play MML content using cat-play-mml subprocess
fn play_mml(content: &str) {
    // Try to spawn cat-play-mml as a subprocess with content as argument
    match Command::new("cat-play-mml")
        .arg(content)
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .spawn()
    {
        Ok(_child) => {
            // Note: We don't wait for the child process to complete
            // It will run in the background
        }
        Err(_) => {
            // cat-play-mml not found or failed to start
            // Silently ignore - the editor continues to work
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_contains_mml_notes_with_valid_notes() {
        assert!(contains_mml_notes("cdefgab"));
        assert!(contains_mml_notes("CDEFGAB"));
        assert!(contains_mml_notes("c"));
        assert!(contains_mml_notes("MML: c d e f g a b"));
    }

    #[test]
    fn test_contains_mml_notes_without_notes() {
        assert!(!contains_mml_notes("xyz"));
        assert!(!contains_mml_notes("123"));
        assert!(!contains_mml_notes(""));
        assert!(!contains_mml_notes("hij klmn"));
    }

    #[test]
    fn test_contains_mml_notes_mixed_content() {
        // These contain MML note letters
        assert!(contains_mml_notes("The note C is important"));
        assert!(contains_mml_notes("tempo 120\nc d e"));
        assert!(contains_mml_notes("hello world")); // contains 'd' and 'e'
    }
}
