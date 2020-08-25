use crate::index::list_helpers::dispatch_event;
use crate::index::subobject_helpers::{
    insert_subobject_in_array, remove_subobject_from_parent_array,
};
use crate::index::{Index, IndexEntryProperty};

fn set_object_parent(
    index: &mut Index,
    id: &str,
    new_parent: IndexEntryProperty,
) -> Result<(), String> {
    index
        .get_mut(id)
        .ok_or("Invalid index")?
        .borrow_mut()
        .set_parent(Some(new_parent));
    Ok(())
}

fn check_target_is_not_a_child(index: &Index, id: &str, target_id: &str) -> Result<bool, String> {
    let parent = index.get(target_id).ok_or("Invalid id")?;
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
) -> Result<(), String> {
    //AN OBJECT CANNOT BE ITS OWN PARENT !
    if target_parent.id == id {
        return Err("An object cannot be its own parent".to_string());
    }

    match index
        .get(id)
        .ok_or("Invalid main id")?
        .borrow()
        .get_parent()
    {
        Some(_) => {}
        None => return Err("You cannot move the root object".to_string()),
    }

    //AN OBJECT CANNOT MOVE INSIDE ITSELF
    if !check_target_is_not_a_child(index, id, &target_parent.id)? {
        return Err("Cannot move an object to a child of his".to_string());
    }
    Ok(())
}

pub fn can_move_object(
    index: &Index,
    id: &str,
    target_parent: IndexEntryProperty,
    before_id: Option<String>,
) -> bool {
    match _can_move_object(index, id, target_parent, before_id) {
        Err(_) => false,
        Ok(_) => true,
    }
}

pub fn move_object(
    index: &mut Index,
    id: &str,
    target_parent: IndexEntryProperty,
    before_id: Option<String>,
) -> Result<(), String> {
    if Some(id.to_string()) == before_id {
        return Ok(());
    }

    _can_move_object(index, id, target_parent.clone(), before_id.clone())?;

    let original_parent = index
        .get(id)
        .ok_or("Invalid index")?
        .borrow()
        .get_parent()
        .ok_or("Entry has no parent (main object)")?;
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
