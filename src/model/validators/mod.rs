use crate::Hit;
use crate::HitError;
use anyhow::Error;
use std::cell::RefCell;
use std::rc::Rc;

pub type Validators<T> = Vec<Rc<RefCell<dyn Validator<T>>>>;
pub trait Validator<T> {
    fn validate(&self, value: &T, context: &ValidatorContext) -> Result<(), Vec<Error>>;
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

#[derive(thiserror::Error, Clone, Debug, PartialEq)]
#[error("MAX_LENGTH")]
pub struct MaxLengthError {}

impl Validator<String> for MaxLength {
    fn validate(&self, value: &String, _context: &ValidatorContext) -> Result<(), Vec<Error>> {
        if value.len() as u8 > self.length {
            // TODO : this should not be a HitError, but a validation error
            return Err(vec![anyhow::anyhow!(MaxLengthError {})]);
        }
        return Ok(());
    }
}
