use anyhow::Result;
use crossterm::event::KeyEvent;
use crate::ui::TerminalUi;
use crate::mml::MmlProcessor;
use crate::event::{EventHandler, EventResult};

/// アプリケーションの状態を管理するメイン構造体
pub struct App {
    ui: TerminalUi,
    previous_content: String,
}

impl App {
    /// 新しいAppインスタンスを作成する
    pub fn new() -> Result<Self> {
        let ui = TerminalUi::new()?;
        let previous_content = String::new();
        
        Ok(Self {
            ui,
            previous_content,
        })
    }

    /// アプリケーションのメインループを実行する
    pub fn run(&mut self) -> Result<()> {
        loop {
            // UIを描画
            self.ui.draw()?;

            // イベントを処理
            let result = EventHandler::poll_and_handle_events(|key| {
                self.handle_key_event(key)
            })?;

            match result {
                EventResult::Exit => break,
                EventResult::ContentChanged => {
                    self.handle_content_change();
                }
                EventResult::Continue => {
                    // 何もしない、ループを続ける
                }
            }
        }

        // ターミナルのクリーンアップ
        self.ui.cleanup()?;
        Ok(())
    }

    /// キーイベントを処理する
    fn handle_key_event(&mut self, key: KeyEvent) -> EventResult {
        // ESCキーで終了
        if key.code == crossterm::event::KeyCode::Esc {
            return EventResult::Exit;
        }

        // キーをテキストエリアに渡す
        self.ui.textarea_mut().input(key);
        EventResult::ContentChanged
    }

    /// コンテンツ変更を処理する
    fn handle_content_change(&mut self) {
        let current_content = self.ui.get_content();
        
        if current_content != self.previous_content {
            let diff_content = MmlProcessor::calculate_diff(&self.previous_content, &current_content);
            
            if !diff_content.trim().is_empty() && MmlProcessor::contains_mml_notes(&diff_content) {
                MmlProcessor::play_mml(&diff_content);
            }
            
            self.previous_content = current_content;
        }
    }
}