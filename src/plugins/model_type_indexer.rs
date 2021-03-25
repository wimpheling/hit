use crate::model::Model;
use crate::object_data::ObjectValues;
use crate::plugins::DeletePlugin;
use crate::plugins::InitEntryPlugin;
use crate::plugins::Plugin;
use crate::HitError;
use crate::Id;
use crate::{hit_mod::HitEntry, ObjectValue};
use crate::{index::IndexEntryProperty, Hit};
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

impl InitEntryPlugin for ModelTypeIndexer {
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

impl DeletePlugin for ModelTypeIndexer {
    fn on_before_delete_entry(
        &mut self,
        _entry: &HitEntry,
        _instance: &mut Hit,
    ) -> Result<(), HitError> {
        Ok(())
    }

    fn on_after_delete_entry(
        &mut self,
        entry: &HitEntry,
        _instance: &mut Hit,
    ) -> Result<(), HitError> {
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

impl Plugin for ModelTypeIndexer {
    fn on_before_add_entry(
        &mut self,
        _model: Rc<Model>,
        _id: &str,
        _data: ObjectValues,
        _parent: IndexEntryProperty,
        _before_id: &Option<Id>,
        _instance: &Hit,
    ) -> Result<(), HitError> {
        Ok(())
    }
    fn on_after_add_entry(
        &mut self,
        model: Rc<Model>,
        id: &str,
        _data: ObjectValues,
        _parent: IndexEntryProperty,
        _before_id: &Option<Id>,
        _instance: &mut Hit,
    ) -> Result<(), HitError> {
        self.add_to_index(model, id);
        Ok(())
    }
    fn on_after_set_value(
        &mut self,
        _property: IndexEntryProperty,
        _value: &ObjectValue,
        _old_value: &Option<ObjectValue>,
        _instance: &mut Hit,
    ) -> Result<(), HitError> {
        Ok(())
    }
    fn on_before_set_value(
        &mut self,
        _property: IndexEntryProperty,
        _value: &ObjectValue,
        _old_value: &Option<ObjectValue>,
        _instance: &Hit,
    ) -> Result<(), HitError> {
        Ok(())
    }

    fn on_before_move_subobject(
        &mut self,
        _id: &str,
        _target: IndexEntryProperty,
        _before_id: Option<String>,
        _instance: &Hit,
    ) -> Result<(), HitError> {
        Ok(())
    }

    fn on_after_move_subobject(
        &mut self,
        _id: &str,
        _target: IndexEntryProperty,
        _original_parent: IndexEntryProperty,
        _before_id: Option<String>,
        _instance: &mut Hit,
    ) -> Result<(), HitError> {
        Ok(())
    }
}
