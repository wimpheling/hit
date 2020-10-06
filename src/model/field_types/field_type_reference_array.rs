use crate::{
    errors::ValidationError,
    model::field_types::{
        check_reference_exists, check_reference_is_authorized, run_validators, ReturnHitError,
    },
};

use crate::model::validators::{ValidatorContext, Validators};
use crate::model::{Model, ModelField};
use crate::object_data::{ObjectValue, Reference};
use anyhow::Error;
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
                let mut errors: Vec<Error> = vec![];
                //verify validity of reference
                let entry = check_reference_exists(value, context)?;
                if !check_reference_is_authorized(&self.authorized_models, &entry.get_model()) {
                    return Err(vec![anyhow::anyhow!(ValidationError::ModelNotAllowed())]);
                } //Run validators
                run_validators(&self.validators, value, &mut errors, context);

                if errors.len() > 0 {
                    return Err(errors);
                }
                return Ok(());
            }
            _ => Err(vec![anyhow::anyhow!(ValidationError::InvalidDataType())]),
        }
    }
    fn is_vec_reference(&self) -> bool {
        true
    }
    fn is_vec_subobject(&self) -> bool {
        false
    }
}
