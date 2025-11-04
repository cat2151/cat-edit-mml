# Audio Playback Implementation Plan

## Overview
This document outlines the plan to integrate real-time MML (Music Macro Language) playback into the cat-edit-mml TUI editor, similar to the functionality in cat-play-mml.

## Current Status
- ✅ Basic TUI editor implemented with ratatui, tui-textarea, and crossterm
- ✅ Multi-line text input and cursor movement working
- ✅ ESC key exits the application
- ⏳ Audio playback feature pending

## Dependencies Required
The following crates from cat-play-mml need to be integrated:

```toml
mmlabc-to-smf = { git = "https://github.com/cat2151/mmlabc-to-smf-rust" }
smf-to-ym2151log-rust = { git = "https://github.com/cat2151/smf-to-ym2151log-rust" }
ym2151-log-player-rust = { git = "https://github.com/cat2151/ym2151-log-player-rust", features = ["realtime-audio"] }
serde_json = "1.0"
```

## Implementation Strategy

### Phase 1: Audio System Integration
1. **Add audio dependencies** to Cargo.toml (commented out for now due to platform-specific requirements)
2. **Create audio module** (`src/audio.rs`) with:
   - MML to audio conversion pipeline
   - Background audio playback thread management
   - Graceful error handling for audio initialization failures

### Phase 2: Editor Integration
1. **Debounced Playback Trigger**
   - Add timer to prevent playback on every keystroke
   - Trigger playback after 500-1000ms of idle time after editing
   - Cancel ongoing playback when new edits occur

2. **Background Audio Thread**
   - Spawn separate thread for audio processing
   - Use channels (mpsc) to communicate between editor and audio thread
   - Send edited MML content to audio thread for processing

3. **UI Status Indicator**
   - Show playback status in the editor border title
   - Display errors if MML parsing or playback fails
   - Show current playback state (idle, playing, error)

### Phase 3: Error Handling
1. **Graceful Degradation**
   - Continue operating as text editor if audio initialization fails
   - Show warning message about audio unavailability
   - Log audio errors to stderr without crashing

2. **MML Validation**
   - Catch parsing errors from mmlabc-to-smf
   - Display error messages in the UI
   - Continue allowing editing even with invalid MML

## Code Structure

### Proposed File Structure
```
src/
├── main.rs          # Main entry point and event loop
├── audio.rs         # Audio playback module
├── editor.rs        # Editor state and UI (optional refactor)
└── config.rs        # Configuration (debounce time, etc.)
```

### Key Components

#### Audio Module (`src/audio.rs`)
```rust
pub struct AudioSystem {
    sender: mpsc::Sender<AudioCommand>,
    thread_handle: JoinHandle<()>,
}

pub enum AudioCommand {
    Play(String),  // MML content to play
    Stop,
    Shutdown,
}

impl AudioSystem {
    pub fn new() -> Result<Self>;
    pub fn play_mml(&self, mml: &str);
    pub fn stop(&self);
}
```

#### Main Event Loop Integration
```rust
// Pseudo-code for main.rs
let mut audio_system = AudioSystem::new().ok(); // Optional audio
let mut last_edit_time = Instant::now();
let mut last_content = String::new();

loop {
    terminal.draw(...)?;
    
    if event::poll(Duration::from_millis(100))? {
        match event::read()? {
            Event::Key(key) => {
                if key.code == KeyCode::Esc { break; }
                textarea.input(key);
                last_edit_time = Instant::now();
            }
        }
    }
    
    // Debounced playback trigger
    if last_edit_time.elapsed() > Duration::from_millis(500) {
        let content = textarea.lines().join("\n");
        if content != last_content {
            if let Some(audio) = &audio_system {
                audio.play_mml(&content);
            }
            last_content = content;
        }
    }
}
```

## Platform Considerations

### Windows (Target Platform)
- Uses WASAPI backend via cpal
- Should work without additional system dependencies
- Test on Windows before final deployment

### Linux (Development)
- Requires ALSA development libraries (`libalsa-dev` package)
- May need additional audio system dependencies
- Use feature flags to make audio optional during development

### macOS
- Uses CoreAudio backend via cpal
- Should work without additional dependencies

## Testing Plan

1. **Unit Tests**
   - Test MML parsing with valid/invalid input
   - Test audio command channel communication
   - Test debounce logic

2. **Integration Tests**
   - Test full MML → audio pipeline
   - Test graceful degradation without audio
   - Test editor continues working if audio fails

3. **Manual Testing**
   - Test on Windows target platform
   - Test with various MML snippets
   - Test performance with large MML files
   - Test rapid editing behavior

## Performance Considerations

1. **Memory Usage**
   - Limit audio buffer size
   - Clean up previous playback before starting new one
   - Consider streaming for large MML files

2. **CPU Usage**
   - Run audio processing in separate thread
   - Use debouncing to avoid excessive processing
   - Consider limiting playback duration for testing

3. **Responsiveness**
   - Ensure UI remains responsive during playback
   - Allow playback cancellation
   - Don't block on audio operations

## Alternative Approaches

If real-time playback proves too complex:

1. **Save and Play Mode**
   - Add command to save and play current content
   - Use external player (like cat-play-mml binary)
   - Simpler integration, less real-time

2. **Preview Mode**
   - Add mode toggle (edit/preview)
   - Only play in preview mode
   - Clearer separation of concerns

3. **Syntax Validation Only**
   - Just validate MML syntax
   - Show errors but don't play
   - Focus on editor quality

## Timeline Estimate

- Phase 1 (Audio Integration): 4-8 hours
- Phase 2 (Editor Integration): 4-6 hours
- Phase 3 (Error Handling): 2-4 hours
- Testing and Refinement: 4-6 hours

**Total**: 14-24 hours of development time

## Dependencies and Blockers

- **Windows testing environment**: Need access to Windows for final testing
- **Audio hardware**: Requires working audio output on target system
- **Platform libraries**: ALSA on Linux, WASAPI on Windows
- **Git dependencies**: Network access to download cat-play-mml crates

## Success Criteria

✅ Editor starts successfully with or without audio support
✅ Multi-line editing works smoothly
✅ Audio plays automatically after editing (with debounce)
✅ Errors are handled gracefully
✅ Performance is acceptable (no lag during typing)
✅ Works on Windows target platform

## Next Steps

1. Enable audio feature flag in Cargo.toml
2. Implement audio module with basic playback
3. Add debouncing logic to main event loop
4. Test on Linux development environment
5. Test on Windows target environment
6. Refine based on performance and usability feedback
