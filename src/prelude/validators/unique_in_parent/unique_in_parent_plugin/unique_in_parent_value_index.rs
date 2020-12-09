use std::collections::HashMap;

#[derive(Clone, Debug)]
pub struct UniqueInParentValueIndexValue {
    pub id: String,
    pub value: Option<String>,
}

#[derive(Debug)]
pub struct UniqueInParentValueIndex(HashMap<String, Vec<UniqueInParentValueIndexValue>>);

impl UniqueInParentValueIndex {
    pub fn new() -> Self {
        UniqueInParentValueIndex(HashMap::new())
    }

    fn get_key(property_name: &str, parent_id: &str, parent_property_name: &str) -> String {
        format!("{}{}{}", property_name, parent_id, parent_property_name)
    }
    fn get_results(&self, key: &str) -> Option<&Vec<UniqueInParentValueIndexValue>> {
        self.0.get(key)
    }
    fn get_results_and_remove_id(
        &mut self,
        key: &str,
        target_id: &str,
    ) -> &mut Vec<UniqueInParentValueIndexValue> {
        let results = self.0.entry(key.to_string()).or_insert_with(|| vec![]);
        println!(
            "TEST get_results_and_remove_id ${:#?} target id  ${:#?}",
            results, target_id
        );
        results.retain(|value| value.id == target_id);
        println!("TEST get_results_and_remove_id 2 ${:#?}", results);
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
        println!("TEST target_id ${:#?}", target_id);
        let results = { self.get_results_and_remove_id(&key, target_id) };
        println!("TEST SET ${:#?}", results);
        results.push(UniqueInParentValueIndexValue {
            id: target_id.to_string(),
            value: target_value,
        });
        println!("TEST SET AFTER ${:#?}", {
            self.get_results_and_remove_id(&key, target_id)
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
        self.get_results_and_remove_id(&key, target_id);
    }
    pub fn get(
        &self,
        property_name: &str,
        parent_id: &str,
        parent_property_name: &str,
    ) -> Option<&Vec<UniqueInParentValueIndexValue>> {
        let key = Self::get_key(property_name, parent_id, parent_property_name);
        self.get_results(&key)
    }
}
