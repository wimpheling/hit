use crate::model::field_types::{check_if_required, run_validators};

use crate::model::validators::{ValidatorContext, Validators};
use crate::model::{Model, ModelField};
use crate::object_data::ObjectValue;
use crate::HitError;
use std::default::Default;

#[derive(Default)]
pub struct FieldTypeStringVec {
    pub required: bool,
    pub name: String,
    pub validators: Validators<String>,
    pub _enum: Option<Vec<String>>,
}

impl ModelField for FieldTypeStringVec {
    fn get_name(&self) -> String {
        return String::from(&self.name);
    }
    fn accepts_for_set(&self, value: &ObjectValue, _context: &ValidatorContext) -> bool {
        match value {
            ObjectValue::String(_) => true,
            _ => false,
        }
    }

    fn accepts_model(&self, _model: &Model) -> bool {
        return false;
    }

    fn validate(
        &self,
        value: &ObjectValue,
        context: &ValidatorContext,
    ) -> Result<(), Vec<HitError>> {
        match value {
            ObjectValue::Null => check_if_required(self.required),
            ObjectValue::String(value) => {
                let mut errors: Vec<HitError> = vec![];
                run_validators(&self.validators, value, &mut errors, context);

                if errors.len() > 0 {
                    return Err(errors);
                }
                return Ok(());
            }
            _ => Err(vec![HitError::InvalidDataType()]),
        }
    }
    fn is_vec_reference(&self) -> bool {
        false
    }
    fn is_vec_subobject(&self) -> bool {
        false
    }
}
