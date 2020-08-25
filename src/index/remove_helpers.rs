use crate::index::reference_index_helpers::unindex_reference_from_property;
use crate::index::subobject_helpers::remove_subobject_from_parent_array;
use crate::index::Index;
use crate::index::IndexEntryProperty;

use crate::object_data::{ObjectValue};
use std::collections::HashMap;

fn find_references(
    index: &Index,
    id: &str,
    output: &mut HashMap<String, Vec<IndexEntryProperty>>,
) -> Result<(), String> {
    let entry = index.get(id).ok_or("Invalid id".to_string())?;
    //Error if entry has references
    let references = entry.borrow().references.clone();
    if references.len() > 0 {
        output.insert(id.to_string(), references);
        return Ok(());
    }
    Ok(())
}

pub fn find_references_recursive(
    index: &Index,
    id: &str,
) -> Result<HashMap<String, Vec<IndexEntryProperty>>, String> {
    let mut output = HashMap::new();
    find_references(index, id, &mut output)?;

    let entry = index.get(id).ok_or("Not found")?;
    for (_, value) in entry.borrow().data.iter() {
        match value {
            ObjectValue::VecSubObjects(value) => {
                for val in value {
                    find_references(index, &val.id, &mut output)?;
                }
            }
            ObjectValue::SubObject(value) => {
                find_references(index, &value.id, &mut output)?;
            }
            _ => {}
        }
    }
    Ok(output)
}

pub fn remove_object(index: &mut Index, id: &str) -> Result<(), String> {
    remove_object_children(index, id)?;
    let entry = index.get(id).ok_or("Index not found")?;
    for plugin in index.plugins.delete_plugins.iter() {
        plugin.borrow_mut().on_before_delete_entry(&entry)?;
    }
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

fn unindex_references_from_properties(index: &mut Index, id: &str) -> Result<(), String> {
    let entry = index.get(id).ok_or("id invalid")?;
    for (key, _) in entry.borrow().data.iter() {
        unindex_reference_from_property(index, id, key)?;
    }
    Ok(())
}

fn remove_object_children(index: &mut Index, id: &str) -> Result<(), String> {
    let entry = index.get(id).ok_or("Not found")?;
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
