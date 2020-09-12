use crate::test_kernel::create_test_kernel;
use crate::Hit;
use crate::HitError;
use crate::IndexEntryProperty;
use crate::ObjectValue;
use crate::Reference;
use std::collections::HashMap;
use std::rc::Rc;

fn create_hit_with_subobjects() -> Hit {
    let kernel = Rc::new(create_test_kernel());
    let mut hit = Hit::new("id", "test/test", kernel).unwrap();
    hit.insert(
        "test/test",
        "id2",
        HashMap::new(),
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
        HashMap::new(),
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
        HashMap::new(),
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
    )
    .expect("Error");
    return hit;
}

#[test]
fn it_should_remove_an_object() {
    let mut hit = create_hit_with_subobjects();
    let parent_sub_items = hit.get_value("id3", "sub_items").expect("Error");
    assert_eq!(
        parent_sub_items,
        ObjectValue::VecSubObjects(vec![Reference {
            id: "id4".to_string()
        }])
    );
    hit.remove_object("id4").expect("Error");
    assert!(hit.get("id4").is_none());
    let parent_sub_items = hit.get_value("id3", "sub_items").expect("Error");
    assert_eq!(parent_sub_items, ObjectValue::Null);
}

#[test]
fn it_should_return_an_error_when_id_is_invalid() {
    let mut hit = create_hit_with_subobjects();
    assert_eq!(
        hit.remove_object("id421").err().unwrap(),
        HitError::IDNotFound("id421".into())
    );
}

#[test]
fn it_should_return_an_error_when_object_has_references() {
    let mut hit = create_hit_with_subobjects();
    assert_eq!(
        hit.remove_object("id2").err().unwrap(),
        HitError::CannotDeleteObjectWithReferences("id2".into())
    );
}
