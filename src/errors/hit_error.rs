use serde::{Deserialize, Serialize};
use std::clone::Clone;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum HitError {
    ModelDoesNotExist(String),
    InvalidDataType(),
    InvalidDateFormat(),
    InvalidReference(),
    InvalidReferenceInArray(),
    InvalidReferenceType(),
    InvalidReferenceTypeInArray(),
    InvalidEnumValue(String),
    Required(),
    DomainError {
        key: String,
        message: Option<String>,
    },
}
