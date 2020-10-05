use crate::index::IndexEntryProperty;
use crate::model::Model;
use crate::object_data::ObjectValues;
use crate::plugins::DeletePlugin;
use crate::plugins::InitPlugin;
use crate::plugins::Plugin;
use crate::HitError;
use crate::{hit_mod::HitEntry, ObjectValue};
use std::collections::HashMap;
use std::rc::Rc;

pub struct ModelTypeIndexer {
    index: HashMap<String, Vec<String>>,
}

impl ModelTypeIndexer {
    pub fn new() -> Self {
        ModelTypeIndexer {
            index: HashMap::new(),
        }
    }

    pub fn get(&self, model_type: &str) -> Option<&Vec<String>> {
        self.index.get(model_type)
    }

    fn add_to_index(&mut self, model: Rc<Model>, id: &str) {
        let vector = self
            .index
            .entry(model.get_name().to_string())
            .or_insert(vec![]);
        vector.push(id.to_string());
    }
}

impl InitPlugin<Rc<Model>, HitEntry> for ModelTypeIndexer {
    fn on_init_add_entry(
        &mut self,
        model: Rc<Model>,
        id: &str,
        _data: ObjectValues,
        _parent: Option<IndexEntryProperty>,
    ) {
        self.add_to_index(model, id);
    }
}

impl DeletePlugin<HitEntry> for ModelTypeIndexer {
    fn on_before_delete_entry(&mut self, _entry: &HitEntry) -> Result<(), HitError> {
        Ok(())
    }

    fn on_after_delete_entry(&mut self, entry: &HitEntry) -> Result<(), HitError> {
        let model = entry.get_model();
        let model_type = model.get_name();
        let id = entry.get_id();
        match self.index.get_mut(model_type) {
            Some(vector) => vector.retain(|i| i != &id),
            None => {}
        };
        Ok(())
    }
}

impl Plugin<Rc<Model>, HitEntry> for ModelTypeIndexer {
    fn on_before_add_entry(
        &mut self,
        _model: Rc<Model>,
        _id: &str,
        _data: ObjectValues,
        _parent: IndexEntryProperty,
    ) {
    }
    fn on_after_add_entry(
        &mut self,
        model: Rc<Model>,
        id: &str,
        _data: ObjectValues,
        _parent: IndexEntryProperty,
    ) {
        self.add_to_index(model, id);
    }
    fn on_after_set_value(
        &mut self,
        property: IndexEntryProperty,
        value: &ObjectValue,
        old_value: &Option<ObjectValue>,
    ) {
    }
    fn on_before_set_value(
        &mut self,
        property: IndexEntryProperty,
        value: &ObjectValue,
        old_value: &Option<ObjectValue>,
    ) {
    }
    fn on_before_move_subobject(&mut self) {}
    fn on_after_move_subobject(&mut self) {}
}
