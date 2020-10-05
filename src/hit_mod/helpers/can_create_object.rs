use crate::Hit;
use crate::HitError;
use crate::HitKernel;
use crate::Model;
use std::collections::HashMap;
use std::rc::Rc;

fn get_allowed_fields(
    kernel: Rc<HitKernel>,
    model: &Model,
    target_model_name: &str,
) -> Result<Option<Vec<String>>, HitError> {
    let mut allowed_fields = vec![];
    let target_model = kernel.get_model(target_model_name)?;
    let fields = target_model.get_fields();
    for (_, field) in fields {
        let borrowed_field = field.borrow();
        if borrowed_field.accepts_model(&model) {
            allowed_fields.push(borrowed_field.get_name());
        }
    }

    if allowed_fields.len() > 0 {
        Ok(Some(allowed_fields))
    } else {
        Ok(None)
    }
}

pub type ObjectPermissions = HashMap<String, HashMap<String, Vec<String>>>;

// Indexes all the models where a given model can be added (used for suggestions)
// TODO : test
pub fn get_all_permissions(kernel: Rc<HitKernel>) -> Result<ObjectPermissions, HitError> {
    let mut output: ObjectPermissions = HashMap::new();
    let list_of_models_clone = kernel.clone().get_models();

    for model_name in kernel.clone().get_models() {
        let model = kernel.clone().get_model(&model_name)?;
        let mut allowed_models = HashMap::new();
        for target_model_name in list_of_models_clone.clone() {
            match get_allowed_fields(kernel.clone(), &model, &target_model_name)? {
                Some(fields) => {
                    allowed_models.insert(target_model_name, fields);
                }
                None => {}
            }
        }
        output.insert(model_name.to_string(), allowed_models);
    }
    Ok(output)
}

// Get all destination models from an index, for a model type
// TODO : use index by type when they are implemented
pub fn get_all_targets(
    model_name: &str,
    index: &Hit,
    permissions: &ObjectPermissions,
) -> Result<Vec<String>, String> {
    match permissions.get(model_name) {
        Some(permissions) => {
            let mut destination_ids = vec![];
            let entries = &index.index;
            for (id, _) in entries.iter() {
                let target_model = index.get_model(id).ok_or("Model not found".to_string())?;
                let target_model_name = target_model.get_name();
                match permissions.get(target_model_name) {
                    Some(_) => {
                        destination_ids.push(id.to_string());
                    }
                    None => {}
                }
            }
            Ok(destination_ids)
        }
        None => Ok(vec![]),
    }
}
