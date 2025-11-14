use anyhow::Result;
use crossterm::event::{KeyEvent, KeyCode, KeyModifiers};
use crate::ui::TerminalUi;
use crate::mml::MmlProcessor;
use crate::event::{EventHandler, EventResult};
use crate::template::MmlTemplate;

/// MML再生モード
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum PlaybackMode {
    /// 現在入力した音のみを鳴らす（デフォルト）
    CurrentNote,
    /// MML全体を鳴らす
    FullMml,
}

/// アプリケーションの状態を管理するメイン構造体
pub struct App {
    ui: TerminalUi,
    previous_content: String,
    template_index: usize,
    playback_mode: PlaybackMode,
}

impl App {
    /// 新しいAppインスタンスを作成する
    pub fn new() -> Result<Self> {
        let mut ui = TerminalUi::new()?;
        let previous_content = String::new();
        let template_index = 0;
        let playback_mode = PlaybackMode::CurrentNote;
        
        // 初期タイトルを設定
        let title = Self::format_title(template_index, playback_mode);
        ui.set_title(&title);
        
        Ok(Self {
            ui,
            previous_content,
            template_index,
            playback_mode,
        })
    }

    /// タイトル文字列をフォーマットする
    fn format_title(template_index: usize, playback_mode: PlaybackMode) -> String {
        let mode_str = match playback_mode {
            PlaybackMode::CurrentNote => "現在の音のみ",
            PlaybackMode::FullMml => "MML全体",
        };
        format!(
            "MML Editor - {} - 再生モード: {} - ESC:終了 F2:テンプレート切替 Ctrl+P:再生モード切替", 
            MmlTemplate::get_template_title(template_index),
            mode_str
        )
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

        // F2キーでテンプレート切り替え
        if key.code == KeyCode::F(2) {
            return self.apply_next_template();
        }

        // Ctrl+Pで再生モード切り替え
        if key.code == KeyCode::Char('p') && key.modifiers.contains(KeyModifiers::CONTROL) {
            return self.toggle_playback_mode();
        }

        // キーをテキストエリアに渡す
        self.ui.textarea_mut().input(key);
        EventResult::ContentChanged
    }

    /// コンテンツ変更を処理する
    fn handle_content_change(&mut self) {
        let current_content = self.ui.get_content();
        
        if current_content != self.previous_content {
            let content_to_play = match self.playback_mode {
                PlaybackMode::CurrentNote => {
                    // 差分のみを再生
                    MmlProcessor::calculate_diff(&self.previous_content, &current_content)
                }
                PlaybackMode::FullMml => {
                    // MML全体を再生
                    current_content.clone()
                }
            };
            
            if !content_to_play.trim().is_empty() && MmlProcessor::contains_mml_notes(&content_to_play) {
                MmlProcessor::play_mml(&content_to_play);
            }
            
            self.previous_content = current_content;
        }
    }

    /// 次のテンプレートを適用する
    fn apply_next_template(&mut self) -> EventResult {
        // テンプレートインデックスを次に進める（循環）
        self.template_index = (self.template_index + 1) % MmlTemplate::template_count();
        
        // 現在のテンプレートを取得
        let template = MmlTemplate::get_template(self.template_index);
        
        // テキストエリアをクリアしてテンプレートを設定
        self.ui.set_content(template);
        
        // タイトルを更新
        let title = Self::format_title(self.template_index, self.playback_mode);
        self.ui.set_title(&title);
        
        EventResult::ContentChanged
    }

    /// 再生モードを切り替える
    fn toggle_playback_mode(&mut self) -> EventResult {
        self.playback_mode = match self.playback_mode {
            PlaybackMode::CurrentNote => PlaybackMode::FullMml,
            PlaybackMode::FullMml => PlaybackMode::CurrentNote,
        };
        
        // タイトルを更新して現在のモードを表示
        let title = Self::format_title(self.template_index, self.playback_mode);
        self.ui.set_title(&title);
        
        EventResult::Continue
    }
}