use linked_hash_map::LinkedHashMap;

use crate::{field_types::*, modele, Hit, IndexEntryProperty, ObjectValue};
use std::{cell::RefCell, rc::Rc};

use crate::{HitError, Kernel, Model, Plugin, Plugins};

#[derive(Debug)]
struct TestPlugin {
    before_insert_count: i32,
    after_insert_count: i32,
    before_set_value_count: i32,
    after_set_value_count: i32,
    before_move_subobject_count: i32,
    after_move_subobject_count: i32,
}

impl Plugin for TestPlugin {
    fn on_before_add_entry(
        &mut self,
        _extra_data: Rc<Model>,
        _id: &str,
        _data: crate::ObjectValues,
        _parent: crate::IndexEntryProperty,
        _instance: &Hit,
    ) {
        self.before_insert_count += 1;
    }

    fn on_after_add_entry(
        &mut self,
        _extra_data: Rc<Model>,
        _id: &str,
        _data: crate::ObjectValues,
        _parent: crate::IndexEntryProperty,
        _instance: &mut Hit,
    ) {
        self.after_insert_count += 1;
    }

    fn on_before_set_value(
        &mut self,
        _property: IndexEntryProperty,
        _value: &ObjectValue,
        _old_value: &Option<ObjectValue>,
        _instance: &Hit,
    ) {
        self.before_set_value_count += 1;
    }

    fn on_after_set_value(
        &mut self,
        _property: IndexEntryProperty,
        _value: &ObjectValue,
        _old_value: &Option<ObjectValue>,
        _instance: &mut Hit,
    ) {
        self.after_set_value_count += 1;
    }

    fn on_before_move_subobject(
        &mut self,
        _id: &str,
        _target: IndexEntryProperty,
        _before_id: Option<String>,
        _instance: &Hit,
    ) -> Result<(), HitError> {
        self.before_move_subobject_count += 1;
        Ok(())
    }
    fn on_after_move_subobject(
        &mut self,
        _id: &str,
        _target: IndexEntryProperty,
        _original_parent: IndexEntryProperty,
        _before_id: Option<String>,
        _instance: &mut Hit,
    ) -> Result<(), HitError> {
        self.after_move_subobject_count += 1;
        Ok(())
    }
}

pub struct TestPluginKernel {
    model: Rc<Model>,
    model2: Rc<Model>,
    test_plugin: Rc<RefCell<TestPlugin>>,
}

impl Kernel for TestPluginKernel {
    fn get_model(&self, name: &str) -> Result<Rc<Model>, HitError> {
        if name == "test/test" {
            return Ok(self.model.clone());
        }
        if name == "test/test_b" {
            return Ok(self.model2.clone());
        }
        return Err(HitError::ModelDoesNotExist(name.into()));
    }

    fn get_instantiable_models(&self) -> Vec<&Model> {
        return vec![&self.model, &self.model2];
    }

    fn get_plugins(&self) -> Plugins {
        let mut plugins = Plugins::new();
        plugins.plugins.push(self.test_plugin.clone());
        plugins
    }

    fn get_models(&self) -> Vec<String> {
        vec!["test/test".to_string()]
    }
}

fn create_test_plugin_model() -> Rc<Model> {
    modele!("test/test", "Filesystem" =>
        "name": FieldTypeString {
            required: true
        },
        "subitems": FieldTypeSubobjectArray {
            authorized_models: vec!["test/test".to_string()]
        },
        "subitems2": FieldTypeSubobjectArray {
            authorized_models: vec!["test/impossible".to_string()]
        },
        "subitems3": FieldTypeSubobjectArray {
            authorized_models: vec!["test/test".to_string()]
        },
        "references": FieldTypeReferenceArray {
            authorized_models: vec!["test/test".to_string()]
        },
        "reference": FieldTypeReference {
            authorized_models: vec!["test/test".to_string()]
        }
    )
}

fn create_test_plugin_model2() -> Rc<Model> {
    modele!("test/test_b", "Filesystem" =>
        "name": FieldTypeString {
            required: true
        },
    )
}

fn create_test_plugin() -> Rc<RefCell<TestPlugin>> {
    Rc::new(RefCell::new(TestPlugin {
        before_insert_count: 0,
        after_insert_count: 0,
        before_set_value_count: 0,
        after_set_value_count: 0,
        before_move_subobject_count: 0,
        after_move_subobject_count: 0,
    }))
}

fn create_test_plugin_kernel(plugin: Rc<RefCell<TestPlugin>>) -> TestPluginKernel {
    TestPluginKernel {
        model: create_test_plugin_model(),
        model2: create_test_plugin_model2(),
        test_plugin: plugin,
    }
}
#[test]
fn it_should_call_only_before_set_value_callback_only_before_an_error() {
    let plugin = create_test_plugin();
    let mut hit_item = Hit::new(
        "id".into(),
        "test/test".into(),
        Rc::new(create_test_plugin_kernel(plugin.clone())),
    )
    .expect("Error 1");

    let error = hit_item.set("id", "reference", ObjectValue::Bool(false));

    assert_eq!(error.err().unwrap(), HitError::InvalidDataType());

    let borrowed_plugin = plugin.borrow();
    assert_eq!(borrowed_plugin.before_set_value_count, 1);
    assert_eq!(borrowed_plugin.after_set_value_count, 0);
}

#[test]
fn it_should_call_both_set_value_callbacks() {
    let plugin = create_test_plugin();
    let mut hit_item = Hit::new(
        "id".into(),
        "test/test".into(),
        Rc::new(create_test_plugin_kernel(plugin.clone())),
    )
    .expect("Error 1");

    hit_item
        .set("id", "name", ObjectValue::String("test".into()))
        .expect("Should work");

    let borrowed_plugin = plugin.borrow();
    assert_eq!(borrowed_plugin.before_set_value_count, 1);
    assert_eq!(borrowed_plugin.after_set_value_count, 1);
}

#[test]
fn it_should_call_both_insert_callbacks() {
    let plugin = create_test_plugin();
    let mut hit_item = Hit::new(
        "id".into(),
        "test/test".into(),
        Rc::new(create_test_plugin_kernel(plugin.clone())),
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
        .insert(
            "test/test".into(),
            "id3",
            LinkedHashMap::new(),
            IndexEntryProperty {
                id: "id".into(),
                property: "subitems".into(),
            },
            None,
        )
        .expect("Error 2");

    let borrowed_plugin = plugin.borrow();
    assert_eq!(borrowed_plugin.before_insert_count, 2);
    assert_eq!(borrowed_plugin.after_insert_count, 2);
}

#[test]
fn it_should_call_only_before_insert_callback_when_insert_fails() {
    let plugin = create_test_plugin();
    let mut hit_item = Hit::new(
        "id".into(),
        "test/test".into(),
        Rc::new(create_test_plugin_kernel(plugin.clone())),
    )
    .expect("Error 1");
    hit_item
        .insert(
            "test/test_b".into(),
            "id2",
            LinkedHashMap::new(),
            IndexEntryProperty {
                id: "id".into(),
                property: "subitems".into(),
            },
            None,
        )
        .expect_err("Should fail");

    let borrowed_plugin = plugin.borrow();
    assert_eq!(borrowed_plugin.before_insert_count, 1);
    assert_eq!(borrowed_plugin.after_insert_count, 0);
}

#[test]
fn it_should_call_both_move_callbacks() {
    let plugin = create_test_plugin();
    let mut hit_item = Hit::new(
        "id".into(),
        "test/test".into(),
        Rc::new(create_test_plugin_kernel(plugin.clone())),
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
        .move_object(
            "id2",
            IndexEntryProperty {
                id: "id".into(),
                property: "subitems3".into(),
            },
            None,
        )
        .expect("Error 2");
    let borrowed_plugin = plugin.borrow();
    assert_eq!(borrowed_plugin.before_move_subobject_count, 1);
    assert_eq!(borrowed_plugin.after_move_subobject_count, 1);
}
