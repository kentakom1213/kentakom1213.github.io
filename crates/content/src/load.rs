use anyhow::{Context, bail};
use std::fs;
use std::path::{Path, PathBuf};

use crate::model::{ConfigToml, IndexData, ProfileToml, SectionToml};

#[derive(Debug, Clone)]
pub struct LoadOptions {
    /// 例：content/
    pub content_dir: PathBuf,

    /// items を日付でソートするか
    pub sort_items: bool,
}

impl Default for LoadOptions {
    fn default() -> Self {
        Self {
            content_dir: PathBuf::from("content"),
            sort_items: true,
        }
    }
}

pub fn load_all(opt: LoadOptions) -> anyhow::Result<IndexData> {
    let content_dir = &opt.content_dir;

    let site_path = content_dir.join("config.toml");
    let profile_path = content_dir.join("profile.toml");
    let sections_dir = content_dir.join("sections");

    let config: ConfigToml =
        read_toml(&site_path).with_context(|| format!("failed to read {}", site_path.display()))?;
    let profile: ProfileToml = read_toml(&profile_path)
        .with_context(|| format!("failed to read {}", profile_path.display()))?;

    let mut sections = read_sections(&sections_dir)
        .with_context(|| format!("failed to read sections from {}", sections_dir.display()))?;

    // ソート
    sections.sort_by_key(section_sort_key);

    for sec in &mut sections {
        sec.subsections.sort_by_key(subsection_sort_key);

        if opt.sort_items {
            sort_items_in_section(sec);
        }
    }

    Ok(IndexData {
        config,
        profile,
        sections,
    })
}

fn read_sections(dir: &Path) -> anyhow::Result<Vec<SectionToml>> {
    let mut out = Vec::new();

    for entry in
        fs::read_dir(dir).with_context(|| format!("failed to read directory: {}", dir.display()))?
    {
        let entry = entry?;
        let meta = entry.metadata()?;
        if !meta.is_file() {
            continue;
        }
        let path = entry.path();
        if path.extension().and_then(|s| s.to_str()) != Some("toml") {
            continue;
        }

        let sec: SectionToml = read_toml(&path)
            .with_context(|| format!("failed to parse section toml: {}", path.display()))?;

        // 最低限のバリデーション
        if sec.key.trim().is_empty() {
            bail!("section key is empty: {}", path.display());
        }

        out.push(sec);
    }

    Ok(out)
}

fn read_toml<T: serde::de::DeserializeOwned>(path: &Path) -> anyhow::Result<T> {
    let s = fs::read_to_string(path)?;
    let v = toml::from_str::<T>(&s)?;
    Ok(v)
}

fn section_sort_key(sec: &crate::model::SectionToml) -> (i32, String) {
    (sec.order.unwrap_or(1000), sec.key.clone())
}

fn subsection_sort_key(sub: &crate::model::SubsectionToml) -> (i32, String) {
    (sub.order.unwrap_or(1000), sub.name.clone())
}

fn sort_items_in_section(sec: &mut crate::model::SectionToml) {
    sec.items.sort_by(item_sort_key_desc);
    for sub in &mut sec.subsections {
        sub.items.sort_by(item_sort_key_desc);
    }
}

/// 降順（新しいものが上）
///
/// 優先順位：start_date > date > ""（不明）
fn item_sort_key_desc(
    a: &crate::model::ItemToml,
    b: &crate::model::ItemToml,
) -> std::cmp::Ordering {
    let ka = item_key(a);
    let kb = item_key(b);
    kb.cmp(&ka)
}

/// ソート用キー（辞書順比較できる形に正規化）
///
/// 許容：
/// - YYYY
/// - YYYY-MM
/// - YYYY-MM-DD
///
/// 正規化：
/// - YYYY       -> YYYY-00-00
/// - YYYY-MM    -> YYYY-MM-00
/// - YYYY-MM-DD -> YYYY-MM-DD
fn item_key(it: &crate::model::ItemToml) -> String {
    if let Some(s) = it.start_date.as_deref() {
        return normalize_date_key(s);
    }
    if let Some(s) = it.date.as_deref() {
        return normalize_date_key(s);
    }
    String::new()
}

fn normalize_date_key(s: &str) -> String {
    let parts: Vec<&str> = s.split('-').collect();
    match parts.len() {
        1 => format!("{:0>4}-00-00", parts[0]),
        2 => format!("{:0>4}-{:0>2}-00", parts[0], parts[1]),
        _ => {
            // 3 以上は先頭 3 つだけ見る
            let y = parts[0];
            let m = parts[1];
            let d = parts[2];
            format!("{y:0>4}-{m:0>2}-{d:0>2}")
        }
    }
}
