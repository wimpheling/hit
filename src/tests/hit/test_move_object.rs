use linked_hash_map::LinkedHashMap;

use crate::test_kernel::create_test_kernel;
use crate::Hit;
use crate::HitError;
use crate::IndexEntryProperty;
use crate::ObjectValue;
use crate::Reference;

use std::rc::Rc;

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
    hit.insert(
        "test/test",
        "id5",
        LinkedHashMap::new(),
        IndexEntryProperty {
            id: "id3".into(),
            property: "sub_items".into(),
        },
        None,
    )
    .expect("Error");
    hit.insert_reference(
        "id2",
        IndexEntryProperty {
            id: "id".into(),
            property: "references".into(),
        },
        None,
    )
    .expect("Error");
    return hit;
}

#[test]
fn it_should_move_an_object() {
    let mut hit = create_hit_with_subobjects();
    hit.move_object(
        "id4",
        IndexEntryProperty {
            id: "id".into(),
            property: "sub_items".into(),
        },
        None,
    )
    .expect("Error");
    let parent_sub_items = hit.get_value("id", "sub_items").expect("Error");
    let expected_sub_items = ObjectValue::VecSubObjects(vec![
        Reference { id: "id2".into() },
        Reference { id: "id4".into() },
    ]);
    assert_eq!(parent_sub_items, expected_sub_items);

    // use before_id
    hit.move_object(
        "id5",
        IndexEntryProperty {
            id: "id".into(),
            property: "sub_items".into(),
        },
        Some("id4".into()),
    )
    .expect("Error");
    let parent_sub_items = hit.get_value("id", "sub_items").expect("Error");
    let expected_sub_items = ObjectValue::VecSubObjects(vec![
        Reference { id: "id2".into() },
        Reference { id: "id5".into() },
        Reference { id: "id4".into() },
    ]);
    assert_eq!(parent_sub_items, expected_sub_items);
}

#[test]
fn it_should_move_an_object_inside_the_same_parent() {
    let mut hit = create_hit_with_subobjects();

    // use before_id
    hit.move_object(
        "id5",
        IndexEntryProperty {
            id: "id3".into(),
            property: "sub_items".into(),
        },
        Some("id4".into()),
    )
    .expect("Error");
    let parent_sub_items = hit.get_value("id3", "sub_items").expect("Error");
    let expected_sub_items = ObjectValue::VecSubObjects(vec![
        Reference { id: "id5".into() },
        Reference { id: "id4".into() },
    ]);
    assert_eq!(parent_sub_items, expected_sub_items);
}

#[test]
fn it_should_return_an_error_when_id_is_invalid() {
    let mut hit = create_hit_with_subobjects();
    assert_eq!(
        hit.move_object(
            "id421",
            IndexEntryProperty {
                id: "id".into(),
                property: "sub_items".into(),
            },
            None,
        )
        .err()
        .unwrap(),
        HitError::IDNotFound("id421".into())
    );
}
#[test]
fn it_should_return_an_error_when_target_id_is_invalid() {
    let mut hit = create_hit_with_subobjects();
    assert_eq!(
        hit.move_object(
            "id4",
            IndexEntryProperty {
                id: "id222".into(),
                property: "sub_items".into(),
            },
            None,
        )
        .err()
        .unwrap(),
        HitError::IDNotFound("id222".into())
    );
}

#[test]
fn it_should_return_an_error_when_target_property_is_invalid() {
    let mut hit = create_hit_with_subobjects();
    assert_eq!(
        hit.move_object(
            "id4",
            IndexEntryProperty {
                id: "id".into(),
                property: "sub_items_wrong".into(),
            },
            None,
        )
        .err()
        .unwrap(),
        HitError::PropertyNotFound("sub_items_wrong".into())
    );
}
