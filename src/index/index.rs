use crate::index::list_helpers::{
    get_parent_index_entry, get_parent_property_value, mutate_insert_in_reference_array,
};
use crate::index::move_object::move_object;
use crate::index::reference_helpers::remove_reference_from_parent_array_from_property;
use crate::index::reference_index_helpers::{
    index_object_references, index_reference, unindex_reference, unindex_reference_from_property,
};
use crate::index::remove_helpers::{get_references, remove_object_helper};
use crate::index::subobject_helpers::insert_subobject_in_array;
use crate::index::{IndexEntry, IndexEntryProperty, IndexEntryRef};
use crate::object_data::Id;
use crate::object_data::ObjectValue;
use crate::object_data::ObjectValues;
use crate::object_data::Reference;
use crate::HitError;
use std::collections::BTreeMap;
use std::collections::{btree_map::Iter, HashMap};

use super::find_references_before_deletion::find_references_recursive;

pub struct Index {
    pub(in crate::index) index: BTreeMap<Id, IndexEntryRef>,
    id: Id,
}

impl Index {
    pub fn new_for_import(id: &str) -> Index {
        Index {
            index: BTreeMap::new(),
            id: id.to_string(),
        }
    }

    pub fn new(id: &str, values: ObjectValues) -> Result<Index, HitError> {
        let mut index = Index {
            index: BTreeMap::new(),
            id: id.to_string(),
        };
        //Disallow references and subobjects
        for (_, value) in values.iter() {
            match value {
                ObjectValue::Reference(_) => {
                    return Err(HitError::CanOnlySetScalarValuesInInsertedObject())
                }
                ObjectValue::VecReference(_) => {
                    return Err(HitError::CanOnlySetScalarValuesInInsertedObject())
                }
                ObjectValue::SubObject(_) => {
                    return Err(HitError::CanOnlySetScalarValuesInInsertedObject())
                }
                ObjectValue::VecSubObjects(_) => {
                    return Err(HitError::CanOnlySetScalarValuesInInsertedObject())
                }
                _ => {}
            }
        }
        index.insert_raw(id, values, None)?;
        Ok(index)
    }

    pub fn get_main_object_id(&self) -> &Id {
        return &self.id;
    }

    pub fn get(&self, id: &str) -> Option<IndexEntryRef> {
        match self.index.get(id) {
            Some(entry) => Some(entry.clone()),
            None => None,
        }
    }

    pub fn get_mut(&mut self, id: &str) -> Option<&mut IndexEntryRef> {
        return self.index.get_mut(id);
    }

    pub fn get_value(&self, id: &str, property: &str) -> Option<ObjectValue> {
        let obj = self.get(id)?;
        let obj = obj.borrow();
        let value = obj.get(&property);
        Some(value.clone())
    }

    pub fn set_value(
        &mut self,
        id: &str,
        property: &str,
        value: ObjectValue,
    ) -> Result<(), HitError> {
        //remove reference for old value
        // TODO : should be put in the ObjectValue::Reference case of the below match ?
        unindex_reference_from_property(self, id, property)?;

        match value.clone() {
            ObjectValue::Null => {}
            ObjectValue::Bool(_) => {}
            ObjectValue::Date(_) => {}
            ObjectValue::F32(_) => {}
            ObjectValue::Reference(value) => {
                index_reference(self, &value, property, id)?;
            }
            ObjectValue::String(_) => {}
            _ => return Err(HitError::CanOnlySetScalarValues()),
        }

        let entry = self.get(id).ok_or(HitError::IDNotFound(id.to_string()))?;
        entry.borrow_mut().set(property, value)?;
        Ok(())
    }

    pub(in crate) fn iter(&self) -> Iter<Id, IndexEntryRef> {
        return self.index.iter();
    }

    pub fn insert(
        &mut self,
        id: &str,
        values: ObjectValues,
        parent: IndexEntryProperty,
        before_id: Option<String>,
    ) -> Result<(), HitError> {
        self.insert_quietly(id, values, parent, before_id)?;
        //dispatch value to parent property
        let (entry, parent) =
            get_parent_index_entry(self, &id)?.ok_or(HitError::InvalidParentID(id.to_string()))?;
        Index::dispatch_value_property(entry, &parent.property);
        Ok(())
    }

    /**
     * Used for import
     */
    pub(in crate::index) fn insert_raw(
        &mut self,
        id: &str,
        values: ObjectValues,
        parent: Option<IndexEntryProperty>,
    ) -> Result<(), HitError> {
        //check id doesnt exist
        if self.index.contains_key(id) {
            return Err(HitError::DuplicateID(id.to_string()));
        }
        self.index.insert(
            id.to_string(),
            IndexEntry::new(id.to_string(), values, parent.clone()),
        );
        Ok(())
    }

    fn insert_quietly(
        &mut self,
        id: &str,
        values: ObjectValues,
        parent: IndexEntryProperty,
        before_id: Option<String>,
    ) -> Result<(), HitError> {
        // insert
        self.insert_raw(id, values.clone(), Some(parent.clone()))?;

        //index references to other objects
        index_object_references(self, values, id)?;

        // update the list of ids in the parent
        insert_subobject_in_array(self, parent, id, before_id)?;

        Ok(())
    }

    pub fn insert_reference(
        &mut self,
        id: &str,
        target: IndexEntryProperty,
        before_id: Option<Id>,
    ) -> Result<(), HitError> {
        //update reference index
        index_reference(
            self,
            &Reference { id: id.to_string() },
            &target.property,
            &target.id,
        )?;

        //generate mutated vector
        let target_entry = {
            self.get_mut(&target.id)
                .ok_or(HitError::IDNotFound(target.id.to_string()))?
        };
        let data = get_parent_property_value(&target_entry, &target);
        let data = mutate_insert_in_reference_array(data, id, before_id)?;
        let value = ObjectValue::VecReference(data);

        //update the value in the index entry
        target_entry
            .borrow_mut()
            .data
            .insert(target.clone().property, value.clone());

        //send the value as an event
        Index::dispatch_value(target_entry.clone(), &target.property, value);

        Ok(())
    }
    pub fn remove_reference(
        &mut self,
        id: &str,
        parent: IndexEntryProperty,
    ) -> Result<(), HitError> {
        let value = remove_reference_from_parent_array_from_property(self, parent.clone(), id)?;

        unindex_reference(self, parent.clone(), id)?;
        //dispatch event
        let entry = self
            .get(&parent.clone().id)
            .ok_or(HitError::IDNotFound(parent.clone().id.to_string()))?;
        Index::dispatch_value(entry, &parent.property, value);
        Ok(())
    }

    pub(in crate) fn get_references(&self, id: &str) -> Result<Vec<IndexEntryProperty>, HitError> {
        get_references(&self, id)
    }

    pub fn find_references_recursive(
        &self,
        id: &str,
    ) -> Result<(HashMap<String, Vec<IndexEntryProperty>>, Vec<String>), HitError> {
        find_references_recursive(self, id)
    }

    pub fn remove_object(&mut self, id: &str) -> Result<Vec<String>, HitError> {
        let (parent_entry, parent) =
            get_parent_index_entry(self, &id)?.ok_or(HitError::CannotDeleteRootObject())?;

        let (refs, id_list) = find_references_recursive(self, id)?;
        if refs.len() > 0 {
            return Err(HitError::CannotDeleteObjectWithReferences(refs));
        }
        remove_object_helper(self, id)?;

        //remove from ref index the references in the object's data
        Index::dispatch_value_property(parent_entry, &parent.property);
        Ok(id_list)
    }

    pub fn move_object(
        &mut self,
        id: &str,
        property: IndexEntryProperty,
        before_id: Option<String>,
    ) -> Result<(), HitError> {
        move_object(self, id, property, before_id)
    }

    fn dispatch_value_property(entry: IndexEntryRef, property: &str) {
        let value = entry.borrow().get(property).clone();
        Index::dispatch_value(entry, property, value)
    }

    fn dispatch_value(entry: IndexEntryRef, property: &str, value: ObjectValue) {
        entry.borrow_mut().dispatch_value(property, value);
    }
}

#[cfg(test)]
mod tests {
    use linked_hash_map::LinkedHashMap;

    use crate::index::Index;
    use crate::HitError;
    use crate::ObjectValue;
    use crate::Reference;

    #[test]
    fn it_should_create_a_new_index_with_values() {
        let mut values = LinkedHashMap::new();
        values.insert("test".into(), ObjectValue::Bool(true));
        values.insert("testString".into(), ObjectValue::String("value".into()));
        assert!(Index::new("id", values).is_ok());
    }

    #[test]
    fn it_should_fail_creating_a_new_index_with_reference_values() {
        let mut values = LinkedHashMap::new();
        values.insert(
            "reference".into(),
            ObjectValue::Reference(Reference { id: "a".into() }),
        );
        assert!(matches!(
            Index::new("id", values).err(),
            Some(HitError::CanOnlySetScalarValuesInInsertedObject())
        ));
    }

    #[test]
    fn it_should_fail_creating_a_new_index_with_reference_array_values() {
        let mut values = LinkedHashMap::new();
        values.insert(
            "reference".into(),
            ObjectValue::VecReference(vec![Reference { id: "a".into() }]),
        );
        assert!(matches!(
            Index::new("id", values).err(),
            Some(HitError::CanOnlySetScalarValuesInInsertedObject())
        ));
    }
    #[test]
    fn it_should_fail_creating_a_new_index_with_subobject_values() {
        let mut values = LinkedHashMap::new();
        values.insert(
            "reference".into(),
            ObjectValue::SubObject(Reference { id: "a".into() }),
        );
        assert!(Index::new("id", values).is_err());
    }
    #[test]
    fn it_should_fail_creating_a_new_index_with_subobject_array_values() {
        let mut values = LinkedHashMap::new();
        values.insert(
            "reference".into(),
            ObjectValue::VecSubObjects(vec![Reference { id: "a".into() }]),
        );
        assert!(Index::new("id", values).is_err());
    }

    #[test]
    fn it_should_get_existing_data() {
        let mut values = LinkedHashMap::new();
        values.insert("test".into(), ObjectValue::Bool(true));
        let index = Index::new("id", values).ok().unwrap();

        let item = index.get("id").unwrap();
        let item = item.borrow();

        let prop = item.get("test");
        match prop {
            ObjectValue::Bool(value) => {
                assert_eq!(value, &true);
            }
            _ => panic!("Should be a boolean"),
        }
    }
}
