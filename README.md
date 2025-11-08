# cat-edit-mml

Rustで構築されたMusic Macro Language (MML)用のTUI（テキストユーザーインターフェース）エディタです。

## 機能

- ✅ シンタックスハイライト対応の複数行テキストエディタ
- ✅ 完全なカーソル移動（矢印キー、Home、End、Page Up/Down）
- ✅ 標準的なテキスト編集操作（挿入、削除、バックスペース）
- ✅ クロスプラットフォーム対応（Windows、Linux、macOS）
- ✅ ESCキーで終了
- ✅ MMLノート（cdefgab）入力時の自動再生（cat-play-mmlを子プロセスとして呼び出し）
- ✅ MMLテンプレート機能（F2キーで順番に切り替え）

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
- **F1**: MMLテンプレートを順番に切り替え
- **ESC**: エディタを終了

### MML再生機能

エディタは入力されたMMLコンテンツを自動的に再生します：

- MMLノート（c, d, e, f, g, a, b）を含むテキストを入力すると、即座に自動的に再生が開始されます
- 再生には[cat-play-mml](https://github.com/cat2151/cat-play-mml)が子プロセスとして使用されます
- cat-play-mmlがインストールされていない場合は、エディタは通常通り動作し、再生機能はスキップされます
- MMLコンテンツはコマンドライン引数としてcat-play-mmlに渡されます

### MMLテンプレート機能

エディタには複数のMMLテンプレートが組み込まれており、F2キーで順番に切り替えることができます：

中身は仮です。現在のMMLの仕様と噛み合っていない仮です。今後修正します

1. **基本スケール**: `cdefgab`
2. **Cメジャースケール（オクターブ付き）**: `c4d4e4f4g4a4b4>c4`
3. **基本メロディパターン**: `c4e4g4e4 c4e4g4e4 f4a4>c4<a4 g4b4>d4<b4`
4. **ドレミの歌**: `c4d4e4c4 c4d4e4c4 e4f4g2 e4f4g2`
5. **コード進行**: `ceg4 fac4 gbd4 ceg4`
6. **テンポ指定付きメロディ**: `t120 c8d8e8f8g8a8b8>c8 <b8a8g8f8e8d8c2`
7. **マルチトラック**: `A o4 cdefgab>c\nB o3 c2e2g2c2`
8. **空のテンプレート**: （空の状態）

F2キーを押すごとに次のテンプレートに切り替わり、最後のテンプレートの後は最初に戻ります。

## 関連プロジェクト

- [cat-play-mml](https://github.com/cat2151/cat-play-mml) - MMLパーサーとオーディオプレイヤー
- [mmlabc-to-smf-rust](https://github.com/cat2151/mmlabc-to-smf-rust) - MMLからMIDIへのコンバーター
- [smf-to-ym2151log-rust](https://github.com/cat2151/smf-to-ym2151log-rust) - MIDIからYM2151ログへのコンバーター
- [ym2151-log-play-server](https://github.com/cat2151/ym2151-log-play-server) - YM2151オーディオプレイヤーのサーバー

## 開発

### テストの実行

```bash
cargo test
```

## 今後の展望

- 待ち。クレートのMML機能が充実するのを待ち
  - より実際的には、MML機能の充実の優先度が高いので、
  - このeditorの機能実装の優先度は比較的低い

## このprojectが目指すもの
- シンプルでミニマムな、コマンドラインから素早くMMLを鳴らす体験を提供すること
- もし鳴らなくなったら、できるだけ優先で鳴るように行動するつもり

## スコープ外
- MMLシンタックスハイライト
- MML表示を高機能に

## ライセンス

MIT License - 詳細は[LICENSE](LICENSE)ファイルを参照してください
