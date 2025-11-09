use std::process::{Command, Stdio};
use std::sync::atomic::{AtomicBool, Ordering};

/// cat-play-mmlのインストール試行済みフラグ
static INSTALL_ATTEMPTED: AtomicBool = AtomicBool::new(false);

/// MML関連の処理を担当するモジュール
pub struct MmlProcessor;

impl MmlProcessor {
    /// cat-play-mmlがインストールされているかチェックする
    fn is_cat_play_mml_installed() -> bool {
        Command::new("cat-play-mml")
            .arg("--version")
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .status()
            .is_ok()
    }

    /// cat-play-mmlを自動的にインストールする
    fn install_cat_play_mml() {
        // 既にインストール試行済みの場合は何もしない
        if INSTALL_ATTEMPTED.swap(true, Ordering::SeqCst) {
            return;
        }

        // cargo install --git でインストールを試みる
        let _ = Command::new("cargo")
            .args(&[
                "install",
                "--git",
                "https://github.com/cat2151/cat-play-mml",
            ])
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .spawn();
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

    /// cat-play-mmlサブプロセスを使ってMMLコンテンツを再生する
    pub fn play_mml(content: &str) {
        // cat-play-mmlをサブプロセスとして起動し、コンテンツを引数として渡す
        match Command::new("cat-play-mml")
            .arg(content)
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .spawn()
        {
            Ok(_child) => {
                // 注意: 子プロセスの完了を待たない
                // バックグラウンドで実行される
            }
            Err(_) => {
                // cat-play-mmlが見つからない場合、インストールされているかチェック
                if !Self::is_cat_play_mml_installed() {
                    // インストールされていない場合、自動インストールを試みる
                    Self::install_cat_play_mml();
                }
                // エディタは動作し続ける
            }
        }
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