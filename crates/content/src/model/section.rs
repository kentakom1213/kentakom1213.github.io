use serde::Deserialize;

use crate::model::item::ItemToml;

/// h2 に相当するセクション
#[derive(Debug, Deserialize, Clone)]
pub struct SectionToml {
    pub name: String,
    pub key: String, // h2 の id に使う
    pub order: Option<i32>,

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

    #[serde(default)]
    pub items: Vec<ItemToml>,
}
