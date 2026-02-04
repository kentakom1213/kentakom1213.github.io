use serde::Deserialize;

use crate::model::item::ItemToml;

/// アイテムをソートする際のキー
#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "snake_case")]
pub enum SortKey {
    /// 日付
    Date,
    /// 開始日付
    StartDate,
    /// 終了日付
    EndDate,
    /// タイトルの辞書順
    Title,
}

/// h2 に相当するセクション
#[derive(Debug, Deserialize, Clone)]
pub struct SectionToml {
    pub name: String,
    pub key: String, // h2 の id に使う
    pub order: Option<i32>,

    pub sort: Option<SortKey>,

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

    pub sort: Option<SortKey>,

    #[serde(default)]
    pub items: Vec<ItemToml>,
}
