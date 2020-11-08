use serde::Serialize;
use std::collections::HashMap;

#[derive(Clone, Debug, Serialize, PartialEq)]
pub enum ValidationErrorLevel {
    Warning,
    Error,
}

#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct ValidationError {
    pub key: String,
    pub level: ValidationErrorLevel,
    pub arguments: Option<HashMap<String, String>>,
}

impl ValidationError {
    pub fn warning(key: String, arguments: Option<HashMap<String, String>>) -> ValidationError {
        return ValidationError {
            key: key,
            level: ValidationErrorLevel::Warning,
            arguments: arguments,
        };
    }
    pub fn error(key: String, arguments: Option<HashMap<String, String>>) -> ValidationError {
        return ValidationError {
            key: key,
            level: ValidationErrorLevel::Error,
            arguments: arguments,
        };
    }
}

pub static VALIDATION_ERROR_REQUIRED: &str = "REQUIRED";
