use crate::index::list_helpers::get_parent_property_value;
use crate::index::{Index, IndexEntryProperty};
use crate::object_data::ObjectValue;
use crate::object_data::Reference;
use crate::HitError;

pub fn remove_reference_from_parent_array_from_property(
    index: &mut Index,
    parent: IndexEntryProperty,
    id: &str,
) -> Result<ObjectValue, HitError> {
    let entry = index
        .get(&parent.id)
        .ok_or(HitError::IDNotFound(parent.id.to_string()))?;
    let data = get_parent_property_value(&entry, &parent);
    let new_data = mutate_remove_from_reference_array(data, id)?;
    match new_data {
        Some(new_data) => {
            entry
                .borrow_mut()
                .data
                .insert(parent.property, ObjectValue::VecReference(new_data.clone()));
            Ok(ObjectValue::VecReference(new_data))
        }
        None => {
            entry
                .borrow_mut()
                .data
                .insert(parent.property, ObjectValue::Null);
            Ok(ObjectValue::Null)
        }
    }
}

pub fn mutate_remove_from_reference_array(
    data: ObjectValue,
    id: &str,
) -> Result<Option<Vec<Reference>>, HitError> {
    match data {
        ObjectValue::VecReference(data) => {
            let mut data = data.clone();
            data.retain(|x| x.id != id);
            if data.len() == 0 {
                return Ok(None);
            }
            Ok(Some(data))
        }
        _ => Err(HitError::CannotRemoveReferenceFromThisDataType()),
    }
}
