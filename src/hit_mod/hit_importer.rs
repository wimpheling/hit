use linked_hash_map::LinkedHashMap;

use crate::index::IndexEntryProperty;
use crate::index::IndexImporter;
use crate::object_data::ObjectValues;
use crate::plugins::Plugins;
use crate::HitError;
use crate::{hit_mod::hit::ModelIndex, utils::ModelPropertyVectors};
use crate::{
    hit_mod::{Hit, HitKernel, HitPlugins},
    ObjectValue,
};
use std::{cell::RefCell, collections::HashMap};

use std::rc::Rc;

pub struct IndexModelImporter {
    index: IndexImporter,
    model_index: Rc<RefCell<ModelIndex>>,
    kernel: Rc<HitKernel>,
    plugins: HitPlugins,
}

impl IndexModelImporter {
    pub fn new(id: &str, kernel: Rc<HitKernel>) -> Self {
        let mut model_index = ModelIndex::new();
        model_index.plugins = kernel.get_plugins().delete_plugins;
        let model_index = Rc::new(RefCell::new(model_index));
        let mut plugins = Plugins::new();
        plugins.delete_plugins.push(model_index.clone());
        IndexModelImporter {
            index: IndexImporter::new(id, plugins),
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
        let index = self.index.finish_import()?;
        Ok(Hit {
            index: index,
            model_index: self.model_index,
            plugins: self.plugins,
            kernel: self.kernel,
            errors: ModelPropertyVectors::new(),
        })
    }
}
