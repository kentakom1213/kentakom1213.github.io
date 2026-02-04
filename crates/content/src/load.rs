use anyhow::{Context, bail};
use std::path::{Path, PathBuf};
use std::{cmp, fs};

use crate::model::{self, ConfigToml, IndexData, ProfileToml, SectionToml, SortKey};

#[derive(Debug, Clone)]
pub struct LoadOptions {
    /// 例：content/
    pub content_dir: PathBuf,
}

impl Default for LoadOptions {
    fn default() -> Self {
        Self {
            content_dir: PathBuf::from("content"),
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
    sections.sort_by_key(|s| s.order);

    // セクションの中身をソート
    for sec in &mut sections {
        // アイテムをソート
        sort_items(sec.sort, sec.rev, &mut sec.items);

        // サブセクションをソート
        sec.subsections.sort_by_key(|s| s.order);

        // サブセクションの中身をソート
        for ssec in &mut sec.subsections {
            sort_items(ssec.sort, ssec.rev, &mut ssec.items);
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

fn sort_items(sort: Option<SortKey>, rev: Option<bool>, items: &mut [model::ItemToml]) {
    let Some(sort) = sort else {
        return;
    };
    let rev = rev.unwrap_or(false);

    items.sort_by(get_item_sort_by(sort, rev));
}

/// アイテム同士を比較する関数を返す
///
/// キーの優先順位:
/// - date がない場合，最も後ろに送る
/// - start_date と date がある場合は start_date を優先
fn get_item_sort_by(
    sort: SortKey,
    rev: bool,
) -> impl FnMut(&model::ItemToml, &model::ItemToml) -> cmp::Ordering {
    let key = move |item: &model::ItemToml| -> (bool, String) {
        match sort {
            SortKey::Title => (false, item.title.clone()),
            SortKey::Date => item_date_key(&[&item.date, &item.start_date, &item.end_date]),
            SortKey::StartDate => item_date_key(&[&item.start_date, &item.date, &item.end_date]),
            SortKey::EndDate => item_date_key(&[&item.end_date, &item.date, &item.start_date]),
        }
    };
    move |a, b| {
        let (mut ka, mut kb) = (key(a), key(b));
        // None は常に末尾に
        ka.0 ^= rev;
        kb.0 ^= rev;

        if rev {
            ka.cmp(&kb).reverse()
        } else {
            ka.cmp(&kb)
        }
    }
}

fn item_date_key(dates: &[&Option<String>]) -> (bool, String) {
    let first = dates.into_iter().find_map(|x| x.as_ref());
    let Some(date) = first else {
        return (true, String::new());
    };
    (false, normalize_date_key(date))
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
