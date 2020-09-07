use crate::hit::HitKernel;
use crate::model::Model;
use crate::model::ModelFieldRef;
use crate::object_data::Reference;
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

pub fn get_value_as_object(value: &Value) -> Result<&JSONObject, String> {
    match value {
        Value::Object(value) => Ok(value),
        _ => Err(String::from("Invalid type. Should be an object.")),
    }
}
pub fn get_value_as_array(value: &Value) -> Result<&JSONArray, String> {
    match value {
        Value::Array(value) => Ok(value),
        _ => Err(String::from("Invalid type. Should be an array.")),
    }
}
pub fn get_value_as_string(value: &Value) -> Result<String, String> {
    match value {
        Value::String(value) => Ok(String::from(value)),
        _ => Err(String::from("Invalid type. Should be a string.")),
    }
}
pub fn get_value_as_u32(value: &Value) -> Result<u32, String> {
    match value {
        Value::Number(value) => match value.as_u64() {
            Some(value) => Ok(value as u32),
            None => Err(String::from("Invalid number value")),
        },
        _ => Err(String::from("Invalid type. Should be a string.")),
    }
}

pub fn get_object_property(object: &JSONObject, property: String) -> Result<&Value, String> {
    match object.get(&property) {
        Some(value) => Ok(value),
        None => Err(format!("Property not found : {}", &property)),
    }
}
pub fn get_object_property_as_string(
    object: &JSONObject,
    property: String,
) -> Result<String, String> {
    let property = get_object_property(object, property)?;
    return get_value_as_string(property);
}
pub fn get_model(kernel: Rc<HitKernel>, data: &JSONObject) -> Result<Rc<Model>, String> {
    let model_name = get_object_property_as_string(data, String::from("model"))?;
    let model = kernel.get_model(&model_name);
    match model {
        Ok(model) => return Ok(model),
        Err(_error) => return Err(format!("Model Not Found: {} ", &model_name)),
    }
}

pub fn get_model_field(model: Rc<Model>, key: String) -> Result<ModelFieldRef, String> {
    let field = model.get_field(&key);
    match field {
        Some(field) => Ok(field.clone()),
        None => Err(format!("Field not found : {}", &key)),
    }
}

pub fn get_reference(sub_sub_value: &Value) -> Result<Reference, String> {
    let sub_sub_value = get_value_as_object(sub_sub_value)?;
    let id = get_object_property_as_string(sub_sub_value, String::from("id"))?;
    return Ok(Reference { id: id });
}

pub fn get_array_of_ids(sub_value: &Value) -> Result<Vec<Reference>, String> {
    let sub_value = get_value_as_array(sub_value)?;
    let mut array_of_ids = vec![];
    for sub_sub_value in sub_value.iter() {
        let reference = get_reference(sub_sub_value)?;
        array_of_ids.push(reference);
    }
    return Ok(array_of_ids);
}
