use crate::options::OPTIONS;
use askama::Template;
use serde::Deserialize;
use warp::{
    reject::Rejection,
    reply::{html, Reply},
};

use crate::{
    algorithms::{entries::StrippedDocument, SearchAlgorithmType},
    errors, ALGORITHMS, ENTRIES,
};

#[derive(Deserialize)]
pub struct SearchResultsQuery {
    page: u16,
    query: String,
    algorithm: SearchAlgorithmType,
}

#[derive(Template)]
#[template(path = "search.html")]
struct SearchTemplate<'a> {
    algorithms: Vec<&'a SearchAlgorithmType>,
}

#[derive(Template)]
#[template(path = "search-results.html")]
struct SearchResultTemplate<'a> {
    pages: u16,
    results: Vec<&'a StrippedDocument>,
}

pub async fn results(query: SearchResultsQuery) -> Result<impl Reply, Rejection> {
    let algorithms = ALGORITHMS.get().expect("algorithms cell should be full");
    let entries_algorithm = ENTRIES.get().expect("entries cell should be full");

    let results = algorithms
        .get(&query.algorithm)
        .ok_or(warp::reject::custom(errors::IncorrectParameters))?
        .search(
            &query.query,
            (OPTIONS.pages as u64) * OPTIONS.page_size as u64,
        )
        .or_else(|error| Err(warp::reject::custom(errors::Failed(error))))?;

    let offset = OPTIONS.page_size * (query.page as u32 - 1);
    let end = (offset + OPTIONS.page_size).min(results.len() as u32) as usize;
    let pages = ((results.len() as f32 / OPTIONS.page_size as f32).ceil() as u16).max(1);
    let results = entries_algorithm.transform(results[offset as usize..end].to_vec());

    let template = SearchResultTemplate { pages, results };

    let rendered = template
        .render()
        .or_else(|error| Err(warp::reject::custom(errors::Failed(error.into()))))?;

    Ok(html(rendered))
}

pub async fn main() -> Result<impl Reply, Rejection> {
    let algorithms = ALGORITHMS.get().expect("algorithms cell should be full");
    let template = SearchTemplate {
        algorithms: algorithms.keys().collect(),
    };

    let rendered = template
        .render()
        .or_else(|error| Err(warp::reject::custom(errors::Failed(error.into()))))?;

    Ok(html(rendered))
}
