#[derive(thiserror::Error, Clone, Debug, PartialEq)]
pub enum ValidationError {
    #[error("FIELD_REQUIRED")]
    Required(),
    #[error("INVALID_DATA_TYPE")]
    InvalidDataType(),
    #[error("MODEL_NOT_ALLOWED")]
    ModelNotAllowed(),
    #[error("REFERENCE_DOES_NOT_EXIST")]
    ReferenceDoesNotExist(),
}
