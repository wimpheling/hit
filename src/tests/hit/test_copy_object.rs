use linked_hash_map::LinkedHashMap;

use crate::test_kernel::create_test_kernel;
use crate::Hit;
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
    hit.set("id5", "name", ObjectValue::String("hello".into()))
        .expect("Error");
    hit.set("id5", "age", ObjectValue::F32(1.5)).expect("Error");
    hit.insert_reference(
        "id2",
        IndexEntryProperty {
            id: "id".into(),
            property: "references".into(),
        },
    )
    .expect("Error");

    // A reference of an object that is not a sub item of id4
    hit.set(
        "id4",
        "reference",
        ObjectValue::Reference(Reference { id: "id5".into() }),
    )
    .expect("Error");
    return hit;
}

#[test]
fn it_should_copy_an_object() {
    let mut hit = create_hit_with_subobjects();
    let id = hit
        .copy_object(
            "id4".to_string(),
            IndexEntryProperty {
                id: "id".into(),
                property: "sub_items".into(),
            },
            None,
        )
        .expect("Error");

    // use before_id
    let id_of_id5_clone = hit
        .copy_object(
            "id5".into(),
            IndexEntryProperty {
                id: "id".into(),
                property: "sub_items".into(),
            },
            Some(id.clone()),
        )
        .expect("Error");

    let parent_sub_items = hit.get_value("id", "sub_items").expect("Error");
    let expected_sub_items = ObjectValue::VecSubObjects(vec![
        Reference { id: "id2".into() },
        Reference {
            id: id_of_id5_clone.clone(),
        },
        Reference { id: id.clone() },
    ]);
    assert_eq!(parent_sub_items, expected_sub_items);

    assert_eq!(
        hit.get_value(&id_of_id5_clone, "name").expect("Error"),
        ObjectValue::String("hello".into())
    );
    assert_eq!(
        hit.get_value(&id_of_id5_clone, "age").expect("Error"),
        ObjectValue::F32(1.5)
    );
}
