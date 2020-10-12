use crate::{
    errors::ValidationError,
    model::field_types::{check_reference_exists, run_validators, ReturnHitError},
    HitError,
};

use crate::model::validators::{ValidatorContext, Validators};
use crate::model::{Model, ModelField};
use crate::object_data::{ObjectValue, Reference};
use std::default::Default;

#[derive(Default)]
pub struct FieldTypeReferenceArray {
    pub name: String,
    pub validators: Validators<Reference>,
    pub authorized_models: Vec<String>,
}

impl ModelField for FieldTypeReferenceArray {
    fn accepts_for_set(&self, value: &ObjectValue, _context: &ValidatorContext) -> bool {
        match value {
            // ObjectValue::VecReference(_) => true,
            _ => false,
        }
    }

    fn accepts_model(&self, _model: &Model) -> bool {
        return false;
    }

    fn get_name(&self) -> String {
        return String::from(&self.name);
    }
    fn validate(&self, value: &ObjectValue, context: &ValidatorContext) -> ReturnHitError {
        match value {
            ObjectValue::Reference(value) => {
                let mut errors: Vec<ValidationError> = vec![];
                //verify validity of reference
                let _entry = check_reference_exists(value, context)?;
                run_validators(&self.validators, value, &mut errors, context)?;
                if errors.len() > 0 {
                    return Ok(Some(errors));
                }
                return Ok(None);
            }
            _ => Err(HitError::InvalidDataType()),
        }
    }
    fn is_vec_reference(&self) -> bool {
        true
    }
    fn is_vec_subobject(&self) -> bool {
        false
    }
}
