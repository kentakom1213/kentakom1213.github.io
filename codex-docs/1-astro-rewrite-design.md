# Astro.js リライト設計

## 目的
- 現在の単一ページプロフィールサイトを Astro ベースに移行する。
- 既存の `contents/*.toml` / `contents/sections/*.toml` をそのまま入力データとして利用する。
- 出力は既存と同じ静的サイト（`docs/index.html` + `docs/assets/*`）を維持する。

## 設計方針
- 1ページ構成は維持し、`src/pages/index.astro` で組み立てる。
- レイアウトとセクション描画をコンポーネント分離して拡張しやすくする。
- 既存データ構造（`profile.toml`, `section*.toml`）を TypeScript 型へ写像する。
- 既存の表示ルール（研究発表フォーマット、年月フォーマット、番号付きリスト）を互換維持する。

## 推奨ディレクトリ構成
```txt
astro-site/
  package.json
  astro.config.mjs
  tsconfig.json
  public/
    assets/
      style.css
  src/
    pages/
      index.astro
    layouts/
      BaseLayout.astro
    components/
      SiteHeader.astro
      SectionBlock.astro
      ItemList.astro
      PublicationItem.astro
      TimelineItem.astro
    lib/
      content.ts
      dates.ts
      links.ts
      types.ts
```

## データモデル対応
| 現在の TOML モデル | Astro 側型 |
|---|---|
| `config.toml` | `SiteConfig` |
| `profile.toml` | `Profile` |
| `sections/*.toml` | `Section[]` |
| `items[*]` | `Item` |

ポイント:
- `authors.length > 0` なら「研究発表」扱い。
- それ以外は「時系列項目（学歴・職歴・資格など）」として描画。
- `sort`/`rev`/`order` は読み込み時に適用して、コンポーネント側は表示責務に限定する。

## 読み込み戦略
- `src/lib/content.ts` で `contents/` 配下をロード。
- TOML パーサーを1つ採用（例: `smol-toml` or `@iarna/toml`）。
- `Section` のソート:
  - セクション: `order asc`
  - サブセクション: `order asc`
  - アイテム: `sort` と `rev` に従って並べ替え

## ページ組み立て
1. `index.astro` で `loadSiteData()` を呼び出す。
2. `BaseLayout` に title/meta/OG/twitter を渡す。
3. `SiteHeader` で名前・所属・メール・目次を描画。
4. `SectionBlock` で各セクションを描画。

## メタ情報の互換
`config.toml` をそのまま利用:
- `title`
- `language`（未指定時は `ja`）
- `google_site_verification`

OG description は既存ロジックと同じ:
- `"{profile.name.ja} ({profile.affiliation.affiliation})"`

## CSS移行
- まずは `docs/assets/style.css` を `public/assets/style.css` にコピーして同等表示を確保。
- その後必要なら `src/styles/` へ分割。

## 移行ステップ
1. Astroプロジェクト作成（`astro-site/`）
2. 型定義 (`types.ts`) と TOML ローダ (`content.ts`) 実装
3. レイアウト/コンポーネント実装
4. `index.astro` で結線
5. 既存 `docs/index.html` とレンダリング結果を目視比較
6. GitHub Pages の publish 先を Astro build 出力へ切替

## 差分確認観点
- 見出し構造（`h1/h2/h3`）が一致しているか
- `#research` などアンカーリンクが正しく遷移するか
- 研究発表の文面フォーマットが一致するか
- `detail` の Markdown リンクが安全に描画されるか
- 日付表示（`2025年11月13日`, `2024年6月`）が一致するか
