use std::{cell::RefCell, collections::HashSet, rc::Rc};

use crate::{DeletePlugin, Hit, Id, IndexEntryProperty, Model, ObjectValue, ObjectValues, Plugin};
use crate::{HitError, InitEntryPlugin};

use super::unique_in_parent_value_index::UniqueInParentValueIndex;

pub struct UniqueInParentPlugin {
    pub(in crate::prelude::validators::unique_in_parent) property_names: HashSet<String>,
    pub(in crate::prelude::validators::unique_in_parent) model_names: HashSet<String>,
    pub(in crate::prelude::validators::unique_in_parent) index:
        Rc<RefCell<UniqueInParentValueIndex>>,
}

impl UniqueInParentPlugin {
    pub fn new(index: Rc<RefCell<UniqueInParentValueIndex>>) -> Self {
        UniqueInParentPlugin {
            property_names: HashSet::new(),
            model_names: HashSet::new(),
            index: index,
        }
    }

    fn handle_new_object(
        &mut self,
        model: Rc<Model>,
        id: &str,
        data: ObjectValues,
        parent: IndexEntryProperty,
    ) {
        // only if model is tracked (faster)
        if self.model_names.contains(model.get_name()) {
            for (field_name, _field) in model.get_fields().iter() {
                // only for matched field names
                if self.property_names.contains(field_name) {
                    // only if there is data and it has a string value
                    match data.get(field_name) {
                        Some(data) => match data {
                            ObjectValue::String(value) => {
                                self.index.borrow_mut().set(
                                    // TODO : use the propertyname defined in the validator config
                                    field_name,
                                    &parent.id,
                                    &parent.property,
                                    id,
                                    Some(value.to_string()),
                                );
                            }
                            _ => {}
                        },
                        None => {}
                    }
                }
            }
        }
    }

    fn validate_index(
        &self,
        instance: &mut Hit,
        property_name: &str,
        parent_id: &str,
        parent_property_name: &str,
    ) -> Result<(), HitError> {
        match self
            .index
            .borrow()
            .get(property_name, parent_id, parent_property_name)
        {
            Some(index) => {
                for entry in index.iter() {
                    instance.validate_field(&entry.id, property_name)?;
                }
            }
            None => {}
        }
        Ok(())
    }
}

impl InitEntryPlugin for UniqueInParentPlugin {
    fn on_init_add_entry(
        &mut self,
        model: Rc<crate::Model>,
        id: &str,
        data: crate::ObjectValues,
        parent: Option<crate::IndexEntryProperty>,
    ) {
        // is not applied for the root object
        match parent {
            Some(parent) => self.handle_new_object(model, id, data, parent),
            _ => {}
        }
    }
}

impl DeletePlugin for UniqueInParentPlugin {
    fn on_before_delete_entry(
        &mut self,
        _entry: &crate::HitEntry,
        _instance: &mut crate::Hit,
    ) -> Result<(), HitError> {
        Ok(())
    }

    fn on_after_delete_entry(
        &mut self,
        entry: &crate::HitEntry,
        _instance: &mut crate::Hit,
    ) -> Result<(), HitError> {
        let model = entry.get_model();
        let parent = entry.get_parent().ok_or(HitError::NoParent())?;
        for (name, _field) in model.get_fields().iter() {
            if self.property_names.contains(name) {
                // Delete from index
                self.index.borrow_mut().remove_value(
                    name,
                    &parent.id,
                    &parent.property,
                    &entry.get_id(),
                );
                self.validate_index(_instance, name, &parent.id, &parent.property)?;
            }
        }
        Ok(())
    }
}

impl Plugin for UniqueInParentPlugin {
    fn on_before_add_entry(
        &mut self,
        _model: Rc<crate::Model>,
        _id: &str,
        _data: crate::ObjectValues,
        _parent: crate::IndexEntryProperty,
        _before_id: &Option<Id>,
        _instance: &crate::Hit,
    ) -> Result<(), HitError> {
        Ok(())
    }

    fn on_after_add_entry(
        &mut self,
        model: Rc<crate::Model>,
        id: &str,
        data: crate::ObjectValues,
        parent: crate::IndexEntryProperty,
        _before_id: &Option<Id>,
        instance: &mut crate::Hit,
    ) -> Result<(), HitError> {
        self.handle_new_object(model.clone(), id, data, parent.clone());
        if self.model_names.contains(model.get_name()) {
            for (field_name, _field) in model.get_fields().iter() {
                // only for matched field names
                if self.property_names.contains(field_name) {
                    self.validate_index(instance, field_name, &parent.id, &parent.property)?;
                }
            }
        }
        Ok(())
    }

    fn on_before_set_value(
        &mut self,
        _property: crate::IndexEntryProperty,
        _value: &ObjectValue,
        _old_value: &Option<ObjectValue>,
        _instance: &crate::Hit,
    ) -> Result<(), HitError> {
        Ok(())
    }

    fn on_after_set_value(
        &mut self,
        property: crate::IndexEntryProperty,
        value: &ObjectValue,
        _old_value: &Option<ObjectValue>,
        instance: &mut crate::Hit,
    ) -> Result<(), HitError> {
        if self.property_names.contains(&property.property) {
            match instance.get_parent(&property.id).clone() {
                Some(parent) => match value {
                    ObjectValue::String(value) => {
                        self.index.borrow_mut().set(
                            &property.property,
                            &parent.id,
                            &parent.property,
                            &property.id,
                            Some(value.to_string()),
                        );
                        self.validate_index(
                            instance,
                            &property.property,
                            &parent.id,
                            &parent.property,
                        )?;
                    }
                    _ => {}
                },
                None => {}
            }
        }
        Ok(())
    }

    fn on_before_move_subobject(
        &mut self,
        _id: &str,
        _target: crate::IndexEntryProperty,
        _before_id: Option<String>,
        _instance: &crate::Hit,
    ) -> Result<(), HitError> {
        Ok(())
    }

    fn on_after_move_subobject(
        &mut self,
        id: &str,
        target: crate::IndexEntryProperty,
        original_parent: crate::IndexEntryProperty,
        _before_id: Option<String>,
        instance: &mut crate::Hit,
    ) -> Result<(), HitError> {
        let model = instance
            .get_model(id)
            .ok_or(HitError::NoModelForId(id.to_string()))?;
        if self.model_names.contains(model.get_name()) {
            for (field_name, _field) in model.get_fields().iter() {
                if self.property_names.contains(field_name) {
                    // Remove from origin index
                    self.index.borrow_mut().remove_value(
                        field_name,
                        &original_parent.id,
                        &original_parent.property,
                        id,
                    );
                    self.validate_index(
                        instance,
                        field_name,
                        &original_parent.id,
                        &original_parent.property,
                    )?;

                    // Add to target index
                    let value = {
                        match instance.get_value(id, field_name) {
                            Some(value) => match value {
                                ObjectValue::String(value) => Some(value),
                                _ => None,
                            },
                            None => None,
                        }
                    };
                    self.index.borrow_mut().set(
                        field_name,
                        &target.id,
                        &target.property,
                        id,
                        value,
                    );

                    self.validate_index(instance, field_name, &target.id, &target.property)?;
                }
            }
        }
        Ok(())
    }
}
