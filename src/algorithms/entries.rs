use eyre::Result;
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, fs};

use crate::{crawled::Document, options::OPTIONS};

const SAVE_FILE: &str = "entries.bin";

#[derive(Serialize, Deserialize, Clone)]
pub struct StrippedDocument {
    pub url: String,
    pub crawled_at: i64,
    pub description: String,
    pub title: String,
    pub site: String,
}

#[derive(Serialize, Deserialize)]
pub struct Entries {
    documents: HashMap<String, StrippedDocument>,
}

impl Entries {
    pub fn new() -> Self {
        Entries {
            documents: HashMap::new(),
        }
    }

    pub fn transform(&self, entries: Vec<String>) -> Vec<&StrippedDocument> {
        entries
            .into_iter()
            .map(|url| self.documents.get(&url))
            .filter_map(|d| d)
            .collect()
    }

    pub fn compute(&mut self, documents: Vec<Document>) -> Result<()> {
        documents.into_iter().try_for_each(|document| {
            let metadata = document.metadata.expect("this should never happen.");
            let url = document
                .url
                .expect("documents without urls are not supported yet");

            self.documents.insert(
                url.clone(),
                StrippedDocument {
                    url,
                    crawled_at: metadata.crawled_at.map_or(0, |v| v.seconds),
                    description: metadata.description.unwrap_or("".to_string()),
                    title: metadata.title.unwrap_or("".to_string()),
                    site: metadata.site.unwrap_or("".to_string()),
                },
            );

            Ok(())
        })
    }

    pub fn save(&self) -> Result<()> {
        let encoded = bincode::serialize(self)?;
        fs::write(OPTIONS.data_path.join(SAVE_FILE), encoded)?;
        Ok(())
    }

    pub fn load(&mut self) -> Result<()> {
        let data = fs::read(OPTIONS.data_path.join(SAVE_FILE))?;
        let decoded = bincode::deserialize(&data)?;
        *self = decoded;
        Ok(())
    }
}
