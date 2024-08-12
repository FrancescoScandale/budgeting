use serde::Deserialize;

#[allow(non_snake_case)]
#[derive(Deserialize)]
pub struct ConfigData {
    pub CACHE_FILE: String,
    pub REPORT_FILE: String,
    pub CATEGORIES_FILE: String
}