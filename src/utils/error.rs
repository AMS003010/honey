use actix_web::{HttpResponse, ResponseError};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("database error")]
    Db,
}

impl ResponseError for Error {
    fn error_response(&self) -> HttpResponse {
        match self {
            Error::Db => HttpResponse::InternalServerError().body(self.to_string()),
        }
    }
}

impl From<surrealdb::Error> for Error {
    fn from(err: surrealdb::Error) -> Self {
        eprintln!("{err}");
        Self::Db
    }
}