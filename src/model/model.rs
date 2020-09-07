pub use crate::errors::ModelError;
use crate::model::{Fields, ModelFieldRef};
use std::collections::BTreeMap;

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
            fields: BTreeMap::new(),
            interfaces: vec![],
        };
    }
    pub fn new_with_fields(name: String, label: String, fields: Fields) -> Model {
        return Model {
            name: name,
            label: label,
            fields: fields,
            interfaces: vec![],
        };
    }

    pub fn get_name(&self) -> &String {
        return &self.name;
    }
    pub fn get_label(&self) -> &String {
        return &self.label;
    }

    pub fn get_field(&self, name: &str) -> Option<&ModelFieldRef> {
        return self.fields.get(name);
    }

    pub fn implements_interface(&self, interface: &String) -> bool {
        return self.interfaces.contains(interface);
    }

    pub(in crate) fn get_fields(&self) -> &Fields {
        return &self.fields;
    }
}

impl PartialEq for Model {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
    }
}
