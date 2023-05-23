#[allow(dead_code)]

use axum::http::header::WWW_AUTHENTICATE;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::response::Response;
use axum::Json;
use std::borrow::Cow;
use std::collections::HashMap;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    /// Return `401 Unauthorized`
    #[error("authentication required")]
    Unauthorized,

    /// Return `403 Forbidden`
    #[error("user may not perform that action")]
    Forbidden,

    /// Return `404 Not Found`
    #[error("request path not found")]
    NotFound,
    #[error("error in the request body")]
    UnprocessableEntity {
        errors: HashMap<Cow<'static, str>, Vec<Cow<'static, str>>>,
    },
    #[error("database error")]
    Db,
    #[error("an internal server error occurred")]
    Anyhow(#[from] anyhow::Error),
}

impl Error {
    /// Convenient constructor for `Error::UnprocessableEntity`.
    ///
    /// Multiple for the same key are collected into a list for that key.
    ///
    /// Try "Go to Usage" in an IDE for examples.
    pub fn unprocessable_entity<K, V>(errors: impl IntoIterator<Item = (K, V)>) -> Self
    where
        K: Into<Cow<'static, str>>,
        V: Into<Cow<'static, str>>,
    {
        let mut error_map = HashMap::new();

        for (key, val) in errors {
            error_map
                .entry(key.into())
                .or_insert_with(Vec::new)
                .push(val.into());
        }

        Self::UnprocessableEntity { errors: error_map }
    }

    fn status_code(&self) -> StatusCode {
        match self {
            Self::Unauthorized => StatusCode::UNAUTHORIZED,
            Self::Forbidden => StatusCode::FORBIDDEN,
            Self::NotFound => StatusCode::NOT_FOUND,
            Self::UnprocessableEntity { .. } => StatusCode::UNPROCESSABLE_ENTITY,
            Self::Db | Self::Anyhow(_) => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}

impl IntoResponse for Error {
    // fn into_response(self) -> Response {
    //     (StatusCode::INTERNAL_SERVER_ERROR, Json(self.to_string())).into_response()
    // }
    fn into_response(self) -> Response {
        match self {
            Self::UnprocessableEntity { errors } => {
                #[derive(serde::Serialize)]
                struct Errors {
                    errors: HashMap<Cow<'static, str>, Vec<Cow<'static, str>>>,
                }

                return (StatusCode::UNPROCESSABLE_ENTITY, Json(Errors { errors })).into_response();
            }
            Self::Unauthorized => {
                return (
                    self.status_code(),
                    // Include the `WWW-Authenticate` challenge required in the specification
                    // for the `401 Unauthorized` response code:
                    // https://developer.mozilla.org/en-US/docs/Web/HTTP/Status/401
                    //
                    // The Realworld spec does not specify this:
                    // https://realworld-docs.netlify.app/docs/specs/backend-specs/error-handling
                    //
                    // However, at Launchbadge we try to adhere to web standards wherever possible,
                    // if nothing else than to try to act as a vanguard of sanity on the web.
                    [(WWW_AUTHENTICATE, "Token")],
                    self.to_string(),
                )
                    .into_response();
            }

            // Self::Sqlx(ref e) => {
            //     // TODO: we probably want to use `tracing` instead
            //     // so that this gets linked to the HTTP request by `TraceLayer`.
            //     log::error!("SQLx error: {:?}", e);
            // }

            Self::Anyhow(ref e) => {
                // TODO: we probably want to use `tracing` instead
                // so that this gets linked to the HTTP request by `TraceLayer`.
                log::error!("Generic error: {:?}", e);
            }

            // Other errors get mapped normally.
            _ => (),
        }

        (self.status_code(), self.to_string()).into_response()
    }
}

impl From<surrealdb::Error> for Error {
    fn from(error: surrealdb::Error) -> Self {
        eprintln!("{error}");
        Self::Db
    }
}
