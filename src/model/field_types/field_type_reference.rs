use crate::model::{Model, ModelField};
use crate::object_data::{ObjectValue, Reference};
use crate::{
    errors::ValidationError,
    model::field_types::{
        check_if_required, check_reference_exists, run_validators, ReturnHitError,
    },
};
use crate::{
    model::validators::{ValidatorContext, Validators},
    HitError,
};

#[derive(Default)]
pub struct FieldTypeReference {
    pub required: bool,
    pub name: String,
    pub validators: Validators<Reference>,
    pub authorized_models: Vec<String>,
}

impl ModelField for FieldTypeReference {
    fn on_kernel_init(&mut self, model_name: &str) {
        for validator in self.validators.iter_mut() {
            validator.on_kernel_init(&self.name, model_name);
        }
    }
    fn accepts_for_set(&self, value: &ObjectValue, _context: &ValidatorContext) -> bool {
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
    fn validate(&self, value: &ObjectValue, context: &ValidatorContext) -> ReturnHitError {
        match value {
            ObjectValue::Null => check_if_required(self.required),
            ObjectValue::Reference(value) => {
                let mut errors: Vec<ValidationError> = vec![];
                //verify validity of reference
                let _entry = check_reference_exists(value, context)?;
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
        false
    }
}
