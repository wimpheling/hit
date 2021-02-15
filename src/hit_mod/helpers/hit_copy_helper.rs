use std::borrow::Borrow;

use crate::{Hit, HitError, Id, IndexEntryProperty, ObjectValues};

pub fn copy_object(
    hit: &mut Hit,
    id: &Id,
    target: IndexEntryProperty,
    before_id: Option<String>,
) -> Result<Id, HitError> {
    let entry = hit.get(id).ok_or(HitError::IDNotFound(id.to_string()))?;
    let model = hit
        .get_model(id)
        .ok_or(HitError::ModelDoesNotExist(id.clone()))?;
    let mut values = ObjectValues::new();
    for (field_name, _field) in model.get_fields().iter() {
        let value = entry.get(field_name);
        values.insert(field_name.to_string(), value.clone());
    }
    let new_id = nanoid::simple();

    hit.insert(
        entry.get_model().get_name(),
        &new_id,
        values,
        target,
        before_id,
    )?;
    Ok(new_id)
}
