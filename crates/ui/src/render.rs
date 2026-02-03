use content::model::{IndexData, ItemToml, SectionToml, SubsectionToml};
use maud::{DOCTYPE, Markup, html};

pub fn render_index(data: &IndexData) -> Markup {
    let lang = data.config.language.as_deref().unwrap_or("ja");

    // CSS のパス（site.toml に合わせて可変にできるが，まずは固定で十分）
    let css_href = "/assets/style.css";

    html! {
        (DOCTYPE)
        html lang=(lang) {
            head {
                meta charset="utf-8";
                meta name="viewport" content="width=device-width, initial-scale=1";
                title { (data.config.title) }
                link rel="stylesheet" href=(css_href);
            }
            body {
                (render_header(data))

                main {
                    @for sec in &data.sections {
                        (render_section(sec))
                    }
                }

                footer {
                    // 太字は使わない方針にする
                    p { "© " (data.profile.name.en) }
                }
            }
        }
    }
}

pub fn render_index_string(data: &IndexData) -> String {
    render_index(data).into_string()
}

fn render_header(data: &IndexData) -> Markup {
    let p = &data.profile;

    html! {
        header {
            h1 { (p.name.en) }
            p { (p.name.ja) }

            @for line in &p.affiliation.lines {
                p { (line) }
            }

            p {
                "Email: " (p.contact.email)
            }

            @if let Some(lead) = &p.lead {
                p { (lead) }
            }

            // 目次（任意だが便利：h2 に id が付く前提）
            nav {
                ul {
                    @for sec in &data.sections {
                        li {
                            a href=(format!("#{}", sec.key)) { (sec.name) }
                        }
                    }
                }
            }
        }
    }
}

fn render_section(sec: &SectionToml) -> Markup {
    html! {
        section {
            h2 id=(sec.key) { (sec.name) }

            @if !sec.items.is_empty() {
                (render_generic_items(&sec.items))
            }

            @if !sec.subsections.is_empty() {
                @for sub in &sec.subsections {
                    (render_subsection(sub))
                }
            }
        }
    }
}

fn render_subsection(sub: &SubsectionToml) -> Markup {
    html! {
        h3 { (sub.name) }
        @if !sub.items.is_empty() {
            (render_generic_items(&sub.items))
        } @else {
            // 空なら何も出さない（必要なら "None" などを出す）
        }
    }
}

/// 1 つの `ItemToml` を「研究発表フォーマット」または「一般フォーマット」で 1 行にする
fn render_generic_items(items: &[ItemToml]) -> Markup {
    html! {
        ul {
            @for it in items {
                li { (render_item_line(it)) }
            }
        }
    }
}

fn render_item_line(it: &ItemToml) -> Markup {
    // research item 判定：authors があり，venue/location/date がそれっぽい
    if is_publication_like(it) {
        return render_publication_like(it);
    }
    render_timeline_like(it)
}

fn is_publication_like(it: &ItemToml) -> bool {
    !it.authors.is_empty()
        && !it.venue.as_deref().unwrap_or("").trim().is_empty()
        && !it.location.as_deref().unwrap_or("").trim().is_empty()
        && !it.date.as_deref().unwrap_or("").trim().is_empty()
}

/// 指定フォーマット：
/// `○著者, "タイトル," 会議名, 開催地, 2025年7月23日`
fn render_publication_like(it: &ItemToml) -> Markup {
    let authors = it.authors.join(", ");
    let title = &it.title;
    let venue = it.venue.as_deref().unwrap_or("");
    let location = it.location.as_deref().unwrap_or("");
    let date_ja = it.date.as_deref().map(format_date_ja).unwrap_or_default();

    html! {
        (authors)
        ", "
        "\""(title)",\" "
        (venue)
        ", "
        (location)
        ", "
        (date_ja)
    }
}

/// 資格・学歴・職歴など：
/// - `date` があればそれを表示
/// - なければ `start_date` と `end_date`（end は省略可）
/// - その後に `title`，必要なら `detail`
fn render_timeline_like(it: &ItemToml) -> Markup {
    let time = format_time(it);

    html! {
        @if let Some(t) = time {
            (t) " "
        }
        (it.title)
        @if let Some(detail) = it.detail.as_deref() {
            @if !detail.trim().is_empty() {
                " — " (detail)
            }
        }
    }
}

fn format_time(it: &ItemToml) -> Option<String> {
    if let Some(d) = it.date.as_deref() {
        let d = d.trim();
        if !d.is_empty() {
            // 研究発表は別フォーマットで日付を末尾に出すので，
            // publication 判定のときはここは呼ばれない想定
            return Some(format_date_or_month_ja(d));
        }
    }

    let s = it.start_date.as_deref()?.trim();
    if s.is_empty() {
        return None;
    }
    let s = format_date_or_month_ja(s);

    let e = it
        .end_date
        .as_deref()
        .map(|x| x.trim())
        .filter(|x| !x.is_empty());
    let e = e
        .map(format_date_or_month_ja)
        .unwrap_or_else(|| "現在".to_string());

    Some(format!("{}–{}", s, e))
}

/// `YYYY-MM-DD` → `YYYY年M月D日`
/// `YYYY-MM` → `YYYY年M月`
/// `YYYY` → `YYYY年`
fn format_date_or_month_ja(s: &str) -> String {
    let parts: Vec<&str> = s.split('-').collect();
    match parts.len() {
        1 => format!("{}年", parts[0]),
        2 => {
            let y = parts[0];
            let m = parts[1].trim_start_matches('0');
            format!("{}年{}月", y, m)
        }
        _ => {
            let y = parts[0];
            let m = parts[1].trim_start_matches('0');
            let d = parts[2].trim_start_matches('0');
            format!("{}年{}月{}日", y, m, d)
        }
    }
}

/// publication は必ず日付が `YYYY-MM-DD` 想定だが，ゆるく扱う
fn format_date_ja(s: &str) -> String {
    format_date_or_month_ja(s)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn item_publication() -> ItemToml {
        ItemToml {
            date: Some("2025-07-03".to_string()),
            start_date: None,
            end_date: None,
            authors: vec!["A".to_string(), "B".to_string()],
            venue: Some("Conf".to_string()),
            location: Some("Tokyo".to_string()),
            title: "Paper".to_string(),
            detail: None,
        }
    }

    fn item_timeline(date: &str, detail: Option<&str>) -> ItemToml {
        ItemToml {
            date: Some(date.to_string()),
            start_date: None,
            end_date: None,
            authors: vec![],
            venue: None,
            location: None,
            title: "Item".to_string(),
            detail: detail.map(|d| d.to_string()),
        }
    }

    #[test]
    fn format_date_or_month_ja_handles_year_month_day() {
        assert_eq!(format_date_or_month_ja("2025"), "2025年");
        assert_eq!(format_date_or_month_ja("2025-07"), "2025年7月");
        assert_eq!(format_date_or_month_ja("2025-07-03"), "2025年7月3日");
    }

    #[test]
    fn format_time_uses_date_or_range() {
        let it = item_timeline("2023-02", None);
        assert_eq!(format_time(&it).as_deref(), Some("2023年2月"));

        let it = ItemToml {
            date: None,
            start_date: Some("2020-04".to_string()),
            end_date: None,
            authors: vec![],
            venue: None,
            location: None,
            title: "Item".to_string(),
            detail: None,
        };
        assert_eq!(format_time(&it).as_deref(), Some("2020年4月–現在"));

        let it = ItemToml {
            date: None,
            start_date: Some("2020-04".to_string()),
            end_date: Some("2022-03".to_string()),
            authors: vec![],
            venue: None,
            location: None,
            title: "Item".to_string(),
            detail: None,
        };
        assert_eq!(format_time(&it).as_deref(), Some("2020年4月–2022年3月"));
    }

    #[test]
    fn render_publication_like_format() {
        let html = render_publication_like(&item_publication()).into_string();
        assert!(html.contains("A, B, "));
        assert!(html.contains("&quot;Paper,&quot;"));
        assert!(html.contains("Conf, Tokyo, 2025年7月3日"));
    }

    #[test]
    fn render_item_line_switches_by_publication_like() {
        let pub_html = render_item_line(&item_publication()).into_string();
        assert!(pub_html.contains("A, B, "));
        assert!(pub_html.contains("&quot;Paper,&quot;"));
        assert!(pub_html.contains("Conf, Tokyo, 2025年7月3日"));

        let timeline = item_timeline("2020-01", Some("Detail"));
        let timeline_html = render_item_line(&timeline).into_string();
        assert!(timeline_html.contains("2020年1月 Item — Detail"));
    }

    #[test]
    fn render_index_string_includes_header_and_sections() {
        let data = IndexData {
            config: content::model::ConfigToml {
                title: "My Site".to_string(),
                language: None,
                build: None,
                assets: None,
            },
            profile: content::model::ProfileToml {
                name: content::model::Name {
                    ja: "兼".to_string(),
                    en: "Ken".to_string(),
                },
                affiliation: content::model::Affiliation {
                    lines: vec!["Uni".to_string()],
                },
                contact: content::model::Contact {
                    email: "a@example.com".to_string(),
                },
                lead: Some("Lead".to_string()),
            },
            sections: vec![content::model::SectionToml {
                name: "Research".to_string(),
                key: "research".to_string(),
                order: None,
                items: vec![item_publication()],
                subsections: vec![content::model::SubsectionToml {
                    name: "Sub".to_string(),
                    order: None,
                    items: vec![item_timeline("2020-01", Some("Detail"))],
                }],
            }],
        };

        let html = render_index_string(&data);
        assert!(html.contains("lang=\"ja\""));
        assert!(html.contains("<title>My Site</title>"));
        assert!(html.contains("Ken"));
        assert!(html.contains("兼"));
        assert!(html.contains("Email: a@example.com"));
        assert!(html.contains("href=\"/assets/style.css\""));
        assert!(html.contains("href=\"#research\""));
        assert!(html.contains("<h2 id=\"research\">Research</h2>"));
        assert!(html.contains("<h3>Sub</h3>"));
        assert!(html.contains("2020年1月 Item — Detail"));
        assert!(html.contains("© Ken"));
    }
}
