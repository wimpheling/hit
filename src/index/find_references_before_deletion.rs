use super::remove_helpers::get_references;
use crate::index::Index;
use crate::index::IndexEntryProperty;
use crate::object_data::ObjectValue;
use crate::HitError;
use std::collections::HashMap;

fn add_references_to_output(
    index: &Index,
    id: &str,
    output: &mut HashMap<String, Vec<IndexEntryProperty>>,
    id_list: &mut Vec<String>,
) -> Result<(), HitError> {
    let refs = get_references(index, id)?;
    if refs.len() > 0 {
        output.insert(id.to_string(), refs);
    }
    id_list.push(id.to_string());

    let entry = index.get(id).ok_or(HitError::IDNotFound(id.to_string(), "add_references_to_output".to_string()))?;
    for (_, value) in entry.borrow().data.iter() {
        match value {
            ObjectValue::VecSubObjects(value) => {
                for val in value {
                    add_references_to_output(index, &val.id, output, id_list)?;
                }
            }
            ObjectValue::SubObject(value) => {
                add_references_to_output(index, &value.id, output, id_list)?;
            }
            _ => {}
        }
    }
    Ok(())
}

pub(in crate::index) fn find_references_recursive(
    index: &Index,
    id: &str,
) -> Result<(HashMap<String, Vec<IndexEntryProperty>>, Vec<String>), HitError> {
    let mut output: HashMap<String, Vec<IndexEntryProperty>> = HashMap::new();
    let mut id_list: Vec<String> = vec![];

    add_references_to_output(index, id, &mut output, &mut id_list)?;

    // ignore references that would be deleted (references from a deleted objects)
    let mut filtered_output: HashMap<String, Vec<IndexEntryProperty>> = HashMap::new();
    for (key, value) in output.iter_mut() {
        value.retain(|entry| !id_list.contains(&entry.id));
        if value.len() > 0 {
            filtered_output.insert(key.to_string(), value.clone());
        }
    }

    Ok((filtered_output, id_list))
}
