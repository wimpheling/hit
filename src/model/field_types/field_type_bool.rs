use crate::model::field_types::{check_if_required, run_validators};

use crate::errors::ModelError;
use crate::model::validators::{ValidatorContext, Validators};
use crate::model::{Model, ModelField};
use crate::object_data::ObjectValue;
use std::default::Default;

#[derive(Default)]
pub struct FieldTypeBool {
    pub required: bool,
    pub name: String,
    pub validators: Validators<bool>,
}

impl ModelField for FieldTypeBool {
    fn get_name(&self) -> String {
        return String::from(&self.name);
    }
    fn validate(
        &self,
        value: &ObjectValue,
        context: &ValidatorContext,
    ) -> Result<(), Vec<ModelError>> {
        match value {
            ObjectValue::Null => check_if_required(self.required),
            ObjectValue::Bool(value) => {
                let mut errors: Vec<ModelError> = vec![];
                run_validators(&self.validators, value, &mut errors, context);
                if errors.len() > 0 {
                    return Err(errors);
                }
                return Ok(());
            }
            _ => Err(vec![ModelError::InvalidDataType()]),
        }
    }

    fn accepts(&self, value: &ObjectValue, _context: &ValidatorContext) -> bool {
        match value {
            ObjectValue::Null => !self.required,
            ObjectValue::Bool(_) => true,
            _ => false,
        }
    }

    fn accepts_model(&self, _model: &Model) -> bool {
        return false;
    }
}
