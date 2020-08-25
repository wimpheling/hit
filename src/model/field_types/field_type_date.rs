use crate::errors::ModelError;
use crate::model::field_types::{check_if_required, run_validators};
use crate::model::validators::{ValidatorContext, Validators};
use crate::model::{Model, ModelField};
use crate::object_data::ObjectValue;
use chrono::{DateTime, Utc};

pub struct FieldTypeDate {
    pub required: bool,
    pub name: String,
    pub validators: Validators<DateTime<Utc>>,
}

impl ModelField for FieldTypeDate {
    fn get_name(&self) -> String {
        return String::from(&self.name);
    }

    fn accepts(&self, value: &ObjectValue, _context: &ValidatorContext) -> bool {
        match value {
            ObjectValue::Null => !self.required,
            ObjectValue::Date(_) => true,
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
    ) -> Result<(), Vec<ModelError>> {
        match value {
            ObjectValue::Null => check_if_required(self.required),
            ObjectValue::Date(value) => {
                let mut errors: Vec<ModelError> = vec![];
                let date = value.get_date();
                run_validators(&self.validators, &date, &mut errors, context);

                if errors.len() > 0 {
                    return Err(errors);
                }
                return Ok(());
            }
            _ => Err(vec![ModelError::InvalidDataType()]),
        }
    }
}
