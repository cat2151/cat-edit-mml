# cat-edit-mml

Rustで構築されたMusic Macro Language (MML)用のTUI（テキストユーザーインターフェース）エディタです。

## 機能

- ✅ シンタックスハイライト対応の複数行テキストエディタ
- ✅ 完全なカーソル移動（矢印キー、Home、End、Page Up/Down）
- ✅ 標準的なテキスト編集操作（挿入、削除、バックスペース）
- ✅ クロスプラットフォーム対応（Windows、Linux、macOS）
- ✅ ESCキーで終了
- ⏳ リアルタイムMML再生（計画中 - [AUDIO_PLAYBACK_PLAN.md](AUDIO_PLAYBACK_PLAN.md)を参照）

## 使用技術

- [ratatui](https://github.com/ratatui-org/ratatui) - TUIフレームワーク
- [tui-textarea](https://github.com/rhysd/tui-textarea) - 複数行テキスト入力ウィジェット
- [crossterm](https://github.com/crossterm-rs/crossterm) - クロスプラットフォーム端末操作

## インストール

### 前提条件

- Rust 1.70以降
- Cargo（Rustに付属）

### ソースからのビルド

```bash
git clone https://github.com/cat2151/cat-edit-mml.git
cd cat-edit-mml
cargo build --release
```

バイナリは`target/release/cat-edit-mml`に生成されます

## 使用方法

エディタを実行：

```bash
cargo run
```

またはコンパイル済みバイナリを実行：

```bash
./target/release/cat-edit-mml
```

### キーボード操作

- **矢印キー**: カーソル移動
- **Home/End**: 行の先頭/末尾へジャンプ
- **Page Up/Down**: ドキュメントをスクロール
- **Enter**: 改行
- **Backspace**: カーソル前の文字を削除
- **Delete**: カーソル位置の文字を削除
- **ESC**: エディタを終了

## 関連プロジェクト

- [cat-play-mml](https://github.com/cat2151/cat-play-mml) - MMLパーサーとオーディオプレイヤー
- [mmlabc-to-smf-rust](https://github.com/cat2151/mmlabc-to-smf-rust) - MMLからMIDIへのコンバーター
- [smf-to-ym2151log-rust](https://github.com/cat2151/smf-to-ym2151log-rust) - MIDIからYM2151ログへのコンバーター
- [ym2151-log-player-rust](https://github.com/cat2151/ym2151-log-player-rust) - YM2151オーディオプレイヤー

## 開発

### テストの実行

```bash
cargo test
```

### 各プラットフォーム向けのビルド

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

## 今後の開発

計画中のオーディオ再生機能については[AUDIO_PLAYBACK_PLAN.md](AUDIO_PLAYBACK_PLAN.md)を参照してください。

## ライセンス

MIT License - 詳細は[LICENSE](LICENSE)ファイルを参照してください

## 作者

cat2151