use linked_hash_map::LinkedHashMap;

use crate::{field_types::*, modele, IndexEntryProperty};
use std::{cell::RefCell, rc::Rc};

use crate::{DeletePlugin, Hit, HitError, Kernel, Plugins};
use crate::{HitEntry, Model};

#[derive(Debug)]
struct TestDeletePlugin {
    before_delete_count: i32,
    after_delete_count: i32,
}

impl DeletePlugin for TestDeletePlugin {
    fn on_before_delete_entry(&mut self, _entry: &HitEntry) -> Result<(), HitError> {
        self.before_delete_count = self.before_delete_count + 1;
        Ok(())
    }

    fn on_after_delete_entry(&mut self, _entry: &HitEntry) -> Result<(), HitError> {
        self.after_delete_count = self.after_delete_count + 1;
        Ok(())
    }
}

pub struct TestDeletePluginKernel {
    model: Rc<Model>,
    test_delete_plugin: Rc<RefCell<TestDeletePlugin>>,
}

impl Kernel for TestDeletePluginKernel {
    fn get_model(&self, _name: &str) -> Result<Rc<Model>, HitError> {
        return Ok(self.model.clone());
    }

    fn get_instantiable_models(&self) -> Vec<&Model> {
        return vec![&self.model];
    }

    fn get_plugins(&self) -> crate::Plugins {
        let mut plugins = Plugins::new();
        plugins.delete_plugins.push(self.test_delete_plugin.clone());
        plugins
    }

    fn get_models(&self) -> Vec<String> {
        vec!["test/test".to_string()]
    }
}

fn create_test_delete_plugin_model() -> Rc<Model> {
    modele!("test/test", "Filesystem" =>
        "name": FieldTypeString {
            required: true
        },
        "subitems": FieldTypeSubobjectArray {
            authorized_models: vec!["test/test".to_string()]
        },
        "references": FieldTypeReferenceArray {
            authorized_models: vec!["test/test".to_string()]
        }
    )
}

fn create_test_delete_plugin() -> Rc<RefCell<TestDeletePlugin>> {
    Rc::new(RefCell::new(TestDeletePlugin {
        before_delete_count: 0,
        after_delete_count: 0,
    }))
}

fn create_test_delete_plugin_kernel(
    plugin: Rc<RefCell<TestDeletePlugin>>,
) -> TestDeletePluginKernel {
    TestDeletePluginKernel {
        model: create_test_delete_plugin_model(),
        test_delete_plugin: plugin,
    }
}

#[test]
fn it_should_call_the_before_delete_callback_only_before_an_error() {
    let plugin = create_test_delete_plugin();
    let mut hit_item = Hit::new(
        "id".into(),
        "test/test".into(),
        Rc::new(create_test_delete_plugin_kernel(plugin.clone())),
    )
    .expect("Error 1");
    hit_item
        .insert(
            "test/test".into(),
            "id2",
            LinkedHashMap::new(),
            IndexEntryProperty {
                id: "id".into(),
                property: "subitems".into(),
            },
            None,
        )
        .expect("Error 2");
    hit_item
        .insert_reference(
            "id2".into(),
            IndexEntryProperty {
                id: "id".into(),
                property: "references".into(),
            },
        )
        .expect("Error 3");

    hit_item.remove_object("id2").expect_err("Shouldn't work");

    let borrowed_plugin = plugin.borrow();
    assert_eq!(borrowed_plugin.before_delete_count, 1);
    assert_eq!(borrowed_plugin.after_delete_count, 0);
}

#[test]
fn it_should_call_both_callbacks_when_deleting() {
    let plugin = create_test_delete_plugin();
    let mut hit_item = Hit::new(
        "id".into(),
        "test/test".into(),
        Rc::new(create_test_delete_plugin_kernel(plugin.clone())),
    )
    .expect("Error 1");
    hit_item
        .insert(
            "test/test".into(),
            "id2",
            LinkedHashMap::new(),
            IndexEntryProperty {
                id: "id".into(),
                property: "subitems".into(),
            },
            None,
        )
        .expect("Error 2");

    hit_item.remove_object("id2").expect("Should work");

    let borrowed_plugin = plugin.borrow();
    assert_eq!(borrowed_plugin.before_delete_count, 1);
    assert_eq!(borrowed_plugin.after_delete_count, 1);
}
