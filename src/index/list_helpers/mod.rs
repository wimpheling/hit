mod private;

use crate::index::{Index, IndexEntryProperty, IndexEntryRef};
use crate::object_data::{ObjectValue, Reference};
use crate::HitError;
pub use private::{get_parent_property_value, mutate_insert_in_ref_array};

pub fn get_parent_index_entry(
    index: &Index,
    id: &str,
) -> Result<Option<(IndexEntryRef, IndexEntryProperty)>, HitError> {
    let entry = index
        .index
        .get(id)
        .ok_or(HitError::IDNotFound(id.to_string()))?
        .borrow();
    let parent = entry.get_parent();
    match parent {
        Some(parent) => Ok(Some(get_parent_index_entry_from_parent(index, parent)?)),
        None => Ok(None),
    }
}

pub fn get_parent_index_entry_from_parent(
    index: &Index,
    parent: IndexEntryProperty,
) -> Result<(IndexEntryRef, IndexEntryProperty), HitError> {
    match index.index.get(&parent.id) {
        Some(entry) => Ok((entry.clone(), parent.clone())),
        None => Err(HitError::InvalidParentID(parent.id.to_string())),
    }
}

pub fn dispatch_event(index: &Index, id: &str, property: &str) -> Result<(), HitError> {
    let entry = index.get(id).ok_or(HitError::IDNotFound(id.to_string()))?;
    let value = entry.borrow().get(&property).clone();
    entry.borrow_mut().dispatch_value(&property, value);
    Ok(())
}

pub fn mutate_insert_in_reference_array(
    data: ObjectValue,
    id: &str,
    before_id: Option<String>,
) -> Result<Vec<Reference>, HitError> {
    match data {
        ObjectValue::VecReference(data) => mutate_insert_in_ref_array(data, id, before_id),
        ObjectValue::Null => Ok(vec![Reference { id: id.into() }]),
        _ => Err(HitError::CannotInsertReferenceInThisDataType()),
    }
}

#[cfg(test)]
mod tests {
    use crate::index::{Index, IndexEntryProperty};
    use crate::HitError;
    use crate::ObjectValue;
    use crate::Plugins;
    use std::collections::HashMap;

    fn insert_string_field(
        index: &mut Index,
        id: &str,
        name: &str,
        parent: IndexEntryProperty,
    ) -> Result<(), HitError> {
        let mut values = HashMap::new();
        values.insert("name".into(), ObjectValue::String(name.to_string()));
        index.insert(id, values, parent, None)
    }

    fn insert_parent(index: &mut Index, id: &str, name: &str) -> Result<(), HitError> {
        let mut values = HashMap::new();
        values.insert("name".into(), ObjectValue::String(name.to_string()));
        index.insert_raw(id, values, None)
    }

    fn insert_sub_field(
        index: &mut Index,
        name: &str,
        id: &str,
        _position: u32,
    ) -> Result<(), HitError> {
        insert_string_field(
            index,
            id,
            name,
            IndexEntryProperty {
                property: "sub_items".to_string(),
                id: "id1".to_string(),
            },
        )
    }

    fn get_position(index: &Index, id: &str) -> usize {
        let parent = index
            .get(id)
            .expect("should be in the index")
            .borrow()
            .get_parent()
            .expect("should have a parent");
        let entry = index.get(&parent.id).expect("parent should exist");
        let entry = entry.borrow();
        let value = entry.get(&parent.property);
        match value {
            ObjectValue::VecReference(refs) => refs.iter().position(|r| r.id == id).unwrap(),
            ObjectValue::VecSubObjects(refs) => refs.iter().position(|r| r.id == id).unwrap(),
            _ => panic!("Invalid value"),
        }
    }

    fn init_index() -> Index {
        let mut index = Index::new(&String::from("id1"), Plugins::new());
        insert_parent(&mut index, "id1", "Name Of The Model 1").expect("Error");
        insert_sub_field(&mut index, "a", "id2", 0).expect("Error");
        insert_sub_field(&mut index, "b", "id3", 1).expect("Error");
        insert_sub_field(&mut index, "c", "id4", 2).expect("Error");
        index
    }

    #[test]
    fn it_should_update_the_positions_when_moving() {
        let mut index = init_index();
        index
            .move_object(
                "id4",
                IndexEntryProperty {
                    id: "id1".to_string(),
                    property: "sub_items".to_string(),
                },
                Some("id2".to_string()),
            )
            .expect("move to go ok");
        let position = get_position(&index, "id4");
        assert_eq!(position, 0);
        let position = get_position(&index, "id3");
        assert_eq!(position, 2);
    }
    #[test]
    fn it_should_update_the_positions_when_deleting() {
        let mut index = init_index();
        index.remove_object("id2").expect("should go ok");

        let position = get_position(&index, "id4");
        assert_eq!(position, 1);

        let position = get_position(&index, "id3");
        assert_eq!(position, 0);
    }
}
