use anyhow::Result;
use crossterm::event::{self, Event, KeyCode};
use std::time::Duration;

/// イベント処理の結果を表すenum
#[derive(Debug, PartialEq)]
pub enum EventResult {
    /// アプリケーションを続行
    Continue,
    /// アプリケーションを終了
    Exit,
    /// コンテンツが変更された
    ContentChanged,
}

/// イベント処理を担当する構造体
pub struct EventHandler;

impl EventHandler {
    /// イベントをポーリングして処理する
    /// タイムアウト時間内にイベントがない場合、EventResult::Continueを返す
    pub fn poll_and_handle_events<F>(mut key_handler: F) -> Result<EventResult>
    where
        F: FnMut(crossterm::event::KeyEvent) -> EventResult,
    {
        if event::poll(Duration::from_millis(100))? {
            if let Event::Key(key) = event::read()? {
                return Ok(key_handler(key));
            }
        }
        Ok(EventResult::Continue)
    }

    /// 標準的なキーイベントを処理する
    /// ESCで終了、その他のキーでコンテンツ変更を示す
    pub fn handle_default_key_event(key: crossterm::event::KeyEvent) -> EventResult {
        match key.code {
            KeyCode::Esc => EventResult::Exit,
            _ => EventResult::ContentChanged,
        }
    }
}