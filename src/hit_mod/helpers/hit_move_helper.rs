use crate::field_types::FieldTypeSubobjectArray;
use crate::index::can_move_object as index_can_move_object;
use crate::model::Model;
use crate::IndexEntryProperty;
use crate::{Hit, HitError};
use std::rc::Rc;

// TODO : that's not generic, it should use the allow_model method
// this prevents from creating custom subobject fields with custom rules :/
fn _can_move_object(
    index: &Hit,
    id: &str,
    target_model: Rc<Model>,
    property: &str,
) -> Result<(), HitError> {
    let model = index
        .get_model(id)
        .ok_or(HitError::NoModelForId(id.to_string()))?;
    let target_field = target_model
        .get_field(property)
        .ok_or(HitError::PropertyNotFound(property.to_string()))?
        .borrow();
    /*let target_field = target_field
        .downcast_ref::<FieldTypeSubobjectArray>()
        .ok_or(HitError::InvalidMoveDestination())?;
      for allowed_model in target_field.authorized_models.iter() {
        if allowed_model == model.get_name() {
            return Ok(());
        }
        for interface in model.interfaces.iter() {
            if allowed_model == interface {
                return Ok(());
            }
        }
    }
    Err(HitError::ModelNotAllowed(model.get_name().clone()))*/
    Ok(())
}
pub fn can_move_object(
    index: &Hit,
    id: &str,
    target_id: &str,
    target_model: &str,
    property: &str,
) -> Result<(), HitError> {
    index_can_move_object(
        &index.index,
        id,
        IndexEntryProperty {
            id: target_id.to_string(),
            property: property.to_string(),
        },
        None,
    )?;
    let target_model = index.kernel.get_model(target_model)?;
    return _can_move_object(index, id, target_model, property);
}
