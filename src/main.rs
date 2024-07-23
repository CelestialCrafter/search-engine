use algorithms::{entries::Entries, preflight, SearchAlgorithm, SearchAlgorithmType};
use eyre::eyre;
use once_cell::sync::OnceCell;
use routes::search::SearchResultsQuery;
use std::collections::HashMap;
use tracing_subscriber::fmt::format::FmtSpan;
use warp::Filter;

pub mod algorithms;
pub mod errors;
pub mod options;
pub mod routes;
pub mod crawled {
    include!(concat!(env!("OUT_DIR"), "/crawler.rs"));
}

pub static ALGORITHMS: OnceCell<HashMap<SearchAlgorithmType, Box<dyn SearchAlgorithm>>> =
    OnceCell::new();
pub static ENTRIES: OnceCell<Entries> = OnceCell::new();

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .with_span_events(FmtSpan::CLOSE)
        .init();

    let (entries_algorithm, preflight_result) = preflight().expect("preflight should not fail");

    ENTRIES
        .set(entries_algorithm)
        .or(Err(eyre!("cell was full")))
        .expect("algorithms cell should be empty");

    ALGORITHMS
        .set(preflight_result)
        .or(Err(eyre!("cell was full")))
        .expect("algorithms cell should be empty");

    let search_results_route = warp::path!("search" / "results")
        .and(warp::get())
        .and(warp::query::<SearchResultsQuery>())
        .and_then(routes::search::results);

    let search_route = warp::path!("search")
        .and(warp::get())
        .and_then(routes::search::main);

    let routes = search_route
        .or(search_results_route)
        .recover(routes::rejections::handle_rejection)
        .with(warp::trace::request());

    warp::serve(routes).run(([127, 0, 0, 1], 8080)).await;
}
