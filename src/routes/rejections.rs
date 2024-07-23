use warp::{
    http::StatusCode,
    reject::Rejection,
    reply::{self, Reply},
};

use crate::errors::{Failed, IncorrectParameters};

pub async fn handle_rejection(rejection: Rejection) -> Result<impl Reply, Rejection> {
    if rejection.is_not_found() {
        Ok(reply::with_status(
            "Not Found".to_string(),
            StatusCode::NOT_FOUND,
        ))
    } else if let Some(error) = rejection.find::<Failed>() {
        Ok(reply::with_status(
            error.0.to_string(),
            StatusCode::INTERNAL_SERVER_ERROR,
        ))
    } else if let Some(..) = rejection.find::<IncorrectParameters>() {
        Ok(reply::with_status(
            "Algorithm Not Found".to_string(),
            StatusCode::BAD_REQUEST,
        ))
    } else {
        Err(rejection)
    }
}
