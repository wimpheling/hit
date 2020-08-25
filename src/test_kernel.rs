use crate::kernel::Kernel;
use crate::model::field_types::FieldTypeString;
use crate::model::validators::MaxLength;
use crate::model::IndexedModelEntry;
use crate::model::Model;
use crate::model::ModelError;
use crate::plugins::Plugins;
use std::cell::RefCell;
use std::rc::Rc;

pub struct TestKernel {
    model: Rc<Model>,
}

impl Kernel<Rc<Model>, IndexedModelEntry> for TestKernel {
    fn get_model(&self, name: &str) -> Result<Rc<Model>, ModelError> {
        if name == "test/test" {
            return Ok(self.model.clone());
        } else {
            return Err(ModelError::ModelDoesNotExist(String::from(name)));
        }
    }

    fn get_instantiable_models(&self) -> Vec<&Model> {
        return vec![&self.model];
    }
    fn get_plugins(&self) -> Plugins<Rc<Model>, IndexedModelEntry> {
        Plugins::new()
    }
    fn get_models(&self) -> Vec<String> {
        vec!["test/test".to_string()]
    }
}

pub fn create_test_kernel() -> TestKernel {
    let mut model = Model::new(String::from("test/test"), String::from("Test"));
    model.fields.insert(
        String::from("name"),
        Rc::new(RefCell::new(FieldTypeString {
            name: String::from("name"),
            required: true,
            validators: vec![],
            _enum: None,
        })),
    );
    model.fields.insert(
        String::from("sub_items"),
        Rc::new(RefCell::new(FieldTypeString {
            name: String::from("sub_items"),
            required: true,
            validators: vec![Rc::new(RefCell::new(MaxLength { length: 8 }))],
            _enum: None,
        })),
    );
    model.fields.insert(
        String::from("age"),
        Rc::new(RefCell::new(FieldTypeString {
            name: String::from("age"),
            required: true,
            validators: vec![],
            _enum: None,
        })),
    );
    return TestKernel {
        model: Rc::new(model),
    };
}
