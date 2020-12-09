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
fn it_should_detect_not_unique_values_on_insert() {
    let kernel = Rc::new(create_test_unique_in_parent_kernel());
    let mut hit = Hit::new("id", "testunique/project", kernel.clone()).expect("Error");
    let mut name = LinkedHashMap::new();
    name.insert("name".into(), ObjectValue::String("identical".into()));
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
    name.insert("name".into(), ObjectValue::String("identical".into()));
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
        &vec![ValidationError::warning("UNIQUE_IN_PARENT".into(), None)]
    );
    assert_eq!(
        hit.get_validation_errors("id2", "name").unwrap(),
        &vec![ValidationError::warning("UNIQUE_IN_PARENT".into(), None)]
    );
}

#[test]
fn it_should_detect_not_unique_values_when_setting_it() {
    let kernel = Rc::new(create_test_unique_in_parent_kernel());
    let mut hit = Hit::new("id", "testunique/project", kernel.clone()).expect("Error");
    let mut name = LinkedHashMap::new();
    name.insert("name".into(), ObjectValue::String("identical".into()));
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
    name.insert("name".into(), ObjectValue::String("not_identical".into()));
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

    assert!(hit.get_validation_errors("id3", "name").is_none());
    assert!(hit.get_validation_errors("id2", "name").is_none());

    hit.set("id3", "name", ObjectValue::String("identical".into()))
        .expect("Error");

    assert_eq!(
        hit.get_validation_errors("id3", "name").unwrap(),
        &vec![ValidationError::warning("UNIQUE_IN_PARENT".into(), None)]
    );
    assert_eq!(
        hit.get_validation_errors("id2", "name").unwrap(),
        &vec![ValidationError::warning("UNIQUE_IN_PARENT".into(), None)]
    );
}

#[test]
fn it_should_remove_unique_errors_when_setting_to_correct_values() {
    let kernel = Rc::new(create_test_unique_in_parent_kernel());
    let mut hit = Hit::new("id", "testunique/project", kernel.clone()).expect("Error");
    let mut name = LinkedHashMap::new();
    name.insert("name".into(), ObjectValue::String("identical".into()));
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
    name.insert("name".into(), ObjectValue::String("identical".into()));
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

    hit.set("id3", "name", ObjectValue::String("not_identical".into()))
        .expect("Error");

    assert!(hit.get_validation_errors("id3", "name").is_none());
    assert!(hit.get_validation_errors("id2", "name").is_none());
}

#[test]
fn it_should_remove_unique_errors_when_deleting_an_entry() {
    let kernel = Rc::new(create_test_unique_in_parent_kernel());
    let mut hit = Hit::new("id", "testunique/project", kernel.clone()).expect("Error");
    let mut name = LinkedHashMap::new();
    name.insert("name".into(), ObjectValue::String("identical".into()));
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
    name.insert("name".into(), ObjectValue::String("identical".into()));
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

    hit.remove_object("id3").expect("Error");

    assert!(hit.get_validation_errors("id2", "name").is_none());
}

#[test]
fn it_should_remove_unique_errors_when_moving_an_entry() {
    let kernel = Rc::new(create_test_unique_in_parent_kernel());
    let mut hit = Hit::new("id", "testunique/project", kernel.clone()).expect("Error");
    let mut name = LinkedHashMap::new();
    name.insert("name".into(), ObjectValue::String("identical".into()));
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
    name.insert("name".into(), ObjectValue::String("identical".into()));
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

    hit.move_object(
        "id3",
        IndexEntryProperty {
            id: "id2".into(),
            property: "folders".into(),
        },
        None,
    )
    .expect("Error");

    assert!(hit.get_validation_errors("id3", "name").is_none());
    assert!(hit.get_validation_errors("id2", "name").is_none());
}
