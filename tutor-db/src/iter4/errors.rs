use actix_web::{error, http::StatusCode, HttpResponse, Result};
use serde::Serialize;
use sqlx::error::Error as SQLXError;
use std::fmt;

#[derive(Debug, Serialize)]
pub enum EzyTutorError {
    DBError(String),
    ActixError(String),
    NotFound(String),
}
#[derive(Debug, Serialize)]
pub struct MyErrorResponse {
    error_message: String,
}