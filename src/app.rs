use anyhow::Result;
use crossterm::event::{KeyEvent, KeyCode};
use crate::ui::TerminalUi;
use crate::mml::MmlProcessor;
use crate::event::{EventHandler, EventResult};
use crate::template::MmlTemplate;

/// アプリケーションの状態を管理するメイン構造体
pub struct App {
    ui: TerminalUi,
    previous_content: String,
    template_index: usize,
}

impl App {
    /// 新しいAppインスタンスを作成する
    pub fn new() -> Result<Self> {
        let mut ui = TerminalUi::new()?;
        let previous_content = String::new();
        let template_index = 0;
        
        // 初期タイトルを設定
        let title = format!("MML Editor - {} - Press ESC to exit, F2 for next template", 
                           MmlTemplate::get_template_title(template_index));
        ui.set_title(&title);
        
        Ok(Self {
            ui,
            previous_content,
            template_index,
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

        // F2キーでテンプレート切り替え
        if key.code == KeyCode::F(2) {
            return self.apply_next_template();
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

    /// 次のテンプレートを適用する
    fn apply_next_template(&mut self) -> EventResult {
        // テンプレートインデックスを次に進める（循環）
        self.template_index = (self.template_index + 1) % MmlTemplate::template_count();
        
        // 現在のテンプレートを取得
        let template = MmlTemplate::get_template(self.template_index);
        
        // テキストエリアをクリアしてテンプレートを設定
        self.ui.set_content(template);
        
        // テンプレートのタイトルを更新（UIに表示するため）
        let title = format!("MML Editor - {} - Press ESC to exit, F2 for next template", 
                           MmlTemplate::get_template_title(self.template_index));
        self.ui.set_title(&title);
        
        EventResult::ContentChanged
    }
}