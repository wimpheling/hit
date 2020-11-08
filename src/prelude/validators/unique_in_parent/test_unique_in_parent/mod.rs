use std::{collections::HashSet, rc::Rc};

use linked_hash_map::LinkedHashMap;

use crate::{Hit, IndexEntryProperty, ObjectValue, ValidationError};

use self::unique_in_parent_kernel::create_test_unique_in_parent_kernel;

mod unique_in_parent_kernel;

#[test]
fn it_should_initialize_the_model_index() {
    let kernel = create_test_unique_in_parent_kernel();
    let mut values = HashSet::new();
    values.insert("testunique/project".into());
    values.insert("testunique/folder".into());
    assert_eq!(kernel.unique_in_parent_plugin.borrow().model_names, values);
}
#[test]
fn it_should_initialize_the_property_index() {
    let kernel = create_test_unique_in_parent_kernel();
    let mut values = HashSet::new();
    values.insert("name".into());
    assert_eq!(
        kernel.unique_in_parent_plugin.borrow().property_names,
        values
    );
}

#[test]
fn it_should_create_errors_when_names_are_not_unique() {
    let kernel = Rc::new(create_test_unique_in_parent_kernel());
    let mut hit = Hit::new("id", "testunique/project", kernel.clone()).expect("Error");
    let mut name = LinkedHashMap::new();
    name.insert("name".into(), ObjectValue::String("test1".into()));
    hit.insert(
        "testunique/folder",
        "id2",
        name,
        IndexEntryProperty {
            id: "id".into(),
            property: "folders".into(),
        },
        None,
    )
    .expect("Ok");
    let mut name = LinkedHashMap::new();
    name.insert("name".into(), ObjectValue::String("test1".into()));
    hit.insert(
        "testunique/folder",
        "id3",
        name,
        IndexEntryProperty {
            id: "id".into(),
            property: "folders".into(),
        },
        None,
    )
    .expect("Ok");

    assert_eq!(
        hit.get_validation_errors("id3", "name").unwrap(),
        &vec![ValidationError::warning("A".into(), None)]
    );
}
