use crate::hit_mod::{Hit, HitKernel, IndexModelImporter};
use crate::index::IndexEntryProperty;
use crate::json::utils::*;
use crate::json::JSONImportError;
use crate::object_data::{DateTimeUtc, ObjectValue, ObjectValues};
use chrono::{DateTime, Utc};
use linked_hash_map::LinkedHashMap;
use serde_json::Value;

use std::rc::Rc;

use crate::import::generic_import::{finish_import, import_data_object_values};

fn json_to_object_value(value: &Value) -> Result<ObjectValue, JSONImportError> {
    match value {
        Value::Null => Ok(ObjectValue::Null),
        Value::Number(value) => match value.as_f64() {
            Some(value) => Ok(ObjectValue::F32(value as f32)),
            None => Err(JSONImportError::InvalidTypeShouldBeANumber()),
        },
        Value::String(value) => Ok(ObjectValue::String(String::from(value))),
        Value::Bool(value) => Ok(ObjectValue::Bool(*value)),
        Value::Object(value) => {
            let _type = get_object_property_as_string(value, String::from("type"))?;
            let sub_value = get_object_property(value, String::from("value"))?;
            if _type == String::from(JSON_FIELD_TYPE_REFERENCE) {
                let reference = get_reference(sub_value)?;
                return Ok(ObjectValue::Reference(reference));
            } else if _type == String::from(JSON_FIELD_TYPE_REFERENCE_ARRAY) {
                let array_of_ids = get_array_of_ids(sub_value)?;
                return Ok(ObjectValue::VecReference(array_of_ids));
            } else if _type == String::from(JSON_FIELD_TYPE_SUBOBJECT) {
                let reference = get_reference(sub_value)?;
                return Ok(ObjectValue::SubObject(reference));
            } else if _type == String::from(JSON_FIELD_TYPE_SUBOBJECT_ARRAY) {
                let array_of_ids = get_array_of_ids(sub_value)?;
                return Ok(ObjectValue::VecSubObjects(array_of_ids));
            } else if _type == String::from(JSON_FIELD_TYPE_DATE) {
                let sub_value = get_value_as_string(sub_value)?;
                let sub_value = DateTime::parse_from_rfc2822(&sub_value);
                match sub_value {
                    Ok(sub_value) => Ok(ObjectValue::Date(DateTimeUtc::new(
                        sub_value.with_timezone(&Utc),
                    ))),
                    Err(_error) => Err(JSONImportError::InvalidDateFormat()),
                }
            } else {
                return Err(JSONImportError::InvalidSubObjectType());
            }
        }
        Value::Array(_value) => Err(JSONImportError::ShouldNotBeAnArray()),
    }
}

fn get_parent(value: &JSONObject) -> Result<Option<IndexEntryProperty>, JSONImportError> {
    let parent = get_object_property(value, String::from("parent"))?;
    match parent {
        Value::Null => {
            return Ok(None);
        }
        Value::Object(parent) => {
            let id = get_object_property_as_string(&parent, String::from("id"))?;
            let property = get_object_property_as_string(&parent, String::from("property"))?;
            return Ok(Some(IndexEntryProperty {
                id: id,
                property: property,
            }));
        }
        _ => return Err(JSONImportError::InvalidTypeShouldBeAnObject()),
    }
}

fn import_data<'index>(
    data: &JSONObject,
    kernel: Rc<HitKernel>,
    new_index: &'index mut IndexModelImporter,
) -> Result<(), JSONImportError> {
    let id = get_object_property_as_string(data, String::from("id"))?;
    let model = get_model(kernel, data)?;
    let parent = get_parent(data)?;
    let sub_data = get_object_property(data, String::from("data"))?;
    let sub_data = get_value_as_object(sub_data)?;
    let mut new_data: ObjectValues = LinkedHashMap::new();
    for d in sub_data.iter() {
        new_data.insert(String::from(d.0), json_to_object_value(d.1)?);
    }

    import_data_object_values(model, id, parent, new_index, new_data)
        .map_err(JSONImportError::HitError)?;

    return Ok(());
}

fn get_json_value(value: &String) -> Result<Value, JSONImportError> {
    match serde_json::from_str(value) {
        Err(_err) => Err(JSONImportError::InvalidJSON()),
        Ok(value) => Ok(value),
    }
}
pub fn import_from_string<'a>(
    value: &String,
    kernel: Rc<HitKernel>,
) -> Result<Hit, JSONImportError> {
    let value = get_json_value(value)?;
    import(&value, kernel)
}

pub fn import<'a>(value: &Value, kernel: Rc<HitKernel>) -> Result<Hit, JSONImportError> {
    let value = get_value_as_object(&value)?;
    let id = get_object_property_as_string(value, String::from("id"))?;
    let mut new_index = IndexModelImporter::new(&id, kernel.clone());
    let data = get_object_property(value, String::from("data"))?;
    let data = get_value_as_array(data)?;
    for entry in data.iter() {
        let clone = kernel.clone();
        let entry = get_value_as_object(entry)?;
        import_data(entry, clone, &mut new_index)?;
    }
    let new_index = finish_import(new_index, kernel).map_err(JSONImportError::HitError)?;
    Ok(new_index)
}
