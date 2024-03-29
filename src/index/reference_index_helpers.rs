use crate::index::{Index, IndexEntryProperty};
use crate::object_data::ObjectValue;
use crate::object_data::ObjectValues;
use crate::object_data::Reference;
use crate::HitError;

pub fn index_reference(
    index: &Index,
    reference: &Reference,
    key: &str,
    id: &str,
) -> Result<(), HitError> {
    let entry = index
        .get(&reference.id)
        .ok_or(HitError::InvalidReference(reference.id.to_string()))?;
    entry.borrow_mut().references.push(IndexEntryProperty {
        id: id.to_string(),
        property: key.to_string(),
    });
    Ok(())
}

pub fn index_object_references(
    index: &mut Index,
    values: ObjectValues,
    id: &str,
) -> Result<(), HitError> {
    for (key, value) in values.iter() {
        match value {
            ObjectValue::Reference(reference) => {
                index_reference(index, reference, key, id)?;
            }
            ObjectValue::VecReference(references) => {
                for reference in references.iter() {
                    index_reference(index, reference, key, id)?;
                }
            }
            _ => {}
        }
    }
    Ok(())
}

pub fn unindex_reference(
    index: &mut Index,
    parent: IndexEntryProperty,
    id: &str,
) -> Result<(), HitError> {
    let entry = index.get(id).ok_or(HitError::IDNotFound(id.to_string(), "unindex_reference".to_string()))?;
    entry.borrow_mut().references.retain(|x| x != &parent);
    Ok(())
}
pub fn unindex_reference_from_property(
    index: &mut Index,
    id: &str,
    property: &str,
) -> Result<(), HitError> {
    let value = index.get_value(id, property);
    match value {
        Some(old_value) => match old_value {
            ObjectValue::Reference(old_value) => {
                unindex_reference(
                    index,
                    IndexEntryProperty {
                        property: property.to_string(),
                        id: id.to_string(),
                    },
                    &old_value.id,
                )?;
            }

            ObjectValue::VecReference(old_value) => {
                for val in old_value {
                    unindex_reference(
                        index,
                        IndexEntryProperty {
                            property: property.to_string(),
                            id: id.to_string(),
                        },
                        &val.id,
                    )?;
                }
            }
            _ => {}
        },
        None => {}
    }
    Ok(())
}
