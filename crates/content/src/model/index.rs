use crate::model::{config::ConfigToml, profile::ProfileToml, section::SectionToml};

/// content から読み込んだものをまとめた最終モデル
#[derive(Debug, Clone)]
pub struct IndexData {
    pub config: ConfigToml,
    pub profile: ProfileToml,
    pub sections: Vec<SectionToml>,
}
