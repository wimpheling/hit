use crate::kernel::Kernel;
use crate::model::field_types::FieldTypeReference;
use crate::model::field_types::FieldTypeReferenceArray;
use crate::model::field_types::FieldTypeString;
use crate::model::field_types::FieldTypeSubobjectArray;
use crate::model::Model;
use crate::plugins::Plugins;
use crate::HitEntry;
use crate::HitError;
use std::cell::RefCell;
use std::rc::Rc;

pub struct TestKernel {
    model: Rc<Model>,
}

impl Kernel<Rc<Model>, HitEntry> for TestKernel {
    fn get_model(&self, name: &str) -> Result<Rc<Model>, HitError> {
        if name == "test/test" {
            return Ok(self.model.clone());
        } else {
            return Err(HitError::ModelDoesNotExist(String::from(name)));
        }
    }

    fn get_instantiable_models(&self) -> Vec<&Model> {
        return vec![&self.model];
    }
    fn get_plugins(&self) -> Plugins<Rc<Model>, HitEntry> {
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
        Rc::new(RefCell::new(FieldTypeSubobjectArray {
            name: String::from("sub_items"),
            validators: vec![],
            authorized_models: vec!["test/test".into()],
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
    model.fields.insert(
        String::from("reference"),
        Rc::new(RefCell::new(FieldTypeReference {
            name: String::from("reference"),
            required: false,
            validators: vec![], 
            authorized_models: vec!["test/test".into()],
        })),
    );
    model.fields.insert(
        String::from("references"),
        Rc::new(RefCell::new(FieldTypeReferenceArray {
            name: String::from("references"),
            validators: vec![],
            authorized_models: vec!["test/test".into()],
        })),
    );
    return TestKernel {
        model: Rc::new(model),
    };
}
