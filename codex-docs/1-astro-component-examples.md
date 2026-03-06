# Astro コンポーネント実装例

## 1. 型定義 (`src/lib/types.ts`)
```ts
export type SortKey = "date" | "title";

export type Item = {
  date?: string;
  start_date?: string;
  end_date?: string;
  authors?: string[];
  venue?: string;
  location?: string;
  title: string;
  detail?: string;
};

export type Subsection = {
  name: string;
  order?: number;
  sort?: SortKey;
  rev?: boolean;
  numbering?: boolean;
  items: Item[];
};

export type Section = {
  name: string;
  key: string;
  order?: number;
  sort?: SortKey;
  rev?: boolean;
  numbering?: boolean;
  items: Item[];
  subsections: Subsection[];
};

export type SiteData = {
  config: {
    title: string;
    language?: string;
    google_site_verification?: string;
  };
  profile: {
    lead?: string;
    name: { ja: string; en: string };
    affiliation: { affiliation: string };
    contact: { email?: string };
  };
  sections: Section[];
};
```

## 2. Base Layout (`src/layouts/BaseLayout.astro`)
```astro
---
export interface Props {
  title: string;
  lang?: string;
  ogDescription: string;
  googleSiteVerification?: string;
}

const { title, lang = "ja", ogDescription, googleSiteVerification } = Astro.props;
---
<!doctype html>
<html lang={lang}>
  <head>
    <meta charset="utf-8" />
    <meta name="viewport" content="width=device-width, initial-scale=1" />
    <title>{title}</title>
    <meta property="og:title" content={title} />
    <meta property="og:type" content="website" />
    <meta property="og:description" content={ogDescription} />
    <meta name="twitter:card" content="summary" />
    <meta name="twitter:title" content={title} />
    <meta name="twitter:description" content={ogDescription} />
    {googleSiteVerification && (
      <meta name="google-site-verification" content={googleSiteVerification} />
    )}
    <link rel="stylesheet" href="/assets/style.css" />
  </head>
  <body>
    <div class="container">
      <slot />
    </div>
  </body>
</html>
```

## 3. ヘッダー (`src/components/SiteHeader.astro`)
```astro
---
import type { SiteData } from "../lib/types";

export interface Props {
  profile: SiteData["profile"];
  sections: SiteData["sections"];
}

const { profile, sections } = Astro.props;
---
<header>
  <h1>{profile.name.ja}</h1>
  <p class="en-name">{profile.name.en}</p>

  <p>
    <span>所属: </span><wbr /><span class="label">{profile.affiliation.affiliation}</span>
  </p>

  {profile.contact.email && (
    <p>
      <span>Email: </span><wbr /><span class="label">{profile.contact.email}</span>
    </p>
  )}

  {profile.lead && <p>{profile.lead}</p>}

  <nav>
    <ul>
      {sections.map((sec) => (
        <li><a href={`#${sec.key}`}>{sec.name}</a></li>
      ))}
    </ul>
  </nav>
</header>
```

## 4. セクション描画 (`src/components/SectionBlock.astro`)
```astro
---
import type { Section } from "../lib/types";
import ItemList from "./ItemList.astro";

export interface Props {
  section: Section;
}

const { section } = Astro.props;
---
<section>
  <h2 id={section.key}>{section.name}</h2>

  {section.items.length > 0 && (
    <ItemList items={section.items} numbering={section.numbering ?? false} />
  )}

  {section.subsections.map((sub) => (
    <Fragment>
      <h3>{sub.name}</h3>
      {sub.items.length > 0 && (
        <ItemList items={sub.items} numbering={sub.numbering ?? false} />
      )}
    </Fragment>
  ))}
</section>
```

## 5. 項目切り替え (`src/components/ItemList.astro`)
```astro
---
import type { Item } from "../lib/types";
import PublicationItem from "./PublicationItem.astro";
import TimelineItem from "./TimelineItem.astro";

export interface Props {
  items: Item[];
  numbering: boolean;
}

const { items, numbering } = Astro.props;
const hasAuthors = (item: Item) => (item.authors?.length ?? 0) > 0;
---
{numbering ? (
  <ol>
    {items.map((item) => (
      <li>{hasAuthors(item) ? <PublicationItem item={item} /> : <TimelineItem item={item} />}</li>
    ))}
  </ol>
) : (
  <ul>
    {items.map((item) => (
      <li>{hasAuthors(item) ? <PublicationItem item={item} /> : <TimelineItem item={item} />}</li>
    ))}
  </ul>
)}
```

## 6. 研究発表項目 (`src/components/PublicationItem.astro`)
```astro
---
import type { Item } from "../lib/types";
import { formatJaDate } from "../lib/dates";

export interface Props {
  item: Item;
}

const { item } = Astro.props;
const authors = item.authors ?? [];
---
{authors.join(", ")}, "{item.title}"
{item.venue ? `, ${item.venue}` : ""}
{item.location ? `, ${item.location}` : ""}
{item.date ? `, ${formatJaDate(item.date)}` : ""}
{item.detail ? ` - ${item.detail}` : ""}
```

## 7. 時系列項目 (`src/components/TimelineItem.astro`)
```astro
---
import type { Item } from "../lib/types";
import { formatTimeline } from "../lib/dates";

export interface Props {
  item: Item;
}

const { item } = Astro.props;
const time = formatTimeline(item);
---
{time && `${time} `}<span class="label">{item.title}</span>
{item.detail && <span class="label"> - {item.detail}</span>}
```

## 8. エントリページ (`src/pages/index.astro`)
```astro
---
import BaseLayout from "../layouts/BaseLayout.astro";
import SiteHeader from "../components/SiteHeader.astro";
import SectionBlock from "../components/SectionBlock.astro";
import { loadSiteData } from "../lib/content";

const data = await loadSiteData();
const ogDescription = `${data.profile.name.ja} (${data.profile.affiliation.affiliation})`;
---
<BaseLayout
  title={data.config.title}
  lang={data.config.language ?? "ja"}
  ogDescription={ogDescription}
  googleSiteVerification={data.config.google_site_verification}
>
  <SiteHeader profile={data.profile} sections={data.sections} />
  <main>
    {data.sections.map((section) => (
      <SectionBlock section={section} />
    ))}
  </main>
  <footer>
    <p>© {data.profile.name.en}</p>
  </footer>
</BaseLayout>
```

## 9. 実装時の注意
- `item.detail` は将来的に Markdown リンクを含むため、テキスト描画を関数化して安全に処理する。
- `formatJaDate` / `formatTimeline` は既存仕様（`YYYY年M月D日`, `YYYY年M月`）に合わせる。
- `sort/rev/order` は表示前に `loadSiteData()` で確定させる。
