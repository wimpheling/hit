use std::{cell::RefCell, clone::Clone, rc::Rc};

use crate::{
    model::validators::{Validator, ValidatorContext},
    ObjectValue, Reference,
};
use crate::{HitError, ValidationError};

use super::unique_in_parent_plugin::UniqueInParentIndex;
static UNIQUE_IN_PARENT: &str = "UNIQUE_IN_PARENT";

pub struct UniqueInParentValidator {
    property_name: String,
    index: Rc<RefCell<UniqueInParentIndex>>,
}

impl UniqueInParentValidator {
    fn get_parent_property_value(&self, context: &ValidatorContext) -> Option<Vec<Reference>> {
        let index = context.index.clone();
        let parent = index.get_parent(context.id)?;
        let val = index.get_value(&parent.id, context.property)?;
        match val {
            crate::ObjectValue::VecSubObjects(val) => Some(val),
            _ => None,
        }
    }
}

impl Validator<String> for UniqueInParentValidator {
    fn validate(
        &self,
        value: &String,
        context: &ValidatorContext,
    ) -> Result<Option<Vec<ValidationError>>, HitError> {
        // TODO : remove unwrap
        let parent_sub_items = self.get_parent_property_value(context).unwrap();
        for sub_item in parent_sub_items.iter() {
            if sub_item.id == context.id {
                continue;
            }
            // TODO : remove unwrap
            let index = context.index.clone();
            match index
                .get_value(&sub_item.id, &self.property_name)
                .unwrap_or(ObjectValue::Null)
            {
                ObjectValue::String(val) => {
                    if &val == value {
                        return Ok(Some(vec![ValidationError::warning(
                            UNIQUE_IN_PARENT.into(),
                            None,
                        )]));
                    }
                }
                _ => {}
            }
        }
        Ok(None)
    }

    fn on_kernel_init(&mut self, field_name: &str, model_name: &str) -> Result<(), HitError> {
        self.index
            .borrow_mut()
            .property_names
            .get_or_insert(field_name.to_string());
        self.index
            .borrow_mut()
            .model_names
            .get_or_insert(model_name.to_string());
        Ok(())
    }
}
