use crate::{errors::ValidationError, Hit};
use crate::{errors::ValidationErrorLevel, HitError};
use std::cell::RefCell;
use std::rc::Rc;

pub type Validators<T> = Vec<Rc<RefCell<dyn Validator<T>>>>;
pub trait Validator<T> {
    fn validate(
        &self,
        value: &T,
        context: &ValidatorContext,
    ) -> Result<Option<Vec<ValidationError>>, HitError>;
}

pub struct MaxLength {
    pub length: u8,
}
pub static ERROR_MAX_LENGTH: &str = "Max length was reached.";

pub struct ValidatorContext<'a> {
    pub id: &'a str,
    pub property: &'a str,
    pub index: Rc<&'a Hit>,
}

static MAX_LENGTH: &str = "MAX_LENGTH";

impl Validator<String> for MaxLength {
    fn validate(
        &self,
        value: &String,
        _context: &ValidatorContext,
    ) -> Result<Option<Vec<ValidationError>>, HitError> {
        if value.len() as u8 > self.length {
            // TODO : this should not be a HitError, but a validation error
            return Ok(Some(vec![ValidationError {
                key: MAX_LENGTH.to_string(),
                level: ValidationErrorLevel::Error,
                arguments: None,
            }]));
        }
        return Ok(None);
    }
}
