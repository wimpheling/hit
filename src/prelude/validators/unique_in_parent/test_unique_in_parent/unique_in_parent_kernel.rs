use std::{cell::RefCell, collections::HashMap, rc::Rc};

use crate::{
    field_types::{FieldTypeString, FieldTypeSubobjectArray},
    modele,
    prelude::validators::unique_in_parent::unique_in_parent_plugin::UniqueInParentValueIndex,
    prelude::UniqueInParentPlugin,
    prelude::UniqueInParentValidator,
    utils::kernel_init,
    HitError, Kernel, Model, Plugins,
};

pub struct TestUniqueKernel {
    models: HashMap<String, Rc<Model>>,
    pub unique_in_parent_plugin: Rc<RefCell<UniqueInParentPlugin>>,
}

impl Kernel for TestUniqueKernel {
    fn get_model(&self, name: &str) -> Result<std::rc::Rc<Model>, HitError> {
        match self.models.get(name) {
            Some(model) => Ok(model.clone()),
            None => Err(HitError::ModelDoesNotExist(String::from(name))),
        }
    }

    fn get_instantiable_models(&self) -> Vec<&Model> {
        todo!()
    }

    fn get_plugins(&self) -> Plugins {
        let mut plugins = Plugins::new();

        // add unique in parent plugin
        plugins
            .delete_plugins
            .push(self.unique_in_parent_plugin.clone());
        plugins
            .init_plugins
            .push(self.unique_in_parent_plugin.clone());
        plugins.plugins.push(self.unique_in_parent_plugin.clone());

        plugins
    }

    fn get_models(&self) -> Vec<String> {
        let mut output = vec![];
        for (key, _) in self.models.iter() {
            output.push(key.to_string());
        }
        output
    }
}

pub fn create_test_unique_in_parent_kernel() -> TestUniqueKernel {
    let value_index = Rc::new(RefCell::new(UniqueInParentValueIndex::new()));
    let unique_in_parent_plugin =
        Rc::new(RefCell::new(UniqueInParentPlugin::new(value_index.clone())));
    let mut models = HashMap::new();
    models.insert(String::from("testunique/project"), modele!("testunique/project", "Project" =>
        "name": FieldTypeString {
            required: true,
            validators: vec![UniqueInParentValidator::new("name".to_string(), unique_in_parent_plugin.clone(), value_index.clone())]
        },
        "folders": FieldTypeSubobjectArray {
            authorized_models: vec![String::from("testunique/project"), String::from("testunique/folder")]
        },
    ));

    models.insert(String::from("testunique/folder"), modele!("testunique/folder", "Folder" =>
        "name": FieldTypeString {
            required: true,
            validators: vec![UniqueInParentValidator::new("name".to_string(), unique_in_parent_plugin.clone(), value_index.clone())]
        },
        "folders": FieldTypeSubobjectArray {
            authorized_models: vec![String::from("testunique/project"), String::from("testunique/folder")]
        },
    ));

    let kernel = TestUniqueKernel {
        models: models,
        unique_in_parent_plugin: unique_in_parent_plugin,
    };
    kernel_init(kernel.models.clone());
    kernel
}
