use std::collections::HashMap;

use crate::{Hit, HitError, Id, IndexEntryProperty, ObjectValue, ObjectValues, Reference};

fn uuid() -> String {
    (0..10)
        .map(|_| (0x20u8 + (rand::random::<f32>() * 96.0) as u8) as char)
        .collect()
}

fn _copy_object(
    hit: &mut Hit,
    id: &Id,
    target: IndexEntryProperty,
    before_id: Option<String>,
    references_to_update: &mut Vec<ReferenceToUpdate>,
    updated_ids: &mut HashMap<Id, Id>,
) -> Result<Id, HitError> {
    let new_id = uuid();
    let entry = hit.get(id).ok_or(HitError::IDNotFound(id.to_string()))?;
    let model = hit
        .get_model(id)
        .ok_or(HitError::ModelDoesNotExist(id.into()))?;
    let mut values = ObjectValues::new();

    // copy simple values
    for (field_name, _field) in model.get_fields().iter() {
        let value = entry.get(field_name);
        match value {
            crate::ObjectValue::Reference(r) => {
                references_to_update.push(ReferenceToUpdate {
                    target: IndexEntryProperty {
                        id: new_id.clone(),
                        property: field_name.clone(),
                    },
                    reference: r.id,
                    vec: false,
                });
            }
            crate::ObjectValue::VecReference(refs) => {
                for r in refs.iter() {
                    references_to_update.push(ReferenceToUpdate {
                        target: IndexEntryProperty {
                            id: new_id.clone(),
                            property: field_name.clone(),
                        },
                        reference: r.id.clone(),
                        vec: true,
                    });
                }
            }
            crate::ObjectValue::SubObject(_) => {}
            crate::ObjectValue::VecSubObjects(_) => {}
            _ => {
                values.insert(field_name.to_string(), value.clone());
            }
        }
    }

    hit.insert(
        entry.get_model().get_name(),
        &new_id,
        values,
        target,
        before_id,
    )?;

    updated_ids.insert(id.clone(), new_id.clone());

    // copy subobjects
    for (field_name, _field) in model.get_fields().iter() {
        let value = entry.get(field_name);
        match value {
            crate::ObjectValue::Reference(_) => {}
            crate::ObjectValue::VecReference(_) => {}
            crate::ObjectValue::SubObject(subobject) => {
                _copy_object(
                    hit,
                    &subobject.id,
                    IndexEntryProperty {
                        id: new_id.clone(),
                        property: field_name.clone(),
                    },
                    None,
                    references_to_update,
                    updated_ids,
                )?;
            }
            crate::ObjectValue::VecSubObjects(subobjects) => {
                for subobject in subobjects.iter() {
                    _copy_object(
                        hit,
                        &subobject.id,
                        IndexEntryProperty {
                            id: new_id.clone(),
                            property: field_name.clone(),
                        },
                        None,
                        references_to_update,
                        updated_ids,
                    )?;
                }
            }
            _ => {}
        }
    }
    Ok(new_id)
}

struct ReferenceToUpdate {
    target: IndexEntryProperty,
    reference: Id,
    vec: bool,
}

fn get_updated_id(id: Id, updated_ids: &HashMap<Id, Id>) -> Id {
    match updated_ids.get(&id.clone()) {
        Some(updated_id) => updated_id.clone(),
        None => id,
    }
}

pub fn copy_object(
    hit: &mut Hit,
    id: &Id,
    target: IndexEntryProperty,
    before_id: Option<String>,
) -> Result<Id, HitError> {
    let mut references_to_update: Vec<ReferenceToUpdate> = vec![];
    let mut updated_ids: HashMap<Id, Id> = HashMap::new();
    let new_id = {
        _copy_object(
            hit,
            id,
            target,
            before_id,
            &mut references_to_update,
            &mut updated_ids,
        )?
    };

    // reproduce inner references, leave the others
    for reference in references_to_update.iter() {
        let updated_id = get_updated_id(reference.reference.clone(), &updated_ids);
        if reference.vec {
            hit.insert_reference(&updated_id, reference.target.clone());
        } else {
            hit.set(
                &reference.target.id,
                &reference.target.property,
                ObjectValue::Reference(Reference { id: updated_id }),
            )?;
        }
    }

    Ok(new_id)
}
