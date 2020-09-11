use crate::index::index::IndexPlugins;
use crate::index::{Index, IndexEntryProperty};
use crate::object_data::{ObjectValue, ObjectValues};
use crate::HitError;
use std::collections::HashMap;

pub struct IndexImporter {
    collected_refs: HashMap<String, Vec<IndexEntryProperty>>,
    index: Index,
}

impl IndexImporter {
    pub fn new(id: &str, plugins: IndexPlugins) -> Self {
        IndexImporter {
            collected_refs: HashMap::new(),
            index: Index::new(id, plugins),
        }
    }

    pub fn add_item(
        &mut self,
        id: &str,
        values: ObjectValues,
        parent: Option<IndexEntryProperty>,
    ) -> Result<(), HitError> {
        self.index.insert_raw(id, values.clone(), parent)?;
        self.collect_references(values, id)?;
        Ok(())
    }

    pub fn finish_import(self) -> Result<Index, HitError> {
        for (id, vector) in self.collected_refs.iter() {
            let entry = self
                .index
                .get(id)
                .ok_or(HitError::IDNotFound(id.to_string()))?;
            for parent in vector.iter() {
                entry.borrow_mut().references.push(parent.clone());
            }
        }
        Ok(self.index)
    }

    fn collect_references(&mut self, values: ObjectValues, id: &str) -> Result<(), HitError> {
        for (key, value) in values.iter() {
            match value {
                ObjectValue::Reference(reference) => {
                    let vector = self
                        .collected_refs
                        .entry(reference.clone().id)
                        .or_insert_with(Vec::new);
                    vector.push(IndexEntryProperty {
                        id: id.to_string(),
                        property: key.to_string(),
                    });
                }
                ObjectValue::VecReference(references) => {
                    //
                    for reference in references.iter() {
                        let vector = self
                            .collected_refs
                            .entry(reference.clone().id)
                            .or_insert_with(Vec::new);
                        vector.push(IndexEntryProperty {
                            id: id.to_string(),
                            property: key.to_string(),
                        });
                    }
                }
                _ => {}
            }
        }
        Ok(())
    }
}
