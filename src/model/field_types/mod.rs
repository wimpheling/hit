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

use crate::hit::HitEntry;
use crate::model::validators::{ValidatorContext, Validators};
use crate::model::Model;
use crate::object_data::Reference;
use crate::HitError;
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

fn check_if_required(required: bool) -> Result<(), Vec<HitError>> {
    if required == true {
        return Err(vec![HitError::Required()]);
    }
    return Ok(());
}

type ReturnHitError = Result<(), Vec<HitError>>;

fn check_reference_exists<'a>(
    value: &Reference,
    context: &'a ValidatorContext<'a>,
) -> Result<HitEntry, Vec<HitError>> {
    //check reference
    let entry = context.index.get(&value.id);
    match entry {
        None => {
            return Err(vec![HitError::InvalidReference()]);
        }
        Some(entry) => Ok(entry),
    }
}

fn check_reference_is_authorized(authorized_models: &Vec<String>, model: &Model) -> ReturnHitError {
    for authorized_model in authorized_models {
        if model.get_name() == authorized_model {
            return Ok(());
        }
        if model.implements_interface(authorized_model) {
            return Ok(());
        }
    }
    return Err(vec![HitError::InvalidReference()]);
}
pub fn run_validators<T>(
    validators: &Validators<T>,
    value: &T,
    all_errors: &mut Vec<HitError>,
    context: &ValidatorContext,
) -> () {
    for validator in validators.iter() {
        let errors = validator.borrow().validate(value, context);
        match errors {
            Err(errors) => all_errors.extend_from_slice(&errors),
            Ok(_arg) => {}
        }
    }
}
