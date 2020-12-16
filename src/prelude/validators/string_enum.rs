use crate::model::validators::{Validator, ValidatorContext};
use crate::{HitError, ValidationError};

pub struct StringEnumValidator {
    values: Vec<String>,
}

impl Validator<String> for StringEnumValidator {
    fn validate(
        &self,
        value: &String,
        _context: &ValidatorContext,
    ) -> Result<Option<Vec<ValidationError>>, HitError> {
        for enum_value in self.values.iter() {
            if enum_value == value {
                return Ok(None);
            }
        }
        return Ok(Some(vec![ValidationError::warning("A".into(), None)]));
    }

    fn on_kernel_init(&mut self, _field_name: &str, _model_namee: &str) -> Result<(), HitError> {
        Ok(())
    }
}
