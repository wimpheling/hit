use crate::hit_mod::HitKernel;
use crate::json::JSONImportError;
use crate::model::Model;
use crate::object_data::Reference;
use crate::HitError;
use serde_json::{Map, Value};
use std::rc::Rc;

pub const JSON_FIELD_TYPE_REFERENCE: &str = "reference";
pub static JSON_FIELD_TYPE_REFERENCE_ARRAY: &str = "reference_array";
pub static JSON_FIELD_TYPE_SUBOBJECT: &str = "subobject";
pub static JSON_FIELD_TYPE_SUBOBJECT_ARRAY: &str = "subobject_array";
pub static JSON_FIELD_TYPE_STRING_ARRAY: &str = "string_array";
pub static JSON_FIELD_TYPE_DATE: &str = "date";

pub type JSONObject = Map<String, Value>;
pub type JSONArray = Vec<Value>;

pub fn get_value_as_object(value: &Value) -> Result<&JSONObject, JSONImportError> {
    match value {
        Value::Object(value) => Ok(value),
        _ => Err(JSONImportError::InvalidTypeShouldBeAnObject()),
    }
}
pub fn get_value_as_array(value: &Value) -> Result<&JSONArray, JSONImportError> {
    match value {
        Value::Array(value) => Ok(value),
        _ => Err(JSONImportError::InvalidTypeShouldBeAnArray()),
    }
}
pub fn get_value_as_string(value: &Value) -> Result<String, JSONImportError> {
    match value {
        Value::String(value) => Ok(String::from(value)),
        _ => Err(JSONImportError::InvalidTypeShouldBeAString()),
    }
}
/*
pub fn get_value_as_u32(value: &Value) -> Result<u32, JSONImportError> {
    match value {
        Value::Number(value) => match value.as_u64() {
            Some(value) => Ok(value as u32),
            None => Err(JSONImportError::InvalidTypeShouldBeANumber()),
        },
        _ => Err(JSONImportError::InvalidTypeShouldBeAString()),
    }
} */

pub fn get_object_property(
    object: &JSONObject,
    property: String,
) -> Result<&Value, JSONImportError> {
    match object.get(&property) {
        Some(value) => Ok(value),
        None => Err(JSONImportError::HitError(HitError::PropertyNotFound(
            property.to_string(),
        ))),
    }
}
pub fn get_object_property_as_string(
    object: &JSONObject,
    property: String,
) -> Result<String, JSONImportError> {
    let property = get_object_property(object, property)?;
    return get_value_as_string(property);
}

pub fn get_model(kernel: Rc<HitKernel>, data: &JSONObject) -> Result<Rc<Model>, JSONImportError> {
    let model_name = get_object_property_as_string(data, String::from("model"))?;
    let model = kernel.get_model(&model_name);
    match model {
        Ok(model) => return Ok(model),
        Err(error) => return Err(JSONImportError::HitError(error)),
    }
}

pub fn get_reference(sub_sub_value: &Value) -> Result<Reference, JSONImportError> {
    let sub_sub_value = get_value_as_object(sub_sub_value)?;
    let id = get_object_property_as_string(sub_sub_value, String::from("id"))?;
    return Ok(Reference { id: id });
}

pub fn get_array_of_ids(sub_value: &Value) -> Result<Vec<Reference>, JSONImportError> {
    let sub_value = get_value_as_array(sub_value)?;
    let mut array_of_ids = vec![];
    for sub_sub_value in sub_value.iter() {
        let reference = get_reference(sub_sub_value)?;
        array_of_ids.push(reference);
    }
    return Ok(array_of_ids);
}
