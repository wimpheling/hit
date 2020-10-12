use linked_hash_map::LinkedHashMap;

use crate::model::{Fields, ModelFieldRef};

pub struct Model {
    name: String,
    label: String,
    pub fields: Fields,
    pub interfaces: Vec<String>,
}

impl Model {
    pub fn new(name: String, label: String) -> Model {
        return Model {
            name: name,
            label: label,
            fields: LinkedHashMap::new(),
            interfaces: vec![],
        };
    }

    pub fn get_name(&self) -> &String {
        &self.name
    }
    pub fn get_label(&self) -> &String {
        &self.label
    }

    pub fn get_field(&self, name: &str) -> Option<&ModelFieldRef> {
        return self.fields.get(name);
    }

    pub fn implements_interface(&self, interface: &String) -> bool {
        return self.interfaces.contains(interface);
    }

    pub(in crate) fn get_fields(&self) -> &Fields {
        &self.fields
    }
}
