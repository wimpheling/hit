use crate::errors::ModelError;
use crate::model::field_types::{
    check_if_required, check_reference_exists, check_reference_is_authorized, run_validators,
    ReturnModelError,
};
use crate::model::validators::{ValidatorContext, Validators};
use crate::model::{Model, ModelField};
use crate::object_data::{ObjectValue, Reference};

pub struct FieldTypeReference {
    pub required: bool,
    pub name: String,
    pub validators: Validators<Reference>,
    pub authorized_models: Vec<String>,
}

impl ModelField for FieldTypeReference {
    fn accepts(&self, value: &ObjectValue, _context: &ValidatorContext) -> bool {
        match value {
            ObjectValue::Null => !self.required,
            ObjectValue::Reference(_) => true,
            _ => false,
        }
    }

    fn accepts_model(&self, _model: &Model) -> bool {
        return false;
    }

    fn get_name(&self) -> String {
        return String::from(&self.name);
    }
    fn validate(&self, value: &ObjectValue, context: &ValidatorContext) -> ReturnModelError {
        match value {
            ObjectValue::Null => check_if_required(self.required),
            ObjectValue::Reference(value) => {
                let mut errors: Vec<ModelError> = vec![];
                //verify validity of reference
                let entry = check_reference_exists(value, context)?;
                check_reference_is_authorized(&self.authorized_models, &entry.get_model())?;
                //Run validators
                run_validators(&self.validators, value, &mut errors, context);

                if errors.len() > 0 {
                    return Err(errors);
                }
                return Ok(());
            }
            _ => Err(vec![ModelError::InvalidDataType()]),
        }
    }
}
