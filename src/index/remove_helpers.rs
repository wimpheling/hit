use crate::index::reference_index_helpers::unindex_reference_from_property;
use crate::index::subobject_helpers::remove_subobject_from_parent_array;
use crate::index::Index;
use crate::index::IndexEntryProperty;
use crate::object_data::ObjectValue;
use crate::HitError;
use crate::Hit;

fn find_references(index: &Index, id: &str) -> Result<Vec<IndexEntryProperty>, HitError> {
    let entry = index.get(id).ok_or(HitError::IDNotFound(id.to_string()))?;
    let entry = entry.borrow();
    Ok(entry.references.clone())
}

pub fn find_references_recursive(
    index: &Index,
    id: &str,
) -> Result<Vec<IndexEntryProperty>, HitError> {
    Ok(find_references(index, id)?)
}

pub fn remove_object(index: &mut Index, id: &str) -> Result<(), HitError> {
    remove_object_children(index, id)?;
    let entry = index.get(id).ok_or(HitError::IDNotFound(id.to_string()))?;
    //remove references from properties
    unindex_references_from_properties(index, id)?;
    //remove object from id list in parent
    remove_subobject_from_parent_array(index, id)?;
    //remove object from index
    index.index.remove(&id.to_string());

    for plugin in index.plugins.delete_plugins.iter() {
        plugin.borrow_mut().on_after_delete_entry(&entry)?;
    }
    Ok(())
}

fn unindex_references_from_properties(index: &mut Index, id: &str) -> Result<(), HitError> {
    let entry = index.get(id).ok_or(HitError::IDNotFound(id.to_string()))?;
    for (key, _) in entry.borrow().data.iter() {
        unindex_reference_from_property(index, id, key)?;
    }
    Ok(())
}

fn remove_object_children(index: &mut Index, id: &str) -> Result<(), HitError> {
    let entry = index.get(id).ok_or(HitError::IDNotFound(id.to_string()))?;
    for (_, value) in entry.borrow().data.iter() {
        match value {
            ObjectValue::VecSubObjects(value) => {
                for val in value {
                    remove_object(index, &val.id)?;
                }
            }
            ObjectValue::SubObject(value) => {
                remove_object(index, &value.id)?;
            }
            _ => {}
        }
    }
    Ok(())
}
