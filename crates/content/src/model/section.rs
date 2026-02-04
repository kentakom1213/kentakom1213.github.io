use serde::Deserialize;

use crate::model::item::ItemToml;

/// アイテムをソートする際のキー
#[derive(Debug, Deserialize, Clone, Copy)]
#[serde(rename_all = "snake_case")]
pub enum SortKey {
    /// 日付
    Date,
    /// タイトルの辞書順
    Title,
}

/// h2 に相当するセクション
#[derive(Debug, Deserialize, Clone)]
pub struct SectionToml {
    pub name: String,
    pub key: String, // h2 の id に使う
    pub order: Option<i32>,

    /// アイテムをソートする際のキー
    pub sort: Option<SortKey>,
    /// 逆順（降順）にするか
    pub rev: Option<bool>,

    // 番号付きで表示するか
    #[serde(default)]
    pub numbering: bool,

    #[serde(default)]
    pub items: Vec<ItemToml>,

    #[serde(default)]
    pub subsections: Vec<SubsectionToml>,
}

/// h3 に相当するサブセクション
#[derive(Debug, Deserialize, Clone)]
pub struct SubsectionToml {
    pub name: String,
    pub order: Option<i32>,

    /// アイテムをソートする際のキー
    pub sort: Option<SortKey>,
    /// 逆順（降順）にするか
    pub rev: Option<bool>,

    // 番号付きで表示するか
    #[serde(default)]
    pub numbering: bool,

    #[serde(default)]
    pub items: Vec<ItemToml>,
}
