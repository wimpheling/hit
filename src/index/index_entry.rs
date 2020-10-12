use crate::events::{FieldListenerRef, Listeners};
use crate::object_data::Id;
use crate::object_data::ObjectValue;
use crate::object_data::ObjectValues;
use crate::HitError;
use serde::{Deserialize, Serialize};
use std::cell::RefCell;
use std::default::Default;
use std::rc::Rc;

pub type IndexEntryRef = Rc<RefCell<IndexEntry>>;

#[derive(Serialize, Deserialize, Default, Clone, PartialEq, Debug)]
pub struct IndexEntryProperty {
    pub id: Id,
    pub property: String,
}

pub struct IndexEntry {
    id: Id,
    pub(in crate) data: ObjectValues,
    parent: Option<IndexEntryProperty>,
    pub(in crate::index) references: Vec<IndexEntryProperty>,
    property_change_listeners: Listeners<ObjectValue>,
}

impl IndexEntry {
    pub fn new(id: Id, data: ObjectValues, parent: Option<IndexEntryProperty>) -> IndexEntryRef {
        return Rc::new(RefCell::new(IndexEntry {
            id: String::from(id),
            data: data,
            parent: parent,
            references: vec![],
            property_change_listeners: Listeners::new(),
        }));
    }

    pub fn get(&self, property: &str) -> &ObjectValue {
        match self.data.get(property) {
            Some(data) => data,
            None => &ObjectValue::Null,
        }
    }

    pub fn get_parent(&self) -> Option<IndexEntryProperty> {
        return self.parent.clone();
    }

    pub(in crate) fn set_parent(&mut self, parent: Option<IndexEntryProperty>) {
        self.parent = parent;
    }

    pub fn get_parent_id(&self) -> Option<Id> {
        let id = &self.parent.as_ref()?.id;
        return Some(String::from(id));
    }

    pub fn get_id(&self) -> &Id {
        &self.id
    }

    pub(in crate::index) fn set(
        &mut self,
        property: &str,
        value: ObjectValue,
    ) -> Result<(), HitError> {
        self.data.insert(property.to_string(), value.clone());
        self.dispatch_value(property, value);
        Ok(())
    }

    pub(in crate::index) fn dispatch_value(&mut self, property: &str, value: ObjectValue) {
        self.property_change_listeners
            .dispatch_value(&property, &value.clone());
    }

    pub fn add_listener(&mut self, property: &str, listener: FieldListenerRef<ObjectValue>) {
        self.property_change_listeners.insert(property, listener);
    }

    pub fn remove_listener(&mut self, property: &str, listener_id: &str) -> Result<(), HitError> {
        self.property_change_listeners.remove(property, listener_id)
    }
}
