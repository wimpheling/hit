use std::collections::HashMap;

#[derive(Clone)]
struct UniqueInParentValueIndexValue {
    id: String,
    value: Option<String>,
}

pub struct UniqueInParentValueIndex(HashMap<String, Vec<UniqueInParentValueIndexValue>>);

impl UniqueInParentValueIndex {
    pub fn new() -> Self {
        UniqueInParentValueIndex(HashMap::new())
    }

    fn get_key(property_name: &str, parent_id: &str, parent_property_name: &str) -> String {
        format!("{}{}{}", property_name, parent_id, parent_property_name)
    }
    fn get_results(
        &mut self,
        key: &str,
        target_id: &str,
    ) -> &mut Vec<UniqueInParentValueIndexValue> {
        let results = self.0.entry(key.to_string()).or_insert_with(|| vec![]);
        results.retain(|value| value.id == target_id);
        results
    }
    pub fn set(
        &mut self,
        property_name: &str,
        parent_id: &str,
        parent_property_name: &str,
        target_id: &str,
        target_value: Option<String>,
    ) {
        let key = Self::get_key(property_name, parent_id, parent_property_name);
        let results = { self.get_results(&key, target_id) };
        results.push(UniqueInParentValueIndexValue {
            id: target_id.to_string(),
            value: target_value,
        });
    }
    pub fn remove_value(
        &mut self,
        property_name: &str,
        parent_id: &str,
        parent_property_name: &str,
        target_id: &str,
    ) {
        let key = Self::get_key(property_name, parent_id, parent_property_name);
        self.get_results(&key, target_id);
    }
}
