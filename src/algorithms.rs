use std::{
    collections::HashMap,
    fmt::{self, Debug},
    fs::{self, OpenOptions},
    io::{Cursor, Read, Write},
};

use bm25l::BM25L;
use entries::Entries;
use eyre::Result;
use fuzzy::Fuzzy;
use prost::Message;
use serde::Deserialize;
use sha1::{Digest, Sha1};
use tracing::info;
use walkdir::WalkDir;

use crate::{crawled::Document, options::OPTIONS};

pub mod bm25l;
pub mod entries;
pub mod fuzzy;

type SearchResponse = String;
type SeachResponseAmount = u64;

#[derive(Eq, Hash, PartialEq, Debug, Deserialize, Copy, Clone)]
pub enum SearchAlgorithmType {
    Fuzzy,
    BM25L,
}

impl fmt::Display for SearchAlgorithmType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl From<SearchAlgorithmType> for Box<dyn SearchAlgorithm> {
    fn from(algorithm_type: SearchAlgorithmType) -> Self {
        match algorithm_type {
            SearchAlgorithmType::Fuzzy => Box::new(Fuzzy::new()),
            SearchAlgorithmType::BM25L => Box::new(BM25L::new()),
        }
    }
}

pub trait SearchAlgorithm: Sync + Send {
    fn search(&self, query: &str, max: SeachResponseAmount) -> Result<Vec<SearchResponse>>;
    fn compute(&mut self, documents: Vec<Document>) -> Result<()>;
    fn save(&self) -> Result<()>;
    fn load(&mut self) -> Result<()>;
}

fn should_compute(paths: Vec<String>) -> Result<(bool, Option<impl FnOnce() -> Result<()>>)> {
    let mut sum = paths.into_iter().fold("".to_string(), |acc, x| {
        let mut hasher = Sha1::new();
        hasher.update(x);
        format!("{}{:x}", acc, hasher.finalize())
    });

    let mut hasher = Sha1::new();
    hasher.update(sum);
    sum = format!("{:x}", hasher.finalize());

    let mut file = OpenOptions::new()
        .create(true)
        .read(true)
        .write(true)
        .open(OPTIONS.data_path.join("search-files-hash"))?;

    let mut data = "".to_string();
    file.read_to_string(&mut data)?;

    if sum == data {
        return Ok((false, None));
    }

    Ok((true, Some(move || Ok(file.write_all(sum.as_bytes())?))))
}

pub fn preflight() -> Result<(
    Entries,
    HashMap<SearchAlgorithmType, Box<dyn SearchAlgorithm>>,
)> {
    let walker = WalkDir::new(OPTIONS.search_files_path.clone());

    let paths: Vec<String> = walker
        .into_iter()
        .map(|entry| entry.expect("walk should not error"))
        .map(|entry| entry.path().to_path_buf())
        .filter(|path| path.is_file())
        .map(|path| path.into_os_string().into_string())
        .map(|path| path.expect("path should be utf-8"))
        .collect();

    let (compute, after_compute) = should_compute(paths.clone())?;
    let mut algorithms: HashMap<SearchAlgorithmType, Box<dyn SearchAlgorithm>> = HashMap::new();

    if compute {
        let fuzzy = SearchAlgorithmType::Fuzzy;
        let bm25l = SearchAlgorithmType::BM25L;
        algorithms.insert(fuzzy, fuzzy.into());
        algorithms.insert(bm25l, bm25l.into());
    }

    let mut documents = vec![];
    if compute {
        documents = paths
            .into_iter()
            .map(|path| fs::read(path))
            .map(|data| data.expect("reading document should not fail"))
            .map(|data| Document::decode(&mut Cursor::new(data)))
            .map(|data| data.expect("decoding document should not fail"))
            .collect()
    }

    let mut entries_algorithm = Entries::new();
    if compute {
        entries_algorithm.compute(documents.clone())?;
        entries_algorithm.save()?;
        info!("computed entries")
    } else {
        entries_algorithm.load()?;
        info!("loaded entries")
    }

    for algorithm_type in [SearchAlgorithmType::Fuzzy, SearchAlgorithmType::BM25L] {
        let mut algorithm: Box<dyn SearchAlgorithm> = algorithm_type.into();
        if compute {
            algorithm.compute(documents.clone())?;
            algorithm.save()?;
            info!(algorithm = algorithm_type.to_string(), "computed algorithm")
        } else {
            algorithm.load()?;
            info!(algorithm = algorithm_type.to_string(), "loaded algorithm")
        }

        algorithms.insert(algorithm_type, algorithm);
    }

    if let Some(after_compute) = after_compute {
        after_compute()?;
    }

    Ok((entries_algorithm, algorithms))
}
