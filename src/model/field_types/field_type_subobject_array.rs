use crate::{
    errors::ValidationError,
    model::field_types::{
        check_reference_exists, check_reference_is_authorized, run_validators, ReturnHitError,
    },
    HitError,
};

use crate::model::validators::{ValidatorContext, Validators};
use crate::model::{Model, ModelField};
use crate::object_data::{ObjectValue, Reference};
use std::default::Default;

#[derive(Default)]
pub struct FieldTypeSubobjectArray {
    pub name: String,
    pub validators: Validators<Vec<Reference>>,
    pub authorized_models: Vec<String>,
}

fn validate_reference(
    sub_value: &Reference,
    context: &ValidatorContext,
    authorized_models: &Vec<String>,
) -> Result<(), HitError> {
    let entry = check_reference_exists(sub_value, context)?;
    if !check_reference_is_authorized(authorized_models, &entry.get_model()) {
        return Err(HitError::ModelNotAllowed(
            entry.get_model().get_name().into(),
        ));
    }
    Ok(())
}

impl ModelField for FieldTypeSubobjectArray {
    fn on_kernel_init(&mut self, model_name: &str) -> Result<(), HitError> {
        for validator in self.validators.iter_mut() {
            validator.on_kernel_init(&self.name, model_name)?;
        }
        Ok(())
    }
    fn accepts_for_set(&self, value: &ObjectValue, _context: &ValidatorContext) -> bool {
        match value {
            // ObjectValue::VecSubObjects(_) => true,
            _ => false,
        }
    }

    fn accepts_model(&self, model: &Model) -> bool {
        check_reference_is_authorized(&self.authorized_models, model)
    }

    fn get_name(&self) -> String {
        return String::from(&self.name);
    }
    fn validate(&self, value: &ObjectValue, context: &ValidatorContext) -> ReturnHitError {
        match value {
            ObjectValue::Null => Ok(None),
            ObjectValue::VecSubObjects(value) => {
                let mut errors: Vec<ValidationError> = vec![];
                //verify validity of reference
                for sub_value in value {
                    validate_reference(&sub_value, context, &self.authorized_models)?;
                }

                //No need to validate further if we have bad references

                if errors.len() > 0 {
                    return Ok(Some(errors));
                }
                //Run validators
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
        false
    }
    fn is_vec_subobject(&self) -> bool {
        true
    }
}
