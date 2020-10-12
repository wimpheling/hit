use linked_hash_map::LinkedHashMap;

use crate::{
    field_types::*, helpers::get_all_permissions, helpers::get_all_targets, modele,
    IndexEntryProperty,
};
use std::{rc::Rc};

use crate::{Hit, HitError, Kernel, Plugins};
use crate::{HitEntry, Model};

pub struct TestHelpersKernel {
    model: Rc<Model>,
    model2: Rc<Model>,
    model3: Rc<Model>,
}

impl Kernel<Rc<Model>, HitEntry> for TestHelpersKernel {
    fn get_model(&self, _name: &str) -> Result<Rc<Model>, HitError> {
        if _name == "test/test" {
            return Ok(self.model.clone());
        }
        if _name == "test/test2" {
            return Ok(self.model2.clone());
        }
        if _name == "test/test3" {
            return Ok(self.model3.clone());
        }
        Err(HitError::ModelDoesNotExist(_name.to_string()))
    }

    fn get_instantiable_models(&self) -> Vec<&Model> {
        return vec![&self.model];
    }

    fn get_plugins(&self) -> crate::Plugins<Rc<Model>, HitEntry> {
        Plugins::new()
    }

    fn get_models(&self) -> Vec<String> {
        vec![
            "test/test".to_string(),
            "test/test2".to_string(),
            "test/test3".to_string(),
        ]
    }
}

fn create_test_helpers_model() -> Rc<Model> {
    modele!("test/test", "Filesystem" =>
        "name": FieldTypeString {
            required: true
        },
        "subitems": FieldTypeSubobjectArray {
            authorized_models: vec!["test/test2".to_string(),"test/test".to_string()]
        },
        interfaces: "interface_ok",
    )
}
fn create_test_helpers_model_2() -> Rc<Model> {
    modele!("test/test2", "Filesystem" =>
        "name": FieldTypeString {
            required: true
        },
        "subitems_accept_object": FieldTypeSubobjectArray {
            authorized_models: vec!["test/test".to_string()]
        },
        "subitems_does_not_accept_object": FieldTypeSubobjectArray {
            authorized_models: vec!["test/test3".to_string()]
        },
        "subitems_accept_interface": FieldTypeSubobjectArray {
            authorized_models: vec!["test/test2".to_string(), "interface_ok".to_string()]
        },
        "subitems_accept_other_interface": FieldTypeSubobjectArray {
            authorized_models: vec!["test/test2".to_string(), "interface_not_ok".to_string()]
        },
    )
}
fn create_test_helpers_model_3() -> Rc<Model> {
    modele!("test/test3", "Filesystem" =>
        "name": FieldTypeString {
            required: true
        },
    )
}

fn create_test_helpers_kernel() -> TestHelpersKernel {
    TestHelpersKernel {
        model: create_test_helpers_model(),
        model2: create_test_helpers_model_2(),
        model3: create_test_helpers_model_3(),
    }
}

#[test]
fn it_should_find_all_targets_for_an_object() {
    let kernel = Rc::new(create_test_helpers_kernel());
    let mut hit_item = Hit::new("id", "test/test", kernel.clone()).expect("Error");

    hit_item
        .insert(
            "test/test",
            "id2",
            LinkedHashMap::new(),
            IndexEntryProperty {
                id: "id".into(),
                property: "subitems".into(),
            },
            None,
        )
        .expect("Error");
    hit_item
        .insert(
            "test/test2",
            "id3",
            LinkedHashMap::new(),
            IndexEntryProperty {
                id: "id2".into(),
                property: "subitems".into(),
            },
            None,
        )
        .expect("Error");

    hit_item
        .insert(
            "test/test3",
            "id5",
            LinkedHashMap::new(),
            IndexEntryProperty {
                id: "id3".into(),
                property: "subitems_does_not_accept_object".into(),
            },
            None,
        )
        .expect("Error");
    hit_item
        .insert(
            "test/test2",
            "id4",
            LinkedHashMap::new(),
            IndexEntryProperty {
                id: "id2".into(),
                property: "subitems".into(),
            },
            None,
        )
        .expect("Error");

    let permissions = get_all_permissions(kernel.clone()).expect("TODO2");

    let targets = get_all_targets("test/test", &hit_item, &permissions).expect("Error");
    let expected_results: Vec<String> = vec!["id".into(), "id2".into(), "id3".into(), "id4".into()];
    assert_eq!(targets, expected_results);
}
