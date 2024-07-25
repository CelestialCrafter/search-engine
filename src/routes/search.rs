use crate::options::OPTIONS;
use askama::Template;
use eyre::Result;
use serde::Deserialize;
use warp::{
    reject::Rejection,
    reply::{html, Reply},
};

use crate::{
    algorithms::{entries::StrippedDocument, SearchAlgorithmType, SearchResponseAmount},
    errors, ALGORITHMS, ENTRIES,
};

#[derive(Deserialize)]
pub struct SearchResultsQuery {
    page: Option<u16>,
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
    page: u16,
    results: Vec<&'a StrippedDocument>,
}

fn fetch_algorithm_results(query: &SearchResultsQuery) -> Result<Vec<String>, Rejection> {
    let algorithms = ALGORITHMS.get().expect("algorithms cell should be full");
    algorithms
        .get(&query.algorithm)
        .ok_or(warp::reject::custom(errors::IncorrectParameters))?
        .search(
            &query.query,
            OPTIONS.pages as SearchResponseAmount * OPTIONS.page_size as SearchResponseAmount,
        )
        .or_else(|error| Err(warp::reject::custom(errors::Failed(error))))
}

pub async fn results(query: SearchResultsQuery) -> Result<impl Reply, Rejection> {
    let entries_algorithm = ENTRIES.get().expect("entries cell should be full");
    let results = fetch_algorithm_results(&query)?;

    let page = query.page.unwrap_or(1).max(1);
    let offset = (OPTIONS.page_size * (page as u32 - 1)) as usize;
    let end: usize;
    let pages = ((results.len() as u32 / OPTIONS.page_size) as u16).max(1);

    if query.page.is_some() {
        end = (offset + OPTIONS.page_size as usize).min(results.len());
    } else {
        end = results.len();
    }

    let results = entries_algorithm.transform(results[offset..end].to_vec());

    let template = SearchResultTemplate {
        page: query.page.unwrap_or(0),
        pages,
        results,
    };

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
