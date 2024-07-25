use eyre::{eyre, Result};
use serde::{Deserialize, Serialize};
use std::{
    fs,
    io::{Read, Write},
    process::{Command, Stdio},
};
use tracing::warn;

use crate::{crawled::Document, options::OPTIONS};

use super::SearchAlgorithm;

const SAVE_FILE: &str = "fuzzy.bin";

#[derive(Serialize, Deserialize, Debug)]
pub struct Fuzzy {
    urls: Vec<String>,
}

impl Fuzzy {
    pub fn new() -> Self {
        Fuzzy { urls: vec![] }
    }
}

impl SearchAlgorithm for Fuzzy {
    fn search(&self, query: &str, max: super::SearchResponseAmount) -> Result<Vec<String>> {
        let mut child = Command::new("fzy")
            .args(["-e", &query.to_lowercase(), "-l", &max.to_string()])
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .spawn()?;

        let mut stdin = child.stdin.take().ok_or(eyre!("could not take stdin"))?;

        let urls_str = self.urls.join("\n");
        let urls = urls_str.as_bytes();
        stdin.write_all(urls)?;
        stdin.flush()?;
        drop(stdin);

        let amount = self.urls.len();
        warn!(amount = ?amount, "pushed to stdin");

        child.wait()?;
        warn!("exited");

        let mut stdout = child.stdout.take().ok_or(eyre!("could not take stdout"))?;
        let mut output = "".to_string();
        stdout.read_to_string(&mut output)?;
        Ok(output.trim().split("\n").map(|s| s.to_string()).collect())
    }

    fn compute(&mut self, documents: Vec<Document>) -> Result<()> {
        documents.into_iter().try_for_each(|document| {
            let url = document.url().to_string();
            self.urls.push(url.clone());

            Ok(())
        })
    }

    fn save(&self) -> Result<()> {
        let encoded = bincode::serialize(self)?;
        fs::write(OPTIONS.data_path.join(SAVE_FILE), encoded)?;
        Ok(())
    }

    fn load(&mut self) -> Result<()> {
        let data = fs::read(OPTIONS.data_path.join(SAVE_FILE))?;
        let decoded = bincode::deserialize(&data)?;
        *self = decoded;
        Ok(())
    }
}
