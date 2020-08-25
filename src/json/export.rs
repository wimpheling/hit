use crate::index::IndexEntryProperty;
use crate::json::utils::*;
use crate::model::{IndexedModel, IndexedModelEntry};
use crate::object_data::{ObjectValue, Reference};
use serde_json::{json, Map, Value};
use std::collections::HashMap;

fn reference_to_data(reference: &Reference) -> JSONObject {
    let mut ref_object = Map::new();
    ref_object.insert(
        String::from("id"),
        Value::String(String::from(&reference.id)),
    );
    return ref_object;
}
fn reference_to_json_object(reference: &Reference, _type: String) -> JSONObject {
    let mut new_value = Map::new();
    new_value.insert(String::from("type"), Value::String(_type));
    let ref_object = reference_to_data(reference);
    new_value.insert(String::from("value"), Value::Object(ref_object));
    return new_value;
}
fn vecreference_to_json_object(references: &Vec<Reference>, _type: String) -> JSONObject {
    let mut new_value = Map::new();
    new_value.insert(String::from("type"), Value::String(_type));
    let mut array_of_refs: Vec<Value> = vec![];
    for reference in references {
        array_of_refs.push(Value::Object(reference_to_data(reference)));
    }
    new_value.insert(String::from("value"), Value::Array(array_of_refs));
    return new_value;
}
fn vecstring_to_json_object(strings: &Vec<String>) -> JSONObject {
    let mut new_value = Map::new();
    new_value.insert(
        String::from("type"),
        Value::String(JSON_FIELD_TYPE_STRING_ARRAY.to_string()),
    );
    let mut array_of_strings: Vec<Value> = vec![];
    for string in strings {
        array_of_strings.push(Value::String(string.to_string()));
    }
    new_value.insert(String::from("value"), Value::Array(array_of_strings));
    return new_value;
}

fn object_value_to_json(value: &ObjectValue) -> Result<Value, String> {
    match value {
        ObjectValue::Bool(value) => Ok(Value::Bool(*value)),
        ObjectValue::Date(value) => {
            let mut new_value = Map::new();
            new_value.insert(
                String::from("type"),
                Value::String(String::from(JSON_FIELD_TYPE_DATE)),
            );
            new_value.insert(
                String::from("value"),
                Value::String(value.get_date_as_string()),
            );
            return Ok(Value::Object(new_value));
        }
        ObjectValue::F32(value) => {
            let number = serde_json::Number::from_f64(*value as f64)
                .ok_or(String::from("Invalid number"))?;
            return Ok(Value::Number(number));
        }
        ObjectValue::I32(value) => {
            let number = serde_json::Number::from_f64(*value as f64)
                .ok_or(String::from("Invalid number"))?;
            return Ok(Value::Number(number));
        }
        ObjectValue::Null => Ok(Value::Null),
        ObjectValue::String(value) => Ok(Value::String(String::from(value))),
        ObjectValue::Reference(value) => {
            return Ok(Value::Object(reference_to_json_object(
                value,
                String::from(JSON_FIELD_TYPE_REFERENCE),
            )));
        }
        ObjectValue::SubObject(value) => {
            return Ok(Value::Object(reference_to_json_object(
                value,
                String::from(JSON_FIELD_TYPE_SUBOBJECT),
            )));
        }
        ObjectValue::VecString(value) => {
            return Ok(Value::Object(vecstring_to_json_object(value)));
        }
        ObjectValue::VecReference(value) => {
            return Ok(Value::Object(vecreference_to_json_object(
                value,
                String::from(JSON_FIELD_TYPE_REFERENCE_ARRAY),
            )));
        }
        ObjectValue::VecSubObjects(value) => {
            return Ok(Value::Object(vecreference_to_json_object(
                value,
                String::from(JSON_FIELD_TYPE_SUBOBJECT_ARRAY),
            )));
        }
    }
}
fn export_parent(parent: Option<IndexEntryProperty>) -> Value {
    match parent {
        None => Value::Null,
        Some(parent) => json!({
            "id": parent.id,
            "property": parent.property
        }),
    }
}

fn export_object(object: IndexedModelEntry) -> Result<Value, String> {
    let model = &object.get_model();
    let mut data = HashMap::new();
    for (key, _field) in model.fields.iter() {
        let entry = object.entry.borrow();
        let entry = entry.get(key);
        match entry {
            ObjectValue::Null => {}
            _ => {
                let json_entry = object_value_to_json(entry)?;
                data.insert(key, json_entry);
            }
        };
    }
    return Ok(json!({
        "model": model.get_name(),
        "id": object.entry.borrow().get_id(),
        "data": data,
        "parent": export_parent(object.entry.borrow().get_parent()),
    }));
}

pub fn export(index: &IndexedModel) -> Result<Value, String> {
    let mut data = vec![];
    for (id, entry) in index.index.iter() {
        let model = { index.get_model(id).ok_or("Model not found")? };
        let exported_object = export_object(IndexedModelEntry {
            entry: entry.clone(),
            model: model.clone(),
        })?;
        data.push(exported_object);
    }
    return Ok(json!({
        "id": index.get_main_object_id(),
        "data": data
    }));
}

#[cfg(test)]
mod tests {
    use crate::json::export::export;
    use crate::json::import::import_from_string;
    use crate::test_kernel::create_test_kernel;
    use serde_json::json;
    use std::rc::Rc;

    #[test]
    pub fn test_json_import() {
        let json_data = json!({
           "data": [
              {
                 "data": {
                    "age": 12.0,
                    "name": "Hello",
                    "sub_items": {
                       "type": "subobject_array",
                       "value": [
                          {
                             "id": "id2"
                          }
                       ]
                    }
                 },
                 "id": "id1",
                 "model": "test/test",
                 "parent": null
              },
              {
                 "data": {
                    "age": 123.0,
                    "name": "Hello2",
                    "sub_items": {
                       "type": "subobject_array",
                       "value": [
                          {
                             "id": "id3"
                          }
                       ]
                    }
                 },
                 "id": "id2",
                 "model": "test/test",
                 "parent": {
                    "id": "id1",
                    "property": "sub_items"
                 }
              },
              {
                 "data": {
                    "age": 123.0,
                    "name": "Hello3"
                 },
                 "id": "id3",
                 "model": "test/test",
                 "parent": {
                    "id": "id2",
                    "property": "sub_items"
                 }
              }
           ],
           "id": "id1"
        });
        let (dump, json_data) = import(json_data);
        let (dump2, json_data) = import(json_data);
        let (dump3, _) = import(json_data);
        assert_eq!(dump, dump2, "Test 1");
        assert_eq!(dump2, dump3, "Test 2");
    }

    fn import(json_data: serde_json::Value) -> (String, serde_json::Value) {
        let dump = format!("{}", json_data);
        let kernel = Rc::new(create_test_kernel());
        let index = import_from_string(&dump, kernel.clone()).expect("Import failed");
        let exported = export(&index).expect("Export failed");
        (dump, exported)
    }
}
