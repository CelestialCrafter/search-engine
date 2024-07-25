use std::{collections::HashMap, fs, str::from_utf8};

use eyre::Result;
use serde::{Deserialize, Serialize};

use crate::{crawled::Document, options::OPTIONS};

use super::{SearchAlgorithm, SearchResponseAmount};

const SAVE_FILE: &str = "bm25l.bin";

#[derive(Serialize, Deserialize)]
pub struct BM25L {
    k1: f32,
    b: f32,
    delta: f32,
    total_documents: u128,
    average_document_length: u32,
    document_frequencies: Vec<HashMap<String, u32>>,
    idf: HashMap<String, f32>,
    urls: Vec<String>,
}

fn calculate_tf(tokens: impl Iterator<Item = String>) -> HashMap<String, u32> {
    let mut frequencies = HashMap::new();
    for token in tokens {
        *frequencies.entry(token).or_insert(0) += 1;
    }

    frequencies
}

impl BM25L {
    pub fn new() -> Self {
        BM25L {
            k1: 1.2,
            b: 0.75,
            delta: 0.5,
            total_documents: 0,
            average_document_length: 0,
            document_frequencies: vec![],
            idf: HashMap::new(),
            urls: vec![],
        }
    }

    fn calculate_idf(&mut self, docs_with_token: HashMap<String, u32>) {
        for (token, frequency) in docs_with_token.into_iter() {
            let token_idf = (self.total_documents as f32 + 1.0).ln() + (frequency as f32 + 0.5);
            self.idf.insert(token, token_idf);
        }
    }
}

impl SearchAlgorithm for BM25L {
    fn search(&self, query: &str, max: SearchResponseAmount) -> eyre::Result<Vec<String>> {
        let query_tokens = query.split_whitespace().map(|token| token.to_lowercase());
        let mut document_scores: Vec<f32> = vec![0.0; self.urls.len()];

        for token in query_tokens {
            self.document_frequencies
                .iter()
                .enumerate()
                .map(|(i, frequencies)| (i, frequencies.get(&token), self.idf.get(&token)))
                .filter(|(_i, frequency, idf)| frequency.is_some() && idf.is_some())
                .map(|(i, frequency, idf)| (i, frequency.unwrap(), idf.unwrap()))
                .map(|(i, frequency, idf)| {
                    (
                        i,
                        *frequency as f32
                            / (1.0 - self.b
                                + self.b
                                    * (self.total_documents / self.average_document_length as u128)
                                        as f32),
                        idf,
                    )
                })
                .for_each(|(i, ctd, idf)| {
                    let score =
                        idf * (self.k1 + 1.0) * (ctd + self.delta) / (self.k1 + ctd + self.delta);
                    document_scores[i] += score;
                });
        }

        let mut urls_scores: Vec<(String, f32)> = document_scores
            .into_iter()
            .enumerate()
            .map(|(i, score)| (self.urls[i].clone(), score))
            .collect();

        urls_scores.sort_unstable_by(|(_a_url, a), (_b_url, b)| b.total_cmp(a));
        urls_scores.truncate(max as usize);
        Ok(urls_scores
            .into_iter()
            .filter(|(_url, score)| *score > 0.0)
            .map(|(url, _score)| url)
            .collect())
    }

    fn compute(&mut self, documents: Vec<Document>) -> Result<()> {
        self.total_documents = documents.len() as u128;
        let mut total_tokens = 0;
        documents
            .iter()
            .for_each(|document| self.urls.push(document.url().to_string()));
        let documents_tokenized = documents
            .iter()
            .filter_map(|d| from_utf8(d.text()).ok())
            .map(|d| d.split_whitespace().map(|s| s.to_lowercase()));

        documents_tokenized.clone().for_each(|document| {
            total_tokens += document.count() as u128;
        });

        let mut docs_with_token = HashMap::new();
        documents_tokenized
            .map(|document| calculate_tf(document.map(|s| s.to_string())))
            .for_each(|token_frequencies| {
                self.document_frequencies.push(token_frequencies.clone());
                for (token, _frequency) in token_frequencies.into_iter() {
                    *docs_with_token.entry(token).or_insert(0) += 1;
                }
            });

        self.calculate_idf(docs_with_token);
        self.average_document_length = (total_tokens / self.total_documents) as u32;

        Ok(())
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
