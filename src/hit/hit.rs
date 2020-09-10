use crate::events::FieldListenerRef;
use crate::hit::helpers::can_move_object;
use crate::hit::hit_entry::HitEntry;
use crate::index::Index;
use crate::index::IndexEntryProperty;
use crate::index::IndexEntryRef;
use crate::model::validators::ValidatorContext;
use crate::model::Model;
use crate::object_data::Id;
use crate::object_data::{ObjectValue, ObjectValues, Reference};
use crate::plugins::DeletePlugin;
use crate::plugins::Plugins;
use crate::Kernel;

use std::cell::RefCell;
use std::collections::HashMap;
use std::iter::Iterator;
use std::rc::Rc;

pub type HitPlugins = Plugins<Rc<Model>, HitEntry>;
pub type HitKernel = dyn Kernel<Rc<Model>, HitEntry>;

pub(in crate) struct ModelIndex {
    pub map: HashMap<String, Rc<Model>>,
    pub plugins: Vec<Rc<RefCell<dyn DeletePlugin<HitEntry>>>>,
}

impl ModelIndex {
    pub fn new() -> Self {
        ModelIndex {
            map: HashMap::new(),
            plugins: vec![],
        }
    }
}

pub struct Hit {
    pub(in crate) index: Index,
    pub(in crate) model_index: Rc<RefCell<ModelIndex>>,
    pub(in crate) plugins: HitPlugins,
    pub kernel: Rc<HitKernel>,
}

impl Hit {
    pub fn new(id: &str, kernel: Rc<HitKernel>) -> Hit {
        let mut model_index = ModelIndex::new();
        model_index.plugins = kernel.get_plugins().delete_plugins;
        let model_index = Rc::new(RefCell::new(model_index));
        let mut plugins = Plugins::new();
        plugins.delete_plugins.push(model_index.clone());
        Hit {
            model_index: model_index,
            index: Index::new(id, plugins),
            plugins: kernel.get_plugins(),
            kernel: kernel,
        }
    }
    pub fn new_with_values(
        id: &str,
        kernel: Rc<HitKernel>,
        values: ObjectValues,
        model_type: &str,
    ) -> Result<Hit, String> {
        let mut model_index = ModelIndex::new();
        model_index.plugins = kernel.get_plugins().delete_plugins;
        let model = kernel
            .get_model(model_type)
            .map_err(|_| "Model not found")?;

        //TODO : validate values
        //  - check all fields are in the model
        //  - check the values are accepted by the fields

        model_index.map.insert(id.to_string(), model);
        let model_index = Rc::new(RefCell::new(model_index));
        let mut plugins = Plugins::new();
        plugins.delete_plugins.push(model_index.clone());
        Ok(Hit {
            index: Index::new_with_values(id, values, plugins)?,
            model_index: model_index,
            plugins: kernel.get_plugins(),
            kernel: kernel,
        })
    }

    pub fn contains_key(&self, key: &str) -> bool {
        return self.model_index.borrow().map.contains_key(key);
    }

    fn validate_reference(&self, id: &str, target: IndexEntryProperty) -> Result<bool, String> {
        let target_model = self.model_index.borrow();
        let target_model = target_model
            .map
            .get(&target.id)
            .ok_or("Target Model not found")?;
        let target_model_field = target_model
            .get_field(&target.property)
            .ok_or("Invalid field")?;
        let target_model_field_borrowed = target_model_field.borrow();
        Ok(target_model_field_borrowed.accepts(
            &ObjectValue::Reference(Reference { id: id.to_string() }),
            &ValidatorContext {
                id: id,
                property: &target.property,
                index: Rc::new(self),
            },
        ))
    }
    pub fn insert_reference(&mut self, id: &str, target: IndexEntryProperty) -> Result<(), String> {
        let is_valid = self.validate_reference(id, target.clone())?;
        if is_valid {
            self.index.insert_reference(id, target)
        } else {
            Err("Object not accepted".to_string())
        }
    }

    pub fn remove_reference(&mut self, id: &str, parent: IndexEntryProperty) -> Result<(), String> {
        self.index.remove_reference(id, parent)
    }

    pub fn find_references_recursive(
        &self,
        id: &str,
    ) -> Result<HashMap<String, Vec<IndexEntryProperty>>, String> {
        self.index.find_references_recursive(id)
    }

    pub fn remove_object(&mut self, id: &str) -> Result<(), String> {
        self.index.remove_object(id)
    }

    pub fn can_move_object(
        &self,
        id: &str,
        target_id: &str,
        target_model: &str,
        property: &str,
    ) -> Result<bool, String> {
        can_move_object(&self, id, target_id, target_model, property)
    }

    pub fn move_object(
        &mut self,
        id: &str,
        property: IndexEntryProperty,
        before_id: Option<String>,
    ) -> Result<(), String> {
        //check destination is allowed
        let target_model = self.get_model(&property.id).ok_or("Model not found ")?;
        let ok = self.can_move_object(
            id,
            &property.id,
            target_model.get_name(),
            &property.property,
        )?;
        if ok {
            self.index.move_object(id, property, before_id)
        } else {
            Err("Can't move object".to_string())
        }
    }

    pub fn get_model(&self, id: &str) -> Option<Rc<Model>> {
        match self.model_index.borrow().map.get(id) {
            Some(model) => Some(model.clone()),
            None => None,
        }
    }

    pub fn get(&self, id: &str) -> Option<HitEntry> {
        let index_entry = self.index.get(id)?;
        let model = self.model_index.borrow();
        let model = model.map.get(id)?;
        Some(HitEntry {
            entry: index_entry,
            model: model.clone(),
        })
    }

    pub fn get_value(&self, id: &str, property: &str) -> Option<ObjectValue> {
        self.index.get_value(id, property)
    }

    pub fn set(&mut self, id: &str, property: &str, value: ObjectValue) -> Result<(), String> {
        let entry = self.get(id).ok_or("Invalid id")?;
        let model_field = entry.model.get_field(property).ok_or("Invalid field")?;
        //does the field accept the object value
        if !model_field.borrow().accepts(
            &value,
            &ValidatorContext {
                id: id,
                property: property,
                index: Rc::new(self),
            },
        ) {
            return Err("Invalid value type".to_string());
        }

        self.index.set_value(id, property, value.clone())?;

        //TODO: plugins

        model_field
            .borrow()
            .validate(
                &value,
                &ValidatorContext {
                    id: &id.to_string(),
                    property: &property.to_string(),
                    index: Rc::new(self),
                },
            )
            .map_err(|_| "Err".to_string())?;
        Ok(())
    }
    pub fn insert(
        &mut self,
        model_type: &str,
        id: &str,
        values: HashMap<String, ObjectValue>,
        parent: IndexEntryProperty,
        before_id: Option<String>,
    ) -> Result<(), String> {
        let model = self
            .kernel
            .get_model(&model_type.to_string())
            .map_err(|_| format!("Error getting model {}", model_type))?;
        for plugin in self.get_plugins().plugins.iter() {
            plugin.borrow_mut().on_before_add_entry(
                model.clone(),
                &id,
                values.clone(),
                parent.clone(),
            );
        }

        self.index
            .insert(id, values.clone(), parent.clone(), before_id)?;
        self.model_index
            .borrow_mut()
            .map
            .insert(id.to_string(), model.clone());

        for plugin in self.get_plugins().plugins.iter() {
            plugin.borrow_mut().on_after_add_entry(
                model.clone(),
                &id,
                values.clone(),
                parent.clone(),
            );
        }
        Ok(())
    }

    pub fn get_plugins(&self) -> &HitPlugins {
        return &self.plugins;
    }
    pub fn get_main_object_id(&self) -> &Id {
        return &self.index.get_main_object_id();
    }
    pub fn subscribe_field(
        &self,
        id: &str,
        field: &str,
        listener: FieldListenerRef,
    ) -> Result<String, String> {
        //check field exists
        let model = self.model_index.borrow();
        let model = model.map.get(id).ok_or("Model not found")?;
        model.get_field(field).ok_or("Field not found")?;

        match self.index.get(id) {
            Some(entry) => {
                {
                    entry.borrow_mut().add_listener(field, listener.clone());
                }
                let listener_id = {
                    let borrow = listener.borrow();
                    borrow.get_unique_id().to_string()
                };
                Ok(listener_id)
            }
            None => Err("Data not found".to_string()),
        }
    }
    pub fn unsubscribe_field(
        &self,
        id: &str,
        field: &str,
        listener_id: &str,
    ) -> Result<(), String> {
        match self.index.get(id) {
            Some(entry) => {
                entry.borrow_mut().remove_listener(field, listener_id)?;
                Ok(())
            }
            None => Err("Object not found".to_string()),
        }
    }
}
impl Iterator for Hit {
    type Item = HitEntry;
    fn next(&mut self) -> Option<HitEntry> {
        // Let's loop until we find something that should not be filtered.
        loop {
            match self.model_index.borrow().map.iter().next() {
                Some((id, model)) => {
                    let entry = self.index.get(&id)?;
                    return Some(HitEntry {
                        entry: entry.clone(),
                        model: model.clone(),
                    });
                }
                None => return None,
            }
        }
    }
}

impl DeletePlugin<IndexEntryRef> for ModelIndex {
    fn on_after_delete_entry(&mut self, entry: &IndexEntryRef) -> Result<(), String> {
        let model = self.map.get(entry.borrow().get_id()).ok_or("Err")?;
        for plugin in self.plugins.iter() {
            plugin.borrow_mut().on_after_delete_entry(&HitEntry {
                entry: entry.clone(),
                model: model.clone(),
            })?;
        }
        Ok(())
    }
    fn on_before_delete_entry(&mut self, entry: &IndexEntryRef) -> Result<(), String> {
        let model = self.map.get(entry.borrow().get_id()).ok_or("Err")?;
        for plugin in self.plugins.iter() {
            plugin.borrow_mut().on_before_delete_entry(&HitEntry {
                entry: entry.clone(),
                model: model.clone(),
            })?;
        }
        Ok(())
    }
}
