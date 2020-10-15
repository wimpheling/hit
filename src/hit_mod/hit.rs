use linked_hash_map::LinkedHashMap;

use crate::hit_mod::helpers::can_move_object;
use crate::hit_mod::hit_entry::HitEntry;
use crate::index::Index;
use crate::index::IndexEntryProperty;
use crate::model::validators::ValidatorContext;
use crate::model::Model;
use crate::object_data::Id;
use crate::object_data::{ObjectValue, ObjectValues};
use crate::plugins::Plugins;
use crate::utils::ModelPropertyVectors;
use crate::HitError;
use crate::Kernel;
use crate::{errors::ValidationError, events::FieldListenerRef};

use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

pub type HitPlugins = Plugins<HitEntry>;
pub type HitKernel = dyn Kernel<HitEntry>;

pub(in crate) struct ModelIndex {
    pub map: HashMap<String, Rc<Model>>,
}

impl ModelIndex {
    pub fn new() -> Self {
        ModelIndex {
            map: HashMap::new(),
        }
    }
}

pub struct Hit {
    pub(in crate) index: Index,
    pub(in crate) model_index: Rc<RefCell<ModelIndex>>,
    pub(in crate) plugins: HitPlugins,
    pub kernel: Rc<HitKernel>,
    pub(in crate) errors: ModelPropertyVectors<ValidationError>,
}

impl Hit {
    pub fn new(id: &str, model_type: &str, kernel: Rc<HitKernel>) -> Result<Hit, HitError> {
        Hit::new_with_values(id, kernel, LinkedHashMap::new(), model_type)
    }
    pub fn new_with_values(
        id: &str,
        kernel: Rc<HitKernel>,
        values: ObjectValues,
        model_type: &str,
    ) -> Result<Hit, HitError> {
        let mut model_index = ModelIndex::new();
        let model = kernel.get_model(model_type)?;
        //TODO : initialize the values in the order defined by the model

        model_index.map.insert(id.to_string(), model);
        let model_index = Rc::new(RefCell::new(model_index));

        let mut hit = Hit {
            index: Index::new(id, LinkedHashMap::new())?,
            model_index: model_index,
            plugins: kernel.get_plugins(),
            kernel: kernel,
            errors: ModelPropertyVectors::new(),
        };
        for (key, value) in values.iter() {
            hit.set(id, key, value.clone())?;
        }
        Ok(hit)
    }

    pub fn contains_key(&self, key: &str) -> bool {
        return self.model_index.borrow().map.contains_key(key);
    }

    fn field_is_reference_array(&self, target: &IndexEntryProperty) -> Result<bool, HitError> {
        let target_model = self.get_model_or_error(&target.id)?;

        let target_model_field = target_model
            .get_field(&target.property)
            .ok_or(HitError::PropertyNotFound((&target.property).to_string()))?;

        let target_model_field_borrowed = target_model_field.borrow();
        if target_model_field_borrowed.is_vec_reference() {
            Ok(true)
        } else {
            Err(HitError::InvalidDataType())
        }
    }

    pub fn insert_reference(
        &mut self,
        id: &str,
        target: IndexEntryProperty,
    ) -> Result<(), HitError> {
        //TODO : validate that the model field accepts reference arrays
        let is_valid = self.field_is_reference_array(&target)?;

        if is_valid {
            self.index.insert_reference(id, target)
        } else {
            Err(HitError::InvalidReference(id.to_string()))
        }
    }

    pub fn remove_reference(
        &mut self,
        id: &str,
        parent: IndexEntryProperty,
    ) -> Result<(), HitError> {
        // check in model that this property exists and is of a valid type
        let target_model = self.get_model_or_error(&parent.id)?;
        let target_property = target_model
            .get_field(&parent.property)
            .ok_or(HitError::PropertyNotFound((&parent.property).into()))?;
        let target_property = target_property.borrow();
        if target_property.is_vec_reference() {
            self.index.remove_reference(id, parent)
        } else {
            Err(HitError::InvalidDataType())
        }
    }

    pub fn get_references(&self, id: &str) -> Result<Vec<IndexEntryProperty>, HitError> {
        self.index.get_references(id)
    }

    pub fn remove_object(&mut self, id: &str) -> Result<Vec<String>, HitError> {
        let entry = self
            .index
            .get(id)
            .ok_or(HitError::IDNotFound(id.to_string()))?;
        let model = self
            .get_model(id)
            .ok_or(HitError::IDNotFound(id.to_string()))?;

        // before plugins call
        for plugin in self.plugins.delete_plugins.iter() {
            plugin.borrow_mut().on_before_delete_entry(&HitEntry {
                entry: entry.clone(),
                model: model.clone(),
            })?;
        }

        let id_list = self.index.remove_object(id)?;

        //remove model indexes of the deleted objects
        for id in id_list.iter() {
            let mut model_index = self.model_index.borrow_mut();
            model_index.map.remove(id);
        }

        // after plugins call
        for plugin in self.plugins.delete_plugins.iter() {
            plugin.borrow_mut().on_after_delete_entry(&HitEntry {
                entry: entry.clone(),
                model: model.clone(),
            })?;
        }
        Ok(id_list)
    }

    pub fn can_move_object(
        &self,
        id: &str,
        target_id: &str,
        target_model: &str,
        property: &str,
    ) -> Result<(), HitError> {
        can_move_object(&self, id, target_id, target_model, property)
    }

    pub fn move_object(
        &mut self,
        id: &str,
        property: IndexEntryProperty,
        before_id: Option<String>,
    ) -> Result<(), HitError> {
        //check destination is allowed
        let target_model = self.get_model_or_error(&property.id)?;
        self.can_move_object(
            id,
            &property.id,
            target_model.get_name(),
            &property.property,
        )?;
        self.index.move_object(id, property, before_id)
    }

    pub fn get_model(&self, id: &str) -> Option<Rc<Model>> {
        match self.model_index.borrow().map.get(id) {
            Some(model) => Some(model.clone()),
            None => None,
        }
    }

    fn get_model_or_error(&self, id: &str) -> Result<Rc<Model>, HitError> {
        self.get_model(id).ok_or(HitError::IDNotFound(id.into()))
    }
    /*
    fn get_model_field_or_error(
        &self,
        id: &str,
        property: &str,
    ) -> Result<&ModelFieldRef, HitError> {
        let model = self.get_model_or_error(id)?;
        let a = model
            .get_field(property)
            .ok_or(HitError::PropertyNotFound(property.into()));
        return a;
    } */

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

    pub fn set(&mut self, id: &str, property: &str, value: ObjectValue) -> Result<(), HitError> {
        let entry = self.get(id).ok_or(HitError::IDNotFound(id.to_string()))?;
        let model_field = entry
            .model
            .get_field(property)
            .ok_or(HitError::PropertyNotFound(property.to_string()))?;

        let old_value = self.get_value(id, property);

        for plugin in self.plugins.plugins.iter() {
            plugin.borrow_mut().on_before_set_value(
                IndexEntryProperty {
                    id: id.into(),
                    property: property.into(),
                },
                &value,
                &old_value,
                &self,
            );
        }

        //does the field accept the object value
        if !model_field.borrow().accepts_for_set(
            &value,
            &ValidatorContext {
                id: id,
                property: property,
                index: Rc::new(self),
            },
        ) {
            return Err(HitError::InvalidDataType());
        }

        self.index.set_value(id, property, value.clone())?;

        let validation_errors = model_field.borrow().validate(
            &value,
            &ValidatorContext {
                id: &id.to_string(),
                property: &property.to_string(),
                index: Rc::new(self),
            },
        )?;
        match validation_errors {
            None => {
                self.errors.delete(id, property);
            }
            Some(validation_errors) => {
                self.errors.delete(id, property);
                for error in validation_errors.into_iter() {
                    self.errors.add(id, property, error);
                }
            }
        }

        for plugin in self.plugins.plugins.iter() {
            plugin.borrow_mut().on_after_set_value(
                IndexEntryProperty {
                    id: id.into(),
                    property: property.into(),
                },
                &value,
                &old_value,
                &self,
            );
        }
        Ok(())
    }

    fn validate_inserted_values(
        &mut self,
        new_object_model: &Rc<Model>,
        id: &str,
        values: &ObjectValues,
    ) -> Result<ObjectValues, HitError> {
        // put the data in the correct field order, initialize null data
        // and validate the data of the new object
        let mut ordered_values: ObjectValues = LinkedHashMap::new();
        for (property, model_field) in new_object_model.fields.iter() {
            //does the field accept the object value
            match values.get(property) {
                Some(value) => {
                    match value {
                        ObjectValue::Null => {}
                        _ => {
                            if !model_field.borrow().accepts_for_set(
                                &value,
                                &ValidatorContext {
                                    id: id,
                                    property: property,
                                    index: Rc::new(self),
                                },
                            ) {
                                return Err(HitError::InvalidDataType());
                            }
                        }
                    };
                    ordered_values.insert(property.to_string(), value.clone());
                }
                None => {
                    ordered_values.insert(property.to_string(), ObjectValue::Null);
                }
            }
        }
        Ok(ordered_values)
    }

    pub fn insert(
        &mut self,
        model_type: &str,
        id: &str,
        values: ObjectValues,
        parent: IndexEntryProperty,
        before_id: Option<String>,
    ) -> Result<(), HitError> {
        let new_object_model = self
            .kernel
            .get_model(&model_type.to_string())
            .map_err(|_| HitError::ModelDoesNotExist(model_type.to_string()))?;

        // before_add_entry hook
        for plugin in self.get_plugins().plugins.iter() {
            plugin.borrow_mut().on_before_add_entry(
                new_object_model.clone(),
                &id,
                values.clone(),
                parent.clone(),
                &self,
            );
        }

        let target_model = self.get_model_or_error(&parent.id)?;

        // verify that the model field exists and is of the right type
        let field = target_model
            .get_field(&parent.property)
            .ok_or(HitError::PropertyNotFound(parent.property.clone()))?;
        let field = field.borrow();
        if !field.is_vec_subobject() {
            return Err(HitError::CannotInsertObjectInThisDataType());
        }
        // check if model is authorized
        if !field.accepts_model(&new_object_model) {
            return Err(HitError::ModelNotAllowed(model_type.into()));
        }

        let values = self.validate_inserted_values(&new_object_model, id, &values)?;
        // update the data
        self.index
            .insert(id, values.clone(), parent.clone(), before_id)?;
        self.model_index
            .borrow_mut()
            .map
            .insert(id.to_string(), new_object_model.clone());

        // after_add_entry hook
        for plugin in self.get_plugins().plugins.iter() {
            plugin.borrow_mut().on_after_add_entry(
                new_object_model.clone(),
                &id,
                values.clone(),
                parent.clone(),
                &self,
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
        listener: FieldListenerRef<ObjectValue>,
    ) -> Result<String, HitError> {
        let model = self.get_model_or_error(id)?;
        model
            .get_field(field)
            .ok_or(HitError::PropertyNotFound(field.to_string()))?;

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
            None => Err(HitError::IDNotFound(id.to_string())),
        }
    }
    pub fn unsubscribe_field(
        &self,
        id: &str,
        field: &str,
        listener_id: &str,
    ) -> Result<(), HitError> {
        match self.index.get(id) {
            Some(entry) => {
                entry.borrow_mut().remove_listener(field, listener_id)?;
                Ok(())
            }
            None => Err(HitError::IDNotFound(id.to_string())),
        }
    }

    pub fn iter(&self) -> Option<HitEntry> {
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

    pub fn get_parent_index(&self, id: &str) -> Option<usize> {
        let parent = self.get(id)?.get_parent()?;
        match self.get_value(&parent.id, &parent.property)? {
            ObjectValue::VecSubObjects(parent_value) => {
                parent_value.iter().position(|r| r.id == id)
            }
            _ => None,
        }
    }
}
