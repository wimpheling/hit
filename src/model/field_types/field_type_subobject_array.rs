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
pub struct FieldTypeSubobjectArray {
    pub name: String,
    pub validators: Validators<Vec<Reference>>,
    pub authorized_models: Vec<String>,
}

fn validate_reference(
    sub_value: &Reference,
    context: &ValidatorContext,
    authorized_models: &Vec<String>,
) -> Result<(), Vec<Error>> {
    let entry = check_reference_exists(sub_value, context)?;
    if !check_reference_is_authorized(authorized_models, &entry.get_model()) {
        return Err(vec![anyhow::anyhow!(ValidationError::ModelNotAllowed())]);
    }
    Ok(())
}

impl ModelField for FieldTypeSubobjectArray {
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
            ObjectValue::Null => Ok(()),
            ObjectValue::VecSubObjects(value) => {
                let mut errors: Vec<Error> = vec![];
                //verify validity of reference
                for sub_value in value {
                    match validate_reference(&sub_value, context, &self.authorized_models) {
                        Err(errs) => errors.extend(errs),
                        Ok(_r) => {}
                    }
                }

                //No need to validate further if we have bad references
                if errors.len() > 0 {
                    return Err(errors);
                }
                //Run validators
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
        false
    }
    fn is_vec_subobject(&self) -> bool {
        true
    }
}
