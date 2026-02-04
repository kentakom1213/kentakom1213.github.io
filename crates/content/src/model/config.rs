use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub struct ConfigToml {
    pub title: String,
    pub language: Option<String>,
    pub google_site_verification: Option<String>,
    pub build: Option<Build>,
    pub assets: Option<Assets>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Build {
    pub output_dir: Option<String>,
    pub output_file: Option<String>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Assets {
    pub dir: Option<String>,
    pub mount_path: Option<String>,
}
