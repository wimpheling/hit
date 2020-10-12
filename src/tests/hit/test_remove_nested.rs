use linked_hash_map::LinkedHashMap;

use crate::test_kernel::create_test_kernel;
use crate::Hit;
use crate::HitError;
use crate::IndexEntryProperty;
use crate::ObjectValue;
use std::{collections::HashMap, rc::Rc};

fn create_hit_with_subobjects() -> Hit {
    let kernel = Rc::new(create_test_kernel());
    let mut hit = Hit::new("id", "test/test", kernel).unwrap();
    hit.insert(
        "test/test",
        "id2",
        LinkedHashMap::new(),
        IndexEntryProperty {
            id: "id".into(),
            property: "sub_items".into(),
        },
        None,
    )
    .expect("Error");
    hit.insert(
        "test/test",
        "id3",
        LinkedHashMap::new(),
        IndexEntryProperty {
            id: "id2".into(),
            property: "sub_items".into(),
        },
        None,
    )
    .expect("Error");
    hit.insert(
        "test/test",
        "id4",
        LinkedHashMap::new(),
        IndexEntryProperty {
            id: "id3".into(),
            property: "sub_items".into(),
        },
        None,
    )
    .expect("Error");
    return hit;
}

#[test]
fn it_should_remove_an_object_containing_other_objects() {
    let mut hit = create_hit_with_subobjects();
    hit.remove_object("id2").expect("Error");
    assert!(hit.get("id2").is_none());
    let parent_sub_items = hit.get_value("id", "sub_items").expect("Error");
    assert_eq!(parent_sub_items, ObjectValue::Null);
}

#[test]
fn it_should_return_an_error_when_a_nested_object_has_references() {
    let mut hit = create_hit_with_subobjects();

    hit.insert_reference(
        "id3",
        IndexEntryProperty {
            id: "id".into(),
            property: "references".into(),
        },
    )
    .expect("Error");

    let mut expected_error = HashMap::new();
    expected_error.insert(
        "id3".into(),
        vec![IndexEntryProperty {
            id: "id".into(),
            property: "references".into(),
        }],
    );

    assert_eq!(
        hit.remove_object("id2").err().unwrap(),
        HitError::CannotDeleteObjectWithReferences(expected_error)
    );
}

#[test]
fn it_should_not_return_an_error_when_a_nested_object_has_nested_references() {
    let mut hit = create_hit_with_subobjects();

    hit.insert_reference(
        "id3",
        IndexEntryProperty {
            id: "id2".into(),
            property: "references".into(),
        },
    )
    .expect("Error");

    hit.remove_object("id2").expect("Error");
    assert!(hit.get("id2").is_none());
    assert!(hit.get("id3").is_none());
    assert!(hit.get("id4").is_none());
    let parent_sub_items = hit.get_value("id", "sub_items").expect("Error");
    assert_eq!(parent_sub_items, ObjectValue::Null);
}
