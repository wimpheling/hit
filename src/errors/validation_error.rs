use std::collections::HashMap;

#[derive(Clone, Debug, PartialEq)]
pub enum ValidationErrorLevel {
    Warning,
    Error,
}

#[derive(Clone, Debug, PartialEq)]
pub struct ValidationError {
    pub key: String,
    pub level: ValidationErrorLevel,
    pub arguments: Option<HashMap<String, String>>,
}

pub static VALIDATION_ERROR_REQUIRED: &str = "REQUIRED";
