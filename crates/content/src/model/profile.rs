use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub struct ProfileToml {
    pub name: Name,
    pub affiliation: Affiliation,
    pub contact: Contact,
    pub lead: Option<String>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Name {
    pub ja: String,
    pub en: String,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Affiliation {
    pub affiliation: String,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Contact {
    pub email: String,
}
