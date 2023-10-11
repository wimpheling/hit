// NOT USED FOR NOW

use std::collections::HashMap;

type PropertyMap<T> = HashMap<String, Vec<T>>;

type EntryMap<T> = HashMap<String, PropertyMap<T>>;

#[derive(Clone)]
pub struct ModelPropertyVectors<T: PartialEq> {
    entry_map: EntryMap<T>,
}

impl<T: PartialEq> ModelPropertyVectors<T> {
    pub fn new() -> ModelPropertyVectors<T> {
        ModelPropertyVectors {
            entry_map: EntryMap::new(),
        }
    }

    fn get_property_map(&self, id: &str) -> Option<&PropertyMap<T>> {
        self.entry_map.get(&id.to_string())
    }

    fn get_property_map_mut(&mut self, id: &str) -> Option<&mut PropertyMap<T>> {
        self.entry_map.get_mut(&id.to_string())
    }

    fn get_or_create_property_map_mut(&mut self, id: &str) -> &mut PropertyMap<T> {
        self.entry_map
            .entry(id.into())
            .or_insert_with(PropertyMap::new)
    }

    pub fn get(&self, id: &str, property: &str) -> Option<&Vec<T>> {
        let properties = self.get_property_map(id)?;
        properties.get(property)
    }

    fn get_or_create_mut(&mut self, id: &str, property: &str) -> &mut Vec<T> {
        let property_map = self.get_or_create_property_map_mut(id);
        property_map.entry(property.into()).or_insert_with(Vec::new)
    }

    /*    fn get_mut(&mut self, id: &str, property: &str) -> Option<&mut Vec<T>> {
        let properties = self.get_property_map_mut(id)?;
        properties.get_mut(property)
    } */

    pub fn add(&mut self, id: &str, property: &str, value: T) {
        let vector = self.get_or_create_mut(id, property);
        vector.push(value);
    }

    /*  pub fn remove(&mut self, id: &str, property: &str, value: &T) {
        let vector = self.get_mut(id, property);
        match vector {
            Some(vector) => {
                vector.retain(|v| v != value);
                if vector.len() == 0 {
                    self.delete(id, property);
                }
            }
            None => {}
        }
    } */

    pub fn delete(&mut self, id: &str, property: &str) {
        let property_map = self.get_property_map_mut(id);
        match property_map {
            Some(property_map) => {
                property_map.remove(property);
                if property_map.len() == 0 {
                    self.entry_map.remove(id);
                }
            }
            None => {}
        }
    }
}
