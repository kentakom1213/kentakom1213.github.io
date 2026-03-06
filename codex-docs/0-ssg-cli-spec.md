# SSG CLI 仕様（crates/sitegen）

## 目的

- `crates/ui` と `crates/content` を利用して静的サイトを生成する CLI を提供する。
- `build` で `docs/index.html` を生成する。
- `serve` でローカルプレビューを起動し、TOML の更新を監視して自動反映する。

## 対象バイナリ

- クレート: `crates/sitegen`
- バイナリ名: `sitegen`（Cargo のデフォルト）

## CLI 全体像（clap）

- ルートコマンド: `sitegen`
- サブコマンド:
  - `build`
  - `serve`

### 共通オプション（将来拡張も含む）

- `--content-dir <PATH>`
  - デフォルト: `content/`
  - `crates/content::LoadOptions` の `content_dir` に渡す。

## build サブコマンド

### 目的

- `content` から読み込んだデータを `ui` で描画して `index.html` を出力する。

### 入力

- `config.toml`, `profile.toml`, `sections/*.toml`
- 入力ディレクトリは `--content-dir` で指定可能。

### 出力

- デフォルト: `docs/index.html`
- 出力先は `config.toml` の `[build]` を参照して上書き可能。
  - `build.output_dir` があればそれを採用（例: `docs`）
  - `build.output_file` があればそれを採用（例: `index.html`）
- `build` セクションが無い場合は `docs/index.html` にフォールバック。

### 処理フロー

1. `content::load_all(LoadOptions)` で `IndexData` を取得。
2. `ui::render_index_string(&IndexData)` で HTML を生成。
3. 出力先ディレクトリを作成（存在しなければ）。
4. `index.html` を UTF-8 で書き出す。

### 例

```
sitegen build
sitegen build --content-dir contents
```

## serve サブコマンド

### 目的

- `build` を実行した後、ローカルサーバーでプレビューを提供する。
- TOML の更新を検知したら自動で再生成し、プレビューに反映する。

### 仕様

- サーバー: `miniserve` を使用。
- 監視対象: `content_dir` 配下の `*.toml`（`config.toml`, `profile.toml`, `sections/*.toml`）
- 監視方法: ファイル監視ライブラリ（例: `notify`）を利用。
- 更新検知時の動作:
  1. `build` 相当の再生成を実行。
  2. 生成後、miniserve の配信ディレクトリに反映される。

### オプション

- `--port <PORT>`
  - デフォルト: `8080`
- `--open`
  - デフォルト: `false`
  - `true` の場合、ブラウザを自動で開く（miniserve の `--open` を利用）。

### 例

```
sitegen serve
sitegen serve --content-dir contents --port 4000 --open
```

## 設定ファイル（config.toml）

`crates/content::ConfigToml` を前提とする。

```
# content/config.toml

title = "My Site"
language = "ja"

[build]
output_dir = "docs"
output_file = "index.html"

[assets]
dir = "assets"
mount_path = "/assets"
```

### 期待するディレクトリ構成

```
content/
  config.toml
  profile.toml
  sections/
    research.toml
    education.toml
```

## エラーハンドリング

- TOML パースエラー、ファイル未発見などは `anyhow` で文脈付きエラーを返す。
- `serve` では再生成失敗時にエラーを標準エラーに表示し、サーバー自体は継続する。

## 非目標

- 複数ページ生成、ルーティング、Markdown 対応などの拡張は対象外。
- `miniserve` 以外のサーバーへの切り替えは対象外。
