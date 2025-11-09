Last updated: 2025-11-10

# Development Status

## 現在のIssues
- [Issue #5](../issue-notes/5.md): `cat-play-mml` がシステムに見つからない場合、自動でGitHubからインストールする機能の実装が進行中。
- `src/mml.rs` にはツールの有無をチェックする `is_cat_play_mml_installed()` とインストールを行う `install_cat_play_mml()` の記述があるが、現在のコードにはまだ反映されていないため実装が必要。
- この機能により、MML再生機能の初回利用時のセットアップが自動化され、ユーザー体験が向上する予定。

## 次の一手候補
1. [Issue #5](../issue-notes/5.md): `cat-play-mml` 自動インストール機能の実装
   - 最初の小さな一歩: `src/mml.rs` に `is_cat_play_mml_installed` 関数を実装し、`cat-play-mml --version` コマンドの実行結果を基にツールの存在を確認するロジックを追加する。
   - Agent実行プロンプ:
     ```
     対象ファイル: `src/mml.rs`

     実行内容: `src/mml.rs` 内に、`cat-play-mml` コマンドがシステムにインストールされているかを確認する `is_cat_play_mml_installed` 関数を実装してください。この関数は、`Command::new("cat-play-mml").arg("--version").output()` を実行し、コマンドが成功し、かつ標準出力に何らかのバージョン情報が含まれることをもってインストール済みと判断するロジックを実装してください。

     確認事項: `cat-play-mml` が存在しない環境、存在する環境の両方でこの関数が期待通りに動作するかを考慮してください。また、将来的に `install_cat_play_mml` 関数と連携する際のインターフェースも念頭に置いてください。

     期待する出力: `src/mml.rs` が更新され、`is_cat_play_mml_installed` 関数が適切に実装されていること。
     ```

2. [Issue #31](../issue-notes/31.md) (新規): `call-daily-project-summary.yml` の呼び出し元パスを最適化
   - 最初の小さな一歩: `.github/workflows/call-daily-project-summary.yml` を開き、`uses` のパスが外部リポジトリ (`cat2151/github-actions/...`) を参照しているか、それとも現在のリポジトリ内 (`./.github/workflows/...`) を参照しているかを確認する。
   - Agent実行プロンプト:
     ```
     対象ファイル: `.github/workflows/call-daily-project-summary.yml`

     実行内容: 対象ファイル内の `uses` キーの値を分析し、現在のリポジトリで定義されている共通ワークフロー `.github/workflows/daily-project-summary.yml` を参照するようにパスを修正してください。例えば、`uses: cat2151/github-actions/.github/workflows/daily-project-summary.yml` のような外部参照を、適切な内部参照 (例: `uses: ./.github/workflows/daily-project-summary.yml@main` または `{現在のリポジトリ名}/.github/workflows/daily-project-summary.yml@main`) に変更してください。

     確認事項: 変更前に、`issue-notes/4.md` で言及されているローカルテスト環境（`act` など）で変更が正しく機能するか確認できることを考慮し、変更後のワークフローが中断なく実行できることを確認してください。また、`daily-project-summary.yml` が確実にルートリポジトリに存在し、アクセス可能であることを確認してください。

     期待する出力: `.github/workflows/call-daily-project-summary.yml` の `uses` パスが、現在のリポジトリ内の共通ワークフローを正しく参照するように修正されていること。
     ```

3. [Issue #32](../issue-notes/32.md) (新規): `README.md` に `cat-play-mml` の自動インストールと初回ビルドに関する情報を追記
   - 最初の小さな一歩: `README.md` を開き、MML再生機能に関する既存の説明セクションを特定し、`cat-play-mml` のインストール方法や前提条件について触れている箇所があるかを確認する。
   - Agent実行プロンプト:
     ```
     対象ファイル: `README.md`

     実行内容: `README.md` 内のMML再生機能に関する説明セクション（例: "MML Playback Feature" や関連する項目）を特定し、以下の情報を追記または更新してください。
     - `cat-play-mml` がシステムに見つからない場合、初回利用時に自動的に `cargo install --git https://github.com/cat2151/cat-play-mml.git` コマンドでインストールされること。
     - 初回インストール時には、ビルドに1分程度の時間がかかる場合があること。

     確認事項: 追記する情報が既存のドキュメントと矛盾しないか、またユーザーがインストールプロセスと初回起動時の挙動を明確に理解できるかを確認してください。既存のMML関連の説明に自然に統合されるように調整してください。

     期待する出力: `README.md` が更新され、`cat-play-mml` の自動インストールに関する情報と初回ビルドの注意点が追記されていること。
     ```

---
Generated at: 2025-11-10 08:19:50 JST
