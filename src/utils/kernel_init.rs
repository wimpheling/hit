use std::{collections::HashMap, rc::Rc};

use crate::{HitError, Model};

pub fn kernel_init(models: HashMap<String, Rc<Model>>) -> Result<(), HitError> {
    for model in models.values() {
        for (_, field) in model.get_fields().iter() {
            let mut field = field.borrow_mut();
            field.on_kernel_init(model.get_name())?;
        }
    }
    Ok(())
}
