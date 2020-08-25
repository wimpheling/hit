use crate::index::IndexEntryProperty;
use crate::index::IndexImporter;
use crate::model::index::indexed_model::ModelIndex;
use crate::model::index::{IndexedModel, IndexedModelKernel, IndexedModelPlugins};

use crate::object_data::ObjectValues;
use crate::plugins::Plugins;
use std::cell::RefCell;

use std::rc::Rc;

pub struct IndexModelImporter {
    index: IndexImporter,
    model_index: Rc<RefCell<ModelIndex>>,
    kernel: Rc<IndexedModelKernel>,
    plugins: IndexedModelPlugins,
}

impl IndexModelImporter {
    pub fn new(id: &str, kernel: Rc<IndexedModelKernel>) -> Self {
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
    ) -> Result<(), String> {
        let model = self.kernel.get_model(model_type).map_err(|_| "Err")?;
        self.model_index
            .borrow_mut()
            .map
            .insert(id.to_string(), model);
        self.index.add_item(id, values, parent)?;
        Ok(())
    }

    pub fn get_plugins(&self) -> &IndexedModelPlugins {
        &self.plugins
    }

    pub fn finish_import(self) -> Result<IndexedModel, String> {
        let index = self.index.finish_import();
        Ok(IndexedModel {
            index: index?,
            model_index: self.model_index,
            plugins: self.plugins,
            kernel: self.kernel,
        })
    }
}
