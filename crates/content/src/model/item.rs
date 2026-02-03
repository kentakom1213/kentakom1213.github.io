use serde::Deserialize;

/// すべての「箇条書き項目」を包括的に受ける
///
/// - 資格：date + title (+ detail)
/// - 学歴/職歴：start_date + (end_date) + title (+ detail)
/// - 発表：authors + title + venue + location + date
#[derive(Debug, Deserialize, Clone)]
pub struct ItemToml {
    // 単発
    pub date: Option<String>,

    // 期間（end は省略可）
    pub start_date: Option<String>,
    pub end_date: Option<String>,

    // 発表用（構造化）
    #[serde(default)]
    pub authors: Vec<String>,
    pub venue: Option<String>,
    pub location: Option<String>,

    // 共通
    pub title: String,

    // 資格/学歴/職歴などの補足
    pub detail: Option<String>,
}
