// NOT USED FOR NOW

use std::collections::HashMap;

type PropertyMap<T> = HashMap<String, Vec<T>>;

type EntryMap<T> = HashMap<String, PropertyMap<T>>;

pub struct ModelPropertyVectors<T: PartialEq> {
    entry_map: EntryMap<T>,
}

impl<T: PartialEq> ModelPropertyVectors<T> {
    fn get_property_map(&self, id: &str) -> Option<&PropertyMap<T>> {
        self.entry_map.get(&id.to_string())
    }

    fn get_property_map_mut(&mut self, id: &str) -> Option<&mut PropertyMap<T>> {
        self.entry_map.get_mut(&id.to_string())
    }
    pub fn get(&self, id: &str, property: &str) -> Option<&Vec<T>> {
        let properties = self.get_property_map(id)?;
        properties.get(property)
    }
    fn get_mut(&mut self, id: &str, property: &str) -> Option<&mut Vec<T>> {
        let properties = self.get_property_map_mut(id)?;
        properties.get_mut(property)
    }

    pub fn add(&mut self, id: &str, property: &str, value: T) -> Result<(), String> {
        let vector = self
            .get_mut(id, property)
            .ok_or("Id not found".to_string())?;
        vector.push(value);
        Ok(())
    }

    pub fn remove(&mut self, id: &str, property: &str, value: &T) -> Result<(), String> {
        let vector = self
            .get_mut(id, property)
            .ok_or("Id not found".to_string())?;
        vector.retain(|v| v != value);
        Ok(())
    }
}
