use crate::index::subobject_helpers::remove_subobject_from_parent_array;
use crate::index::Index;
use crate::index::IndexEntryProperty;
use crate::object_data::ObjectValue;
use crate::HitError;

pub fn get_references(index: &Index, id: &str) -> Result<Vec<IndexEntryProperty>, HitError> {
    let entry = index.get(id).ok_or(HitError::IDNotFound(id.to_string()))?;
    let entry = entry.borrow();
    Ok(entry.references.clone())
}

pub fn remove_object_helper(index: &mut Index, id: &str) -> Result<(), HitError> {
    let entry = index.get(id).ok_or(HitError::IDNotFound(id.to_string()))?;
    remove_object_children(index, id)?;
    //remove references from properties

    // TODO: what does this do exactly?
    // should not be used because you can't delete if there are references
    // unindex_references_from_properties(index, id)?;

    //remove object from id list in parent
    remove_subobject_from_parent_array(index, id)?;
    //remove object from index
    index.index.remove(&id.to_string());

    for plugin in index.plugins.delete_plugins.iter() {
        plugin.borrow_mut().on_after_delete_entry(&entry)?;
    }
    Ok(())
}

fn remove_object_children(index: &mut Index, id: &str) -> Result<(), HitError> {
    let data = {
        let entry = index.get(id).ok_or(HitError::IDNotFound(id.to_string()))?;
        let entry = entry.borrow();
        entry.data.clone()
    };
    for (_, value) in data.iter() {
        match value {
            ObjectValue::VecSubObjects(value) => {
                for val in value {
                    remove_object_helper(index, &val.id)?;
                }
            }
            ObjectValue::SubObject(value) => {
                remove_object_helper(index, &value.id)?;
            }
            _ => {}
        }
    }
    Ok(())
}
