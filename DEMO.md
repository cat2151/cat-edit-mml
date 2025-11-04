# Demo Guide for cat-edit-mml

## Quick Start

```bash
cargo run
```

## Expected Behavior

When you run the editor, you'll see:
- A full-screen terminal interface
- A bordered window with title "MML Editor - Press ESC to exit"
- A text cursor ready for input

## Features Demonstration

### 1. Multi-line Text Input
- Type any text
- Press Enter to create new lines
- The cursor follows your input

### 2. Cursor Movement
- **Arrow Keys** (←↑↓→): Move cursor in any direction
- **Home**: Jump to beginning of current line
- **End**: Jump to end of current line
- **Page Up**: Scroll up one page
- **Page Down**: Scroll down one page

### 3. Text Editing
- **Backspace**: Delete character before cursor
- **Delete**: Delete character at cursor
- **Ctrl+H**: Alternative backspace
- **Any character**: Insert at cursor position

### 4. Exit
- **ESC**: Exit the editor and return to shell

## Example MML Content to Try

Once the audio playback is implemented (see AUDIO_PLAYBACK_PLAN.md), you can try editing MML like:

```
C D E F G A B
```

or

```
T120 L4 O4
C E G > C < G E C
```

## Technical Details

The editor is built using:
- **ratatui 0.29**: For the TUI framework
- **tui-textarea 0.7**: For multi-line text editing
- **crossterm 0.28**: For cross-platform terminal handling

The implementation is in `src/main.rs` and is approximately 60 lines of code.

## Known Limitations

- No file I/O yet (planned for future)
- No syntax highlighting (could be added)
- No audio playback (see AUDIO_PLAYBACK_PLAN.md)
- No save/load functionality

## Performance

The editor is lightweight and responsive:
- Binary size: ~1.1 MB (release build)
- Memory usage: < 5 MB
- CPU usage: Minimal (event-driven)

## Next Steps

See [AUDIO_PLAYBACK_PLAN.md](AUDIO_PLAYBACK_PLAN.md) for the roadmap to add MML playback functionality.
