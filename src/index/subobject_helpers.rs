use crate::index::list_helpers::{
    get_parent_index_entry_from_parent, get_parent_property_value,
    mutate_insert_in_ref_array,
};
use crate::index::{Index, IndexEntryProperty};
use crate::object_data::ObjectValue;
use crate::object_data::Reference;
use crate::HitError;

pub fn remove_subobject_from_parent_array(index: &mut Index, id: &str) -> Result<(), HitError> {
    remove_from_subobject_array(index, &id)
}
/* TODO: remove ?

pub fn remove_subobject_from_parent_array_from_property(
    index: &mut Index,
    parent: IndexEntryProperty,
    id: &str,
) -> Result<ObjectValue, String> {
    let entry = index
        .get(&parent.id)
        .ok_or(format!("Id of referencer object not found : {}", parent.id))?;
    let data = get_parent_property_value(&entry, &parent);
    let new_data = mutate_remove_from_subobject_array(data, id)?;
    match new_data {
        Some(new_data) => {
            entry.borrow_mut().data.insert(
                parent.property,
                ObjectValue::VecSubObjects(new_data.clone()),
            );
            Ok(ObjectValue::VecSubObjects(new_data))
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
 */
pub fn insert_subobject_in_array(
    index: &mut Index,
    parent: IndexEntryProperty,
    id: &str,
    before_id: Option<String>,
) -> Result<(), HitError> {
    let (parent_index_entry, parent) = get_parent_index_entry_from_parent(index, parent)?;
    let reference_array = get_parent_property_value(&parent_index_entry, &parent);
    let new_reference_array = mutate_insert_in_subobject_array(reference_array, id, before_id)?;
    parent_index_entry.borrow_mut().data.insert(
        parent.property,
        ObjectValue::VecSubObjects(new_reference_array),
    );
    Ok(())
}

fn mutate_remove_from_subobject_array(
    data: ObjectValue,
    id: &str,
) -> Result<Option<Vec<Reference>>, HitError> {
    match data {
        ObjectValue::VecSubObjects(data) => {
            let mut data = data.clone();
            data.retain(|x| x.id != id);
            if data.len() == 0 {
                return Ok(None);
            }
            Ok(Some(data))
        }
        _ => Err(HitError::CannotRemoveObjectFromThisDataType()),
    }
}

fn remove_from_subobject_array(index: &Index, id: &str) -> Result<(), HitError> {
    let (array_of_refs, parent) = {
        let entry = index.get(id).ok_or(HitError::IDNotFound(id.to_string()))?;
        let entry = entry.borrow();
        let parent = entry
            .get_parent()
            .ok_or(HitError::IDNotFound(id.to_string()))?;
        (
            index.get_value(&parent.id, &parent.property),
            parent.clone(),
        )
    };
    let array_of_refs = array_of_refs.ok_or(HitError::IDNotFound(id.to_string()))?;
    // let refs_as_vec = get_object_value_as_vec_reference(array_of_refs.clone())?;
    let new_value = mutate_remove_from_subobject_array(array_of_refs, id)?;
    let parent_index_entry = index
        .get(&parent.id)
        .ok_or(HitError::IDNotFound(id.to_string()))?;
    match new_value {
        Some(new_data) => {
            parent_index_entry
                .borrow_mut()
                .data
                .insert(parent.property, ObjectValue::VecSubObjects(new_data));
        }
        None => {
            parent_index_entry
                .borrow_mut()
                .data
                .insert(parent.property, ObjectValue::Null);
        }
    }
    Ok(())
}
fn mutate_insert_in_subobject_array(
    data: ObjectValue,
    id: &str,
    before_id: Option<String>,
) -> Result<Vec<Reference>, HitError> {
    match data {
        ObjectValue::VecSubObjects(data) => mutate_insert_in_ref_array(data, id, before_id),
        ObjectValue::Null => Ok(vec![Reference { id: id.into() }]),
        _ => Err(HitError::CannotInsertObjectInThisDataType()),
    }
}
