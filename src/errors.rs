use axum::{http::StatusCode, response::IntoResponse, response::Response};
use maud::html;
use thiserror::Error;

use crate::brp::content;

#[derive(Debug, Error)]
pub enum ApiError {
    #[error("SQL error: {0}")]
    SQL(#[from] sqlx::Error),
    #[error("HTTP request error: {0}")]
    Request(#[from] reqwest::Error),
    #[error("OAuth token error: {0}")]
    TokenError(
        #[from]
        oauth2::RequestTokenError<
            oauth2::reqwest::Error<reqwest::Error>,
            oauth2::StandardErrorResponse<oauth2::basic::BasicErrorResponseType>,
        >,
    ),
    #[error("You're not authorized!")]
    Unauthorized,
    #[error("Attempted to get a non-none value but found none")]
    OptionError,
    #[error("Attempted to parse a number to an integer but errored out: {0}")]
    ParseIntError(#[from] std::num::TryFromIntError),
    #[error("Encountered an error trying to convert an infallible value: {0}")]
    FromRequestPartsError(#[from] std::convert::Infallible),

    #[error("Error while retrieving the book")]
    ChapterRetrievalError(#[from] content::ChapterError),
}

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        match self {
            Self::SQL(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response(),
            Self::Request(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response(),
            Self::TokenError(e) => {
                (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response()
            }
            Self::Unauthorized => {
                (StatusCode::UNAUTHORIZED, "Unauthorized!".to_string()).into_response()
            }
            Self::OptionError => (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Attempted to get a non-none value but found none".to_string(),
            )
                .into_response(),
            Self::ParseIntError(e) => {
                (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response()
            }
            Self::FromRequestPartsError(e) => {
                (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response()
            }

            Self::ChapterRetrievalError(_e) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                html! {
                    h1 { "Internal Server Error" }
                },
            )
                .into_response(),
        }
    }
}
