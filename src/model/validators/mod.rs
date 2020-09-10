use crate::Hit;
use crate::HitError;
use std::cell::RefCell;
use std::rc::Rc;
pub type Validators<T> = Vec<Rc<RefCell<dyn Validator<T>>>>;
pub trait Validator<T> {
    fn validate(&self, value: &T, context: &ValidatorContext) -> Result<(), Vec<HitError>>;
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

impl Validator<String> for MaxLength {
    fn validate(&self, value: &String, _context: &ValidatorContext) -> Result<(), Vec<HitError>> {
        if value.len() as u8 > self.length {
            // TODO : this should not be a HitError, but a validation error
            return Err(vec![HitError::DomainError {
                key: String::from(ERROR_MAX_LENGTH),
                message: ERROR_MAX_LENGTH.to_string(),
            }]);
        }
        return Ok(());
    }
}
