use crate::index::{IndexEntryProperty, IndexEntryRef};
use crate::object_data::ObjectValue;
use crate::object_data::Reference;
use crate::HitError;

pub fn get_parent_property_value(
    parent_entry: &IndexEntryRef,
    parent: &IndexEntryProperty,
) -> ObjectValue {
    match parent_entry.borrow().data.get(&parent.property) {
        Some(data) => data.clone(),
        None => ObjectValue::Null,
    }
}

pub fn mutate_insert_in_ref_array(
    data: Vec<Reference>,
    id: &str,
    before_id: Option<String>,
) -> Result<Vec<Reference>, HitError> {
    let mut data = data.clone();
    let new_ref = Reference { id: id.into() };
    match before_id {
        Some(before_id) => {
            let position = data
                .iter()
                .position(|r| r.id == before_id)
                .ok_or(HitError::InvalidBeforeId(before_id.to_string()))?;
            data.insert(position, new_ref);
        }
        None => data.push(new_ref),
    }
    Ok(data)
}

// pub fn get_object_value_as_vec_reference(data: ObjectValue) -> Result<Vec<Reference>, String> {
//     match data {
//         ObjectValue::VecReference(data) => Ok(data),
//         ObjectValue::VecSubObjects(data) => Ok(data),
//         _ => Err("Invalid data type : not a reference_vec".to_string()),
//     }
// }
