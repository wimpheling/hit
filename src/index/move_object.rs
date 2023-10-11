use crate::index::list_helpers::dispatch_event;
use crate::index::subobject_helpers::{
    insert_subobject_in_array, remove_subobject_from_parent_array,
};
use crate::index::{Index, IndexEntryProperty};
use crate::HitError;

fn set_object_parent(
    index: &mut Index,
    id: &str,
    new_parent: IndexEntryProperty,
) -> Result<(), HitError> {
    index
        .get_mut(id)
        .ok_or(HitError::IDNotFound(id.to_string(), "set_object_parent".to_string()))?
        .borrow_mut()
        .set_parent(Some(new_parent));
    Ok(())
}

fn check_target_is_not_a_child(index: &Index, id: &str, target_id: &str) -> Result<bool, HitError> {
    let parent = index
        .get(target_id)
        .ok_or(HitError::IDNotFound(target_id.to_string(), "check_target_is_not_a_child".to_string()))?;
    let parent_parent = parent.borrow().get_parent();
    match parent_parent {
        None => Ok(true),
        Some(parent_parent) => {
            if &parent_parent.id == id {
                Ok(false)
            } else {
                return check_target_is_not_a_child(index, id, &parent_parent.id);
            }
        }
    }
}

fn _can_move_object(
    index: &Index,
    id: &str,
    target_parent: IndexEntryProperty,
    _before_id: Option<String>,
) -> Result<(), HitError> {
    //AN OBJECT CANNOT BE ITS OWN PARENT !
    if target_parent.id == id {
        return Err(HitError::CannotBeOwnParent(id.to_string()));
    }

    match index
        .get(id)
        .ok_or(HitError::IDNotFound(id.to_string(), "_can_move_object".to_string()))?
        .borrow()
        .get_parent()
    {
        Some(_) => {}
        None => return Err(HitError::CannotMoveRootObject()),
    }

    //AN OBJECT CANNOT MOVE INSIDE ITSELF
    if !check_target_is_not_a_child(index, id, &target_parent.id)? {
        return Err(HitError::CannotBeOwnChild());
    }
    Ok(())
}

pub fn can_move_object(
    index: &Index,
    id: &str,
    target_parent: IndexEntryProperty,
    before_id: Option<String>,
) -> Result<(), HitError> {
    _can_move_object(index, id, target_parent, before_id)
}

pub fn move_object(
    index: &mut Index,
    id: &str,
    target_parent: IndexEntryProperty,
    before_id: Option<String>,
) -> Result<(), HitError> {
    if Some(id.to_string()) == before_id {
        return Ok(());
    }

    _can_move_object(index, id, target_parent.clone(), before_id.clone())?;

    let original_parent = index
        .get(id)
        .ok_or(HitError::IDNotFound(id.to_string(), "move_object".to_string()))?
        .borrow()
        .get_parent()
        .ok_or(HitError::CannotMoveRootObject())?;
    remove_subobject_from_parent_array(index, id)?;

    set_object_parent(index, id, target_parent.clone())?;

    insert_subobject_in_array(index, target_parent.clone(), id, before_id)?;

    //dispatch event
    dispatch_event(index, &target_parent.id, &target_parent.property)?;
    if target_parent.id != original_parent.id || target_parent.property != original_parent.property
    {
        dispatch_event(index, &original_parent.id, &original_parent.property)?;
    }
    Ok(())
}
