use std::{collections::HashMap, rc::Rc};

use crate::Model;

pub fn kernel_init(models: HashMap<String, Rc<Model>>) {
    for model in models.values() {
        for (_, field) in model.get_fields().iter() {
            let mut field = field.borrow_mut();
            field.on_kernel_init(model.get_name());
        }
    }
}
