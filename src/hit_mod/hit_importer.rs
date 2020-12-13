use linked_hash_map::LinkedHashMap;

use crate::index::IndexImporter;
use crate::object_data::ObjectValues;
use crate::HitError;
use crate::{events::Listeners, index::IndexEntryProperty};
use crate::{hit_mod::hit::ModelIndex, utils::ModelPropertyVectors};
use crate::{
    hit_mod::{Hit, HitKernel, HitPlugins},
    ObjectValue,
};
use std::cell::RefCell;

use std::rc::Rc;

pub struct IndexModelImporter {
    index: IndexImporter,
    model_index: Rc<RefCell<ModelIndex>>,
    kernel: Rc<HitKernel>,
    plugins: HitPlugins,
}

impl IndexModelImporter {
    pub fn new(id: &str, kernel: Rc<HitKernel>) -> Self {
        let model_index = ModelIndex::new();
        let model_index = Rc::new(RefCell::new(model_index));
        IndexModelImporter {
            index: IndexImporter::new(id),
            model_index: model_index,
            plugins: kernel.get_plugins(),
            kernel: kernel,
        }
    }

    pub fn add_item(
        &mut self,
        model_type: &str,
        id: &str,
        values: ObjectValues,
        parent: Option<IndexEntryProperty>,
    ) -> Result<(), HitError> {
        let model = self.kernel.get_model(model_type)?;
        self.model_index
            .borrow_mut()
            .map
            .insert(id.to_string(), model.clone());

        // put keys in the order defined in the model
        let mut new_values: ObjectValues = LinkedHashMap::new();
        for (key, _field) in model.fields.iter() {
            match values.get(key) {
                Some(value) => {
                    new_values.insert(key.to_string(), value.clone());
                }
                None => {
                    new_values.insert(key.to_string(), ObjectValue::Null);
                }
            }
        }

        self.index.add_item(id, values, parent)?;
        Ok(())
    }

    pub fn get_plugins(&self) -> &HitPlugins {
        &self.plugins
    }

    pub fn finish_import(self) -> Result<Hit, HitError> {
        let errors = ModelPropertyVectors::new();
        // validate every item
        let index = self.index.finish_import()?;
        let mut hit = Hit {
            index: index,
            model_index: self.model_index,
            plugins: self.plugins,
            kernel: self.kernel,
            errors: errors,
            errors_subscriptions: Listeners::new(),
        };
        hit.validate_all()?;

        Ok(hit)
    }
}
