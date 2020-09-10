use serde::{Deserialize, Serialize};
use std::clone::Clone;
use thiserror::Error;

#[derive(Error, Clone, Debug)]
pub enum HitError {
    #[error("Model `{0}` does not exist")]
    ModelDoesNotExist(String),
    #[error("Invalid data type")]
    InvalidDataType(),
    #[error("Invalid date format")]
    InvalidDateFormat(),
    #[error("Invalid reference")]
    InvalidReference(),
    // Todo : Is it useful for it to be different than invalid reference ?
    #[error("Invalid Reference in array")]
    InvalidReferenceInArray(),
    //TODO : should it be mandatory ?
    #[error("Invalid reference type")]
    InvalidReferenceType(),
    #[error("Invalid reference type in an array")]
    InvalidReferenceTypeInArray(),
    #[error("This field is required")]
    Required(),
    #[error("`{key}` error: `{message}` ")]
    DomainError { key: String, message: String },
}
