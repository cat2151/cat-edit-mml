use anyhow::Result;
use crossterm::{
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
use tui_textarea::TextArea;

/// ターミナルUIの管理を担当する構造体
pub struct TerminalUi {
    terminal: Terminal<CrosstermBackend<io::Stdout>>,
    textarea: TextArea<'static>,
}

impl TerminalUi {
    /// 新しいTerminalUiインスタンスを作成し、ターミナルを初期化する
    pub fn new() -> Result<Self> {
        // ターミナルの設定
        enable_raw_mode()?;
        let mut stdout = io::stdout();
        execute!(stdout, EnterAlternateScreen)?;
        let backend = CrosstermBackend::new(stdout);
        let terminal = Terminal::new(backend)?;

        // テキストエリアの作成
        let mut textarea = TextArea::default();
        textarea.set_block(
            Block::default()
                .borders(Borders::ALL)
                .title("MML Editor - Press ESC to exit"),
        );
        textarea.set_cursor_line_style(Style::default());

        Ok(Self { terminal, textarea })
    }

    /// テキストエリアのミュータブルな参照を取得
    pub fn textarea_mut(&mut self) -> &mut TextArea<'static> {
        &mut self.textarea
    }

    /// テキストエリアの現在のコンテンツを取得
    pub fn get_content(&self) -> String {
        self.textarea.lines().join("\n")
    }

    /// テキストエリアのコンテンツを設定
    pub fn set_content(&mut self, content: &str) {
        // 全選択してカット
        self.textarea.select_all();
        self.textarea.cut();
        
        // 新しいコンテンツを挿入
        self.textarea.insert_str(content);
    }

    /// テキストエリアのタイトルを設定  
    pub fn set_title(&mut self, title: &str) {
        // タイトル付きの新しいブロックを設定
        let block = Block::default()
            .borders(Borders::ALL)
            .title(title.to_string());
        self.textarea.set_block(block);
    }

    /// UIを描画する
    pub fn draw(&mut self) -> Result<()> {
        self.terminal.draw(|f| {
            f.render_widget(&self.textarea, f.area());
        })?;
        Ok(())
    }

    /// ターミナルの設定をクリーンアップする
    pub fn cleanup(&mut self) -> Result<()> {
        disable_raw_mode()?;
        execute!(self.terminal.backend_mut(), LeaveAlternateScreen)?;
        self.terminal.show_cursor()?;
        Ok(())
    }
}

impl Drop for TerminalUi {
    /// TerminalUiがドロップされるときに自動的にクリーンアップを実行
    fn drop(&mut self) {
        let _ = self.cleanup();
    }
}