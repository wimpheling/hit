use crate::model::validators::{ValidatorContext, Validators};
use crate::model::{Model, ModelField};
use crate::object_data::ObjectValue;
use crate::HitError;
use crate::{
    errors::ValidationError,
    model::field_types::{check_if_required, run_validators},
};
use anyhow::Error;

pub struct FieldTypeFloat {
    pub required: bool,
    pub name: String,
    pub validators: Validators<f32>,
}

impl ModelField for FieldTypeFloat {
    fn get_name(&self) -> String {
        return String::from(&self.name);
    }

    fn accepts_for_set(&self, value: &ObjectValue, _context: &ValidatorContext) -> bool {
        match value {
            ObjectValue::Null => !self.required,
            ObjectValue::F32(_) => true,
            _ => false,
        }
    }

    fn accepts_model(&self, _model: &Model) -> bool {
        return false;
    }

    fn validate(&self, value: &ObjectValue, context: &ValidatorContext) -> Result<(), Vec<Error>> {
        match value {
            ObjectValue::Null => check_if_required(self.required),
            ObjectValue::F32(value) => {
                let mut errors: Vec<Error> = vec![];
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
        false
    }
}
