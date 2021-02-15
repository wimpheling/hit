use crate::{Hit, HitError, Id, IndexEntryProperty, ObjectValues};

pub fn copy_object(
    hit: &mut Hit,
    id: &Id,
    target: IndexEntryProperty,
    before_id: Option<String>,
) -> Result<Id, HitError> {
    let new_id = nanoid::simple();
    let entry = hit.get(id).ok_or(HitError::IDNotFound(id.to_string()))?;
    let model = hit
        .get_model(id)
        .ok_or(HitError::ModelDoesNotExist(id.into()))?;
    let mut values = ObjectValues::new();

    // copy simple values
    for (field_name, _field) in model.get_fields().iter() {
        let value = entry.get(field_name);
        match value {
            crate::ObjectValue::Reference(_) => {}
            crate::ObjectValue::VecReference(_) => {}
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

    // copy subobjects
    for (field_name, _field) in model.get_fields().iter() {
        let value = entry.get(field_name);
        match value {
            crate::ObjectValue::Reference(_) => {}
            crate::ObjectValue::VecReference(_) => {}
            crate::ObjectValue::SubObject(subobject) => {
                copy_object(
                    hit,
                    &subobject.id,
                    IndexEntryProperty {
                        id: new_id.clone(),
                        property: field_name.clone(),
                    },
                    None,
                )?;
            }
            crate::ObjectValue::VecSubObjects(subobjects) => {
                for subobject in subobjects.iter() {
                    copy_object(
                        hit,
                        &subobject.id,
                        IndexEntryProperty {
                            id: new_id.clone(),
                            property: field_name.clone(),
                        },
                        None,
                    )?;
                }
            }
            _ => {}
        }
    }
    Ok(new_id)
}
