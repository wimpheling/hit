use linked_hash_map::LinkedHashMap;

use crate::test_kernel::create_test_kernel;
use crate::Hit;
use crate::IndexEntryProperty;
use crate::ObjectValue;
use crate::Reference;

use std::rc::Rc;

pub(super) fn create_hit_with_subobjects() -> Hit {
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

    // A reference of an object that is not a sub item of id3
    hit.set(
        "id4",
        "reference",
        ObjectValue::Reference(Reference { id: "id2".into() }),
    )
    .expect("Error");
    hit.insert_reference(
        "id2",
        IndexEntryProperty {
            id: "id4".into(),
            property: "references".into(),
        },
        None,
    )
    .expect("Error");
    // A reference of an object that is a sub item of id3
    hit.set(
        "id5",
        "reference",
        ObjectValue::Reference(Reference { id: "id4".into() }),
    )
    .expect("Error");
    hit.insert_reference(
        "id3",
        IndexEntryProperty {
            id: "id5".into(),
            property: "references".into(),
        },
        None,
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

#[test]
fn it_should_copy_an_object_and_its_subobjects() {
    let mut hit = create_hit_with_subobjects();
    let id_of_id3_clone = hit
        .copy_object(
            "id3".to_string(),
            IndexEntryProperty {
                id: "id2".into(),
                property: "sub_items".into(),
            },
            None,
        )
        .expect("Error");
    let sub_items = hit.get_value("id2", "sub_items").expect("Error");
    let sub_items_array_obj = match sub_items {
        ObjectValue::VecSubObjects(sub_items) => Ok(sub_items),
        _ => Err(()),
    }
    .expect("Error");
    assert_eq!(sub_items_array_obj.len(), 2);
    assert_eq!(
        sub_items_array_obj.get(1).expect("Error").clone(),
        Reference {
            id: id_of_id3_clone.clone()
        }
    );

    // the clone of id3 shoudl have two sub_items
    let sub_items_of_id3_clone = hit.get_value(&id_of_id3_clone, "sub_items").expect("Error");
    let sub_items_of_id3_clone = match sub_items_of_id3_clone {
        ObjectValue::VecSubObjects(sub_items_of_id3_clone) => Ok(sub_items_of_id3_clone),
        _ => Err(()),
    }
    .expect("Error");
    assert_eq!(sub_items_of_id3_clone.len(), 2);
    assert_ne!(sub_items_of_id3_clone.get(0).expect("Error").id, "id4");
    let id_of_id5_clone = sub_items_of_id3_clone.get(1).expect("Error").id.clone();
    assert_ne!(id_of_id5_clone.clone(), "id5");

    assert_eq!(
        hit.get_value(&id_of_id5_clone, "name").expect("Error"),
        ObjectValue::String("hello".into())
    );
    assert_eq!(
        hit.get_value(&id_of_id5_clone, "age").expect("Error"),
        ObjectValue::F32(1.5)
    );
}

#[test]
fn it_should_copy_an_object_and_inner_references() {
    let mut hit = create_hit_with_subobjects();
    let id_of_id3_clone = hit
        .copy_object(
            "id3".to_string(),
            IndexEntryProperty {
                id: "id2".into(),
                property: "sub_items".into(),
            },
            None,
        )
        .expect("Error");
    let sub_items_of_id3_clone = hit.get_value(&id_of_id3_clone, "sub_items").expect("Error");
    let sub_items_of_id3_clone = match sub_items_of_id3_clone {
        ObjectValue::VecSubObjects(sub_items_of_id3_clone) => Ok(sub_items_of_id3_clone),
        _ => Err(()),
    }
    .expect("Error");
    let id_of_id4_clone = sub_items_of_id3_clone.get(0).expect("Error").id.clone();
    assert_eq!(
        hit.get_value(&id_of_id4_clone, "reference"),
        Some(ObjectValue::Reference(Reference { id: "id2".into() }))
    );
    assert_eq!(
        hit.get_value(&id_of_id4_clone, "references"),
        Some(ObjectValue::VecReference(vec![Reference {
            id: "id2".into()
        }]))
    );

    let id_of_id5_clone = sub_items_of_id3_clone.get(1).expect("Error").id.clone();
    assert_eq!(
        hit.get_value(&id_of_id5_clone, "reference"),
        Some(ObjectValue::Reference(Reference {
            id: id_of_id4_clone
        }))
    );
    assert_eq!(
        hit.get_value(&id_of_id5_clone, "references"),
        Some(ObjectValue::VecReference(vec![Reference {
            id: id_of_id3_clone
        }]))
    );
}
