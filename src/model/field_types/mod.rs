mod field_type_bool;
mod field_type_date;
mod field_type_float;
mod field_type_integer;
mod field_type_reference;
mod field_type_reference_array;
mod field_type_string;
mod field_type_string_vec;
mod field_type_subobject;
mod field_type_subobject_array;

use crate::model::validators::{ValidatorContext, Validators};
use crate::model::Model;
use crate::object_data::Reference;
use crate::HitError;
use crate::{errors::ValidationError, hit_mod::HitEntry};
use anyhow::Error;
pub use field_type_bool::FieldTypeBool;
pub use field_type_date::FieldTypeDate;
pub use field_type_float::FieldTypeFloat;
pub use field_type_integer::FieldTypeInteger;
pub use field_type_reference::FieldTypeReference;
pub use field_type_reference_array::FieldTypeReferenceArray;
pub use field_type_string::FieldTypeString;
pub use field_type_string_vec::FieldTypeStringVec;
pub use field_type_subobject::FieldTypeSubobject;
pub use field_type_subobject_array::FieldTypeSubobjectArray;

fn check_if_required(required: bool) -> Result<(), Vec<Error>> {
    if required == true {
        return Err(vec![anyhow::anyhow!(ValidationError::Required())]);
    }
    return Ok(());
}

type ReturnHitError = Result<(), Vec<Error>>;

fn check_reference_exists<'a>(
    value: &Reference,
    context: &'a ValidatorContext<'a>,
) -> Result<HitEntry, Vec<Error>> {
    //check reference
    let entry = context.index.get(&value.id);
    match entry {
        None => {
            return Err(vec![anyhow::anyhow!(
                ValidationError::ReferenceDoesNotExist()
            )]);
        }
        Some(entry) => Ok(entry),
    }
}

fn check_reference_is_authorized(authorized_models: &Vec<String>, model: &Model) -> bool {
    for authorized_model in authorized_models {
        if model.get_name() == authorized_model {
            return true;
        }
        if model.implements_interface(authorized_model) {
            return true;
        }
    }
    return false;
}

pub fn run_validators<T>(
    validators: &Validators<T>,
    value: &T,
    all_errors: &mut Vec<Error>,
    context: &ValidatorContext,
) -> () {
    for validator in validators.iter() {
        let errors = validator.borrow().validate(value, context);
        match errors {
            Err(errors) => all_errors.extend(errors),
            Ok(_arg) => {}
        }
    }
}
