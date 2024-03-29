use linked_hash_map::LinkedHashMap;

use crate::index::{IndexEntry, IndexEntryProperty};
use crate::model::validators::ValidatorContext;
use crate::model::Model;
use crate::object_data::Id;
use crate::object_data::{ObjectValue, ObjectValues};
use crate::plugins::Plugins;
use crate::utils::ModelPropertyVectors;
use crate::HitError;
use crate::Kernel;
use crate::{errors::ValidationError, events::FieldListenerRef};
use crate::{events::Listeners, hit_mod::hit_entry::HitEntry};
use crate::{helpers::copy_object, index::Index};
use crate::{hit_mod::helpers::can_move_object, ModelField};

use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

pub type HitPlugins = Plugins;
pub type HitKernel = dyn Kernel;

pub(crate) struct ModelIndex {
    pub map: HashMap<String, Rc<Model>>,
}

impl ModelIndex {
    pub fn new() -> Self {
        ModelIndex {
            map: HashMap::new(),
        }
    }
}

#[derive(Clone)]
pub struct Hit {
    pub index: Index,
    pub(crate) model_index: Rc<RefCell<ModelIndex>>,
    pub(crate) plugins: HitPlugins,
    pub kernel: Rc<HitKernel>,
    pub(crate) errors: ModelPropertyVectors<ValidationError>,
    pub(crate) errors_subscriptions: Listeners<Vec<ValidationError>>,
}

impl Hit {
    pub fn new(id: &str, model_type: &str, kernel: Rc<HitKernel>) -> Result<Hit, HitError> {
        Hit::new_with_values(id, kernel, LinkedHashMap::new(), model_type)
    }

    // imports the data of another hit instance into this hit
    // used for dependencies
    pub fn import(&mut self, hit: Hit, parent: IndexEntryProperty) -> Result<(), HitError> {
        let main_id = hit.get_main_object_id();
        let get_parent = |x: &IndexEntry| {
            if x.get_id() == main_id {
                return Some(parent.clone());
            }
            return x.get_parent().clone();
        };

        for (id, value) in hit.index.index.iter() {
            if id == main_id {
                continue;
            }
            let value = value.borrow();

            let new_entry = IndexEntry::new_raw(
                id.clone(),
                value.data.clone(),
                get_parent(&value),
                value.references.clone(),
            );
            let model = hit.get_model_or_error(&id)?;
            self.model_index.borrow_mut().map.insert(id.clone(), model);
            self.index.index.insert(id.clone(), new_entry);
        }

        // insert main
        let main = hit.get(&main_id).unwrap();
        let model = hit.get_model_or_error(main_id)?;

        self.insert(&model.get_name(), main_id, main.get_data(), parent, None)?;
        Ok(())
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
            errors_subscriptions: Listeners::new(),
        };
        for (key, value) in values.iter() {
            hit.set(id, key, value.clone())?;
        }
        hit.validate_all()?;
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
        before_id: Option<String>,
    ) -> Result<(), HitError> {
        // before plugins call
        for plugin in self.plugins.reference_plugins.clone().iter() {
            plugin.borrow_mut().on_before_add_reference(
                self,
                &id.to_string(),
                &target,
                &before_id,
            )?;
        }

        let is_valid = self.field_is_reference_array(&target)?;

        if is_valid {
            self.index
                .insert_reference(id, target.clone(), before_id.clone())?;
            for plugin in self.plugins.reference_plugins.clone().iter() {
                plugin.borrow_mut().on_after_add_reference(
                    self,
                    &id.to_string(),
                    &target,
                    &before_id,
                )?;
            }
            Ok(())
        } else {
            Err(HitError::InvalidReference(id.to_string()))
        }
    }

    pub fn remove_reference(
        &mut self,
        id: &str,
        parent: IndexEntryProperty,
    ) -> Result<(), HitError> {
        // before plugins call
        for plugin in self.plugins.reference_plugins.clone().iter() {
            plugin
                .borrow_mut()
                .on_before_remove_reference(self, &id.to_string(), &parent)?;
        }

        // check in model that this property exists and is of a valid type
        let target_model = self.get_model_or_error(&parent.id)?;
        let target_property = target_model
            .get_field(&parent.property)
            .ok_or(HitError::PropertyNotFound((&parent.property).into()))?;
        let target_property = target_property.borrow();
        if target_property.is_vec_reference() {
            self.index.remove_reference(id, parent.clone())?;

            for plugin in self.plugins.reference_plugins.clone().iter() {
                plugin
                    .borrow_mut()
                    .on_after_remove_reference(self, &id.to_string(), &parent)?;
            }
            Ok(())
        } else {
            Err(HitError::InvalidDataType())
        }
    }

    pub fn move_reference(
        &mut self,
        id: &str,
        target: IndexEntryProperty,
        before_id: Option<Id>,
    ) -> Result<(), HitError> {
        // before plugins call
        for plugin in self.plugins.reference_plugins.clone().iter() {
            plugin
                .borrow_mut()
                .on_before_remove_reference(self, &id.to_string(), &target)?;
        }

        // check in model that this property exists and is of a valid type
        let target_model = self.get_model_or_error(&target.id)?;
        let target_property = target_model
            .get_field(&target.property)
            .ok_or(HitError::PropertyNotFound((&target.property).into()))?;
        let target_property = target_property.borrow();
        if target_property.is_vec_reference() {
            self.index
                .move_reference(id, target.clone(), before_id.clone())?;

            for plugin in self.plugins.reference_plugins.clone().iter() {
                plugin
                    .borrow_mut()
                    .on_after_remove_reference(self, &id.to_string(), &target)?;
            }
            Ok(())
        } else {
            Err(HitError::InvalidDataType())
        }
    }

    pub fn get_references(&self, id: &str) -> Result<Vec<IndexEntryProperty>, HitError> {
        self.index.get_references(id)
    }

    pub fn find_references_recursive(
        &self,
        id: &str,
    ) -> Result<(HashMap<String, Vec<IndexEntryProperty>>, Vec<String>), HitError> {
        self.index.find_references_recursive(id)
    }

    pub fn remove_object(&mut self, id: &str) -> Result<Vec<String>, HitError> {
        let entry = self.index.get(id).ok_or(HitError::IDNotFound(
            id.to_string(),
            "remove_object entry".to_string(),
        ))?;
        let model = self.get_model(id).ok_or(HitError::IDNotFound(
            id.to_string(),
            "remove_object model".to_string(),
        ))?;

        // before plugins call
        for plugin in self.plugins.delete_plugins.clone().iter() {
            plugin.borrow_mut().on_before_delete_entry(
                &HitEntry {
                    entry: entry.clone(),
                    model: model.clone(),
                },
                self,
            )?;
        }

        let id_list = self.index.remove_object(id)?;

        // after plugins call
        for plugin in self.plugins.delete_plugins.clone().iter() {
            plugin.borrow_mut().on_after_delete_entry(
                &HitEntry {
                    entry: entry.clone(),
                    model: model.clone(),
                },
                self,
            )?;
        }

        //remove model indexes of the deleted objects
        for id in id_list.iter() {
            let mut model_index = self.model_index.borrow_mut();
            model_index.map.remove(id);
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
        target: IndexEntryProperty,
        before_id: Option<String>,
    ) -> Result<(), HitError> {
        //check destination is allowed
        let target_model = self.get_model_or_error(&target.id)?;
        if !self.model_index.borrow().map.contains_key(id) {
            return Err(HitError::IDNotFound(id.into(), "move_object".into()));
        }
        let original_parent = self
            .get_parent(id)
            .ok_or(HitError::CannotMoveRootObject())?;

        for plugin in self.plugins.plugins.iter() {
            plugin.borrow_mut().on_before_move_subobject(
                id.clone(),
                target.clone(),
                before_id.clone(),
                &self,
            )?;
        }

        self.can_move_object(id, &target.id, target_model.get_name(), &target.property)?;
        self.index
            .move_object(id, target.clone(), before_id.clone())?;
        let plugins = { self.plugins.plugins.clone() };
        for plugin in plugins.iter() {
            plugin.borrow_mut().on_after_move_subobject(
                id.clone(),
                target.clone(),
                original_parent.clone(),
                before_id.clone(),
                self,
            )?;
        }
        Ok(())
    }

    pub fn copy_object(
        &mut self,
        id: Id,
        target: IndexEntryProperty,
        before_id: Option<String>,
    ) -> Result<Id, HitError> {
        let target_model = self.get_model_or_error(&target.id)?;
        if !self.model_index.borrow().map.contains_key(&id) {
            return Err(HitError::IDNotFound(id.into(), "copy_object".into()));
        }
        self.can_move_object(&id, &target.id, target_model.get_name(), &target.property)?;

        let id = copy_object(self, &id, target, before_id)?;
        return Ok(id);
    }

    pub fn get_model(&self, id: &str) -> Option<Rc<Model>> {
        match self.model_index.borrow().map.get(id) {
            Some(model) => Some(model.clone()),
            None => None,
        }
    }

    fn get_model_or_error(&self, id: &str) -> Result<Rc<Model>, HitError> {
        self.get_model(id)
            .ok_or(HitError::IDNotFound(id.into(), "get_model".into()))
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
        let entry = self
            .get(id)
            .ok_or(HitError::IDNotFound(id.to_string(), "set".into()))?;
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
            )?;
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

        for plugin in { self.plugins.plugins.clone() }.iter() {
            plugin.borrow_mut().on_after_set_value(
                IndexEntryProperty {
                    id: id.into(),
                    property: property.into(),
                },
                &value,
                &old_value,
                self,
            )?;
        }

        self._validate_field(model_field, id, property, value)?;

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
                &before_id,
                &self,
            )?;
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
            .insert(id, values.clone(), parent.clone(), before_id.clone())?;
        self.model_index
            .borrow_mut()
            .map
            .insert(id.to_string(), new_object_model.clone());

        // after_add_entry hook
        for plugin in { self.get_plugins().plugins.clone() }.iter() {
            plugin.borrow_mut().on_after_add_entry(
                new_object_model.clone(),
                &id,
                values.clone(),
                parent.clone(),
                &before_id,
                self,
            )?;
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
            None => Err(HitError::IDNotFound(
                id.to_string(),
                "subscribe_field".into(),
            )),
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
            None => Err(HitError::IDNotFound(
                id.to_string(),
                "unsubscribe_field".into(),
            )),
        }
    }

    pub fn get_parent(&self, id: &str) -> Option<IndexEntryProperty> {
        self.get(id)?.get_parent()
    }

    pub fn get_parent_index(&self, id: &str) -> Option<usize> {
        let parent = self.get_parent(id)?;
        match self.get_value(&parent.id, &parent.property)? {
            ObjectValue::VecSubObjects(parent_value) => {
                parent_value.iter().position(|r| r.id == id)
            }
            _ => None,
        }
    }

    fn _validate_field(
        &mut self,
        model_field: &Rc<RefCell<dyn ModelField>>,
        id: &str,
        property: &str,
        value: ObjectValue,
    ) -> Result<(), HitError> {
        let validation_errors = model_field.borrow().validate(
            &value,
            &ValidatorContext {
                id: &id.to_string(),
                property: &property.to_string(),
                index: Rc::new(self),
            },
        )?;
        self.errors.delete(id, property);
        match validation_errors.clone() {
            None => {}
            Some(validation_errors) => {
                for error in validation_errors.into_iter() {
                    self.errors.add(id, property, error);
                }
            }
        }

        // dispatch event

        self.errors_subscriptions.dispatch_value(
            &Self::get_validation_subscription_key(id, property),
            &{
                match validation_errors {
                    Some(validation_errors) => validation_errors,
                    None => vec![],
                }
            },
        );

        Ok(())
    }

    pub fn validate_field(&mut self, id: &str, property: &str) -> Result<(), HitError> {
        let model = self.get_model_or_error(id)?;
        let model_field = model
            .get_field(property)
            .ok_or(HitError::PropertyNotFound(property.to_string()))?;

        let value = self.get_value(id, property);
        self._validate_field(
            model_field,
            id,
            property,
            value.or(Some(ObjectValue::Null)).unwrap(),
        )?;
        Ok(())
    }

    fn get_validation_subscription_key(id: &str, field: &str) -> String {
        format!("{}{}", id, field)
    }
    pub fn get_validation_errors(&self, id: &str, field: &str) -> Option<&Vec<ValidationError>> {
        self.errors.get(id, field)
    }
    pub fn subscribe_field_validation(
        &mut self,
        id: &str,
        field: &str,
        listener: FieldListenerRef<Vec<ValidationError>>,
    ) {
        self.errors_subscriptions
            .insert(&Self::get_validation_subscription_key(id, field), listener);
    }

    pub fn unsubscribe_field_validation(
        &mut self,
        id: &str,
        field: &str,
        listener_id: &str,
    ) -> Result<(), HitError> {
        self.errors_subscriptions.remove(
            &Self::get_validation_subscription_key(id, field),
            listener_id,
        )
    }

    pub(crate) fn validate_all(&mut self) -> Result<(), HitError> {
        let model_index = self.model_index.borrow().map.clone();

        for (id, model) in model_index.iter() {
            for (field_name, _field) in model.get_fields().iter() {
                let model_field = model
                    .get_field(field_name)
                    .ok_or(HitError::PropertyNotFound(field_name.to_string()))?;
                let value = self
                    .get_value(&id, field_name)
                    .or(Some(ObjectValue::Null))
                    .unwrap();
                self._validate_field(model_field, &id, field_name, value)?;
            }
        }
        Ok(())
    }
}
