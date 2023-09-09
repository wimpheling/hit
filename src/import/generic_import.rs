use std::rc::Rc;

use linked_hash_map::LinkedHashMap;

use crate::{
    hit_mod::IndexModelImporter, Hit, HitError, HitKernel, IndexEntryProperty, Model,
    ModelFieldRef, ObjectValue, ObjectValues,
};

fn get_model_field(model: Rc<Model>, key: String) -> Result<ModelFieldRef, HitError> {
    let field = model.get_field(&key);
    match field {
        Some(field) => Ok(field.clone()),
        None => Err(HitError::PropertyNotFound(key.to_string())),
    }
}
pub fn import_data_object_values<'index>(
    model: Rc<Model>,
    id: String,
    parent: Option<IndexEntryProperty>,
    new_index: &'index mut IndexModelImporter,
    sub_data: ObjectValues,
) -> Result<(), HitError> {
    let mut new_data: ObjectValues = LinkedHashMap::new();

    for (key, value) in sub_data {
        //checks that the model field exists
        get_model_field(model.clone(), String::from(key.clone()))?;
        match value {
            ObjectValue::Null => {}
            _ => {
                new_data.insert(String::from(key), value);
            }
        }
    }

    new_index.add_item(model.get_name(), &id, new_data.clone(), parent.clone())?;

    for plugin in new_index.get_plugins().init_plugins.iter() {
        plugin
            .borrow_mut()
            .on_init_add_entry(model.clone(), &id, new_data.clone(), parent.clone());
    }
    return Ok(());
}

pub fn finish_import(
    new_index: IndexModelImporter,
    kernel: Rc<HitKernel>,
) -> Result<Hit, HitError> {
    let new_index = new_index.finish_import()?;

    for plugin in kernel.get_plugins().after_import_plugins.iter() {
        plugin.borrow_mut().after_import(&new_index)?;
    }
    Ok(new_index)
}
