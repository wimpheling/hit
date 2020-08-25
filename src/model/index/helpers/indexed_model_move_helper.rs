use crate::field_types::FieldTypeSubobjectArray;
use crate::index::can_move_object as index_can_move_object;
use crate::model::Model;
use crate::IndexEntryProperty;
use crate::IndexedModel;
use std::rc::Rc;

fn _can_move_object(
    index: &IndexedModel,
    id: &str,
    target_model: Rc<Model>,
    property: &str,
) -> Result<bool, String> {
    let model = index.get_model(id).ok_or("Model not found")?;
    let target_field = target_model
        .get_field(property)
        .ok_or(format!("Field not found: {}", property))?
        .borrow();
    let target_field = target_field
        .downcast_ref::<FieldTypeSubobjectArray>()
        .ok_or(format!("That field is not a reference array: {}", property))?;
    for allowed_model in target_field.authorized_models.iter() {
        if allowed_model == model.get_name() {
            return Ok(true);
        }
        for interface in model.interfaces.iter() {
            if allowed_model == interface {
                return Ok(true);
            }
        }
    }
    Ok(false)
}
pub fn can_move_object(
    index: &IndexedModel,
    id: &str,
    target_id: &str,
    target_model: &str,
    property: &str,
) -> Result<bool, String> {
    if !index_can_move_object(
        &index.index,
        id,
        IndexEntryProperty {
            id: target_id.to_string(),
            property: property.to_string(),
        },
        None,
    ) {
        return Ok(false);
    }
    let target_model = index
        .kernel
        .get_model(target_model)
        .map_err(|_| "Model not found")?;
    return _can_move_object(index, id, target_model, property);
}
