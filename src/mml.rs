use anyhow::Result;

// Windows専用のモジュール
#[cfg(windows)]
use mmlabc_to_smf::{pass1_parser, pass2_ast, pass3_events, pass4_midi};
#[cfg(windows)]
use smf_to_ym2151log::convert_smf_to_ym2151_log;
#[cfg(windows)]
use ym2151_log_play_server::client;

/// MML関連の処理を担当するモジュール
pub struct MmlProcessor;

impl MmlProcessor {
    /// サーバーを初期化する（起動時に一度だけ呼ぶ）
    #[cfg(windows)]
    pub fn ensure_server_running() -> Result<()> {
        // ライブラリの関数1つでサーバー存在を保証
        client::ensure_server_ready("cat-play-mml")
    }

    /// サーバーを初期化する（非Windows環境用スタブ）
    #[cfg(not(windows))]
    pub fn ensure_server_running() -> Result<()> {
        Ok(())
    }

    /// MML音符（c, d, e, f, g, a, b）が含まれているかチェックする
    pub fn contains_mml_notes(content: &str) -> bool {
        let lowercase = content.to_lowercase();
        lowercase
            .chars()
            .any(|c| matches!(c, 'c' | 'd' | 'e' | 'f' | 'g' | 'a' | 'b'))
    }

    /// 前回のコンテンツと現在のコンテンツの差分を計算する
    /// 新しく追加された文字（差分コンテンツ）を返す
    pub fn calculate_diff(previous: &str, current: &str) -> String {
        // シンプルなアプローチ: 現在の方が長く、前回がプレフィックスの場合
        // 追加された文字を返す
        if current.len() > previous.len() && current.starts_with(previous) {
            current[previous.len()..].to_string()
        } else if current.len() < previous.len() && previous.starts_with(current) {
            // コンテンツが削除された - 再生する差分はない
            String::new()
        } else {
            // 複雑な変更が行われた - 現在の状態をユーザーが聞けるよう
            // 現在のコンテンツ全体を再生する
            current.to_string()
        }
    }

    /// MML文字列をYM2151 JSON形式に変換する（Windows専用）
    #[cfg(windows)]
    fn convert_mml_to_json(mml: &str) -> Result<String> {
        // MML → SMF (4パスの統合)
        let tokens = pass1_parser::parse_mml(mml);
        let ast = pass2_ast::tokens_to_ast(&tokens);
        let events = pass3_events::ast_to_events(&ast);
        let smf_data = pass4_midi::events_to_midi(&events)?;

        // SMF → YM2151ログ
        let json = convert_smf_to_ym2151_log(&smf_data)?;
        Ok(json)
    }

    /// MMLコンテンツを再生する（Windows専用）
    #[cfg(windows)]
    pub fn play_mml(content: &str) {
        // MMLをJSON形式に変換
        let json = match Self::convert_mml_to_json(content) {
            Ok(j) => j,
            Err(e) => {
                eprintln!("⚠️  MML変換エラー: {}", e);
                return;
            }
        };

        // send_json()を使用（サイズに応じて自動的に最適な送信方法を選択）
        if let Err(e) = client::send_json(&json) {
            eprintln!("⚠️  演奏エラー: {}", e);
            eprintln!("   サーバーが起動していない可能性があります");
        }
    }

    /// MMLコンテンツを再生する（非Windows環境用スタブ）
    #[cfg(not(windows))]
    pub fn play_mml(_content: &str) {
        // 非Windows環境では再生機能は利用できない
        eprintln!("⚠️  音声再生はWindows専用機能です");
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_contains_mml_notes_with_valid_notes() {
        assert!(MmlProcessor::contains_mml_notes("cdefgab"));
        assert!(MmlProcessor::contains_mml_notes("CDEFGAB"));
        assert!(MmlProcessor::contains_mml_notes("c"));
        assert!(MmlProcessor::contains_mml_notes("MML: c d e f g a b"));
    }

    #[test]
    fn test_contains_mml_notes_without_notes() {
        assert!(!MmlProcessor::contains_mml_notes("xyz"));
        assert!(!MmlProcessor::contains_mml_notes("123"));
        assert!(!MmlProcessor::contains_mml_notes(""));
        assert!(!MmlProcessor::contains_mml_notes("hij klmn"));
    }

    #[test]
    fn test_contains_mml_notes_mixed_content() {
        // MML音符文字が含まれている
        assert!(MmlProcessor::contains_mml_notes("The note C is important"));
        assert!(MmlProcessor::contains_mml_notes("tempo 120\nc d e"));
        assert!(MmlProcessor::contains_mml_notes("hello world")); // 'd' と 'e' が含まれている
    }

    #[test]
    fn test_calculate_diff_append() {
        // 文字を追加
        assert_eq!(MmlProcessor::calculate_diff("cde", "cdefg"), "fg");
        assert_eq!(MmlProcessor::calculate_diff("", "abc"), "abc");
        assert_eq!(MmlProcessor::calculate_diff("hello", "hello world"), " world");
    }

    #[test]
    fn test_calculate_diff_removal() {
        // 文字を削除 - 空を返すべき
        assert_eq!(MmlProcessor::calculate_diff("cdefg", "cde"), "");
        assert_eq!(MmlProcessor::calculate_diff("hello world", "hello"), "");
    }

    #[test]
    fn test_calculate_diff_complex_change() {
        // 複雑な変更 - 現在のコンテンツ全体を返すべき
        assert_eq!(MmlProcessor::calculate_diff("abc", "def"), "def");
        assert_eq!(MmlProcessor::calculate_diff("cde", "cgf"), "cgf");
    }
}