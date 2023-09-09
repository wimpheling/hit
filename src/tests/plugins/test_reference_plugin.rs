use linked_hash_map::LinkedHashMap;

use crate::{field_types::*, modele, IndexEntryProperty};
use std::{cell::RefCell, rc::Rc};

use crate::Model;
use crate::{Hit, HitError, Kernel, Plugins, ReferencePlugin};

#[derive(Debug)]
struct TestReferencePlugin {
    before_add_reference_count: i32,
    after_add_reference_count: i32,
    before_remove_reference_count: i32,
    after_remove_reference_count: i32,
    before_move_reference_count: i32,
    after_move_reference_count: i32,
}

impl ReferencePlugin for TestReferencePlugin {
    fn on_before_add_reference(
        &mut self,
        instance: &mut Hit,
        reference_id: &crate::Id,
        target: &IndexEntryProperty,
        before_id: &Option<String>,
    ) -> Result<(), HitError> {
        self.before_add_reference_count = self.before_add_reference_count + 1;
        Ok(())
    }

    fn on_after_add_reference(
        &mut self,
        instance: &mut Hit,
        reference_id: &crate::Id,
        target: &IndexEntryProperty,
        before_id: &Option<String>,
    ) -> Result<(), HitError> {
        self.after_add_reference_count = self.after_add_reference_count + 1;
        Ok(())
    }
    fn on_before_move_reference(
        &mut self,
        instance: &mut Hit,
        reference_id: &crate::Id,
        target: &IndexEntryProperty,
        before_id: &Option<String>,
    ) -> Result<(), HitError> {
        self.before_move_reference_count = self.before_move_reference_count + 1;
        Ok(())
    }

    fn on_after_move_reference(
        &mut self,
        instance: &mut Hit,
        reference_id: &crate::Id,
        target: &IndexEntryProperty,
        before_id: &Option<String>,
    ) -> Result<(), HitError> {
        self.after_move_reference_count = self.after_move_reference_count + 1;
        Ok(())
    }

    fn on_before_remove_reference(
        &mut self,
        instance: &mut Hit,
        reference_id: &crate::Id,
        target: &IndexEntryProperty,
    ) -> Result<(), HitError> {
        self.before_remove_reference_count = self.before_remove_reference_count + 1;
        Ok(())
    }

    fn on_after_remove_reference(
        &mut self,
        instance: &mut Hit,
        reference_id: &crate::Id,
        target: &IndexEntryProperty,
    ) -> Result<(), HitError> {
        self.after_remove_reference_count = self.after_remove_reference_count + 1;
        Ok(())
    }
}

pub struct TestReferencePluginKernel {
    model: Rc<Model>,
    test_reference_plugin: Rc<RefCell<TestReferencePlugin>>,
}

impl Kernel for TestReferencePluginKernel {
    fn get_model(&self, _name: &str) -> Result<Rc<Model>, HitError> {
        return Ok(self.model.clone());
    }

    fn get_instantiable_models(&self) -> Vec<&Model> {
        return vec![&self.model];
    }

    fn get_plugins(&self) -> crate::Plugins {
        let mut plugins = Plugins::new();
        plugins
            .reference_plugins
            .push(self.test_reference_plugin.clone());
        plugins
    }

    fn get_models(&self) -> Vec<String> {
        vec!["test/test".to_string()]
    }
}

fn create_test_remove_plugin_model() -> Rc<Model> {
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

fn create_test_delete_plugin() -> Rc<RefCell<TestReferencePlugin>> {
    Rc::new(RefCell::new(TestReferencePlugin {
        before_add_reference_count: 0,
        after_add_reference_count: 0,
        before_remove_reference_count: 0,
        after_remove_reference_count: 0,
        before_move_reference_count: 0,
        after_move_reference_count: 0,
    }))
}

fn create_test_delete_plugin_kernel(
    plugin: Rc<RefCell<TestReferencePlugin>>,
) -> TestReferencePluginKernel {
    TestReferencePluginKernel {
        model: create_test_remove_plugin_model(),
        test_reference_plugin: plugin,
    }
}

#[test]
fn it_should_call_both_callbacks_when_adding_and_removing_reference() {
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
            None,
        )
        .expect("Error");

    hit_item
        .remove_reference(
            "id2".into(),
            IndexEntryProperty {
                id: "id".into(),
                property: "references".into(),
            },
        )
        .expect("Error");
    let borrowed_plugin = plugin.borrow();
    assert_eq!(borrowed_plugin.before_add_reference_count, 1);
    assert_eq!(borrowed_plugin.after_add_reference_count, 1);

    assert_eq!(borrowed_plugin.before_remove_reference_count, 1);
    assert_eq!(borrowed_plugin.after_remove_reference_count, 1);
}
