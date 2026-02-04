use std::fs;
use std::path::Path;

use content::{LoadOptions, load_all};
use tempfile::tempdir;

fn write(path: &Path, body: &str) {
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).unwrap();
    }
    fs::write(path, body).unwrap();
}

#[test]
fn load_all_reads_sections_and_sorts() {
    let temp = tempdir().unwrap();
    let content_dir = temp.path().join("content");
    let sections_dir = content_dir.join("sections");

    write(
        &content_dir.join("config.toml"),
        r#"title = "Test Site"
"#,
    );

    write(
        &content_dir.join("profile.toml"),
        r#"[name]
ja = "Taro"
en = "Taro"

[affiliation]
affiliation = "Example University"

[contact]
email = "taro@example.com"
"#,
    );

    write(
        &sections_dir.join("b.toml"),
        r#"name = "Section B"
key = "b"
order = 1
"#,
    );

    write(
        &sections_dir.join("a.toml"),
        r#"name = "Section A"
key = "a"
order = 2

[[items]]
date = "2020-01-01"
title = "Old"

[[items]]
start_date = "2021-05"
title = "New"

[[subsections]]
name = "Sub A1"
order = 1

  [[subsections.items]]
  date = "2019"
  title = "Sub Old"

  [[subsections.items]]
  start_date = "2022-01-01"
  title = "Sub New"
"#,
    );

    // ネストされた toml は読み込まれないことを確認
    write(
        &sections_dir.join("nested").join("ignored.toml"),
        r#"name = "Ignored"
key = "ignored"
order = 0
"#,
    );

    let data = load_all(LoadOptions {
        content_dir: content_dir.clone(),
        sort_items: true,
    })
    .unwrap();

    assert_eq!(data.sections.len(), 2);
    assert_eq!(data.sections[0].key, "b");
    assert_eq!(data.sections[1].key, "a");

    let items = &data.sections[1].items;
    assert_eq!(items.len(), 2);
    assert_eq!(items[0].title, "New");
    assert_eq!(items[1].title, "Old");

    let sub_items = &data.sections[1].subsections[0].items;
    assert_eq!(sub_items.len(), 2);
    assert_eq!(sub_items[0].title, "Sub New");
    assert_eq!(sub_items[1].title, "Sub Old");
}
