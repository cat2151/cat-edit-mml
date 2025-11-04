# cat-edit-mml

A TUI (Text User Interface) editor for Music Macro Language (MML), built with Rust.

## Features

- ✅ Multi-line text editor with syntax highlighting support
- ✅ Full cursor movement (arrow keys, Home, End, Page Up/Down)
- ✅ Standard text editing operations (insert, delete, backspace)
- ✅ Cross-platform support (Windows, Linux, macOS)
- ✅ ESC key to exit
- ⏳ Real-time MML playback (planned - see [AUDIO_PLAYBACK_PLAN.md](AUDIO_PLAYBACK_PLAN.md))

## Built With

- [ratatui](https://github.com/ratatui-org/ratatui) - TUI framework
- [tui-textarea](https://github.com/rhysd/tui-textarea) - Multi-line text input widget
- [crossterm](https://github.com/crossterm-rs/crossterm) - Cross-platform terminal manipulation

## Installation

### Prerequisites

- Rust 1.70 or later
- Cargo (comes with Rust)

### Building from Source

```bash
git clone https://github.com/cat2151/cat-edit-mml.git
cd cat-edit-mml
cargo build --release
```

The binary will be available at `target/release/cat-edit-mml`

## Usage

Run the editor:

```bash
cargo run
```

Or run the compiled binary:

```bash
./target/release/cat-edit-mml
```

### Keyboard Controls

- **Arrow Keys**: Move cursor
- **Home/End**: Jump to start/end of line
- **Page Up/Down**: Scroll through document
- **Enter**: New line
- **Backspace**: Delete character before cursor
- **Delete**: Delete character at cursor
- **ESC**: Exit editor

## Related Projects

- [cat-play-mml](https://github.com/cat2151/cat-play-mml) - MML parser and audio player
- [mmlabc-to-smf-rust](https://github.com/cat2151/mmlabc-to-smf-rust) - MML to MIDI converter
- [smf-to-ym2151log-rust](https://github.com/cat2151/smf-to-ym2151log-rust) - MIDI to YM2151 log converter
- [ym2151-log-player-rust](https://github.com/cat2151/ym2151-log-player-rust) - YM2151 audio player

## Development

### Running Tests

```bash
cargo test
```

### Building for Different Platforms

**Windows:**
```bash
cargo build --release --target x86_64-pc-windows-gnu
```

**Linux:**
```bash
cargo build --release --target x86_64-unknown-linux-gnu
```

**macOS:**
```bash
cargo build --release --target x86_64-apple-darwin
```

## Future Development

See [AUDIO_PLAYBACK_PLAN.md](AUDIO_PLAYBACK_PLAN.md) for planned audio playback features.

## License

MIT License - see [LICENSE](LICENSE) file for details

## Author

cat2151