use std::{fs::read_to_string, path::PathBuf};

use once_cell::sync::Lazy;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct Options {
    #[serde(default = "default_data_path")]
    pub data_path: PathBuf,
    #[serde(default = "default_search_files_path")]
    pub search_files_path: PathBuf,
    #[serde(default = "default_pages")]
    pub pages: u16,
    #[serde(default = "default_page_size")]
    pub page_size: u32,
}

fn default_data_path() -> PathBuf {
    "data/".into()
}

fn default_search_files_path() -> PathBuf {
    "search-files".into()
}

fn default_pages() -> u16 {
    10
}

fn default_page_size() -> u32 {
    100
}

const OPTIONS_PATH: &str = "options.toml";

pub static OPTIONS: Lazy<Options> = Lazy::new(|| {
    let data = read_to_string(OPTIONS_PATH).expect("reading options file should not fail");
    toml::from_str::<Options>(data.as_str()).expect("parsing options should not fail")
});
