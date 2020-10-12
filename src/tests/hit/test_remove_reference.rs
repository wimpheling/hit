use linked_hash_map::LinkedHashMap;

use crate::test_kernel::create_test_kernel;
use crate::Hit;
use crate::HitError;
use crate::IndexEntryProperty;
use crate::ObjectValue;
use crate::Reference;

use std::rc::Rc;

fn create_hit_with_reference() -> Hit {
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
    hit.insert_reference(
        "id2",
        IndexEntryProperty {
            id: "id".into(),
            property: "references".into(),
        },
    )
    .expect("Error");
    hit.insert_reference(
        "id",
        IndexEntryProperty {
            id: "id".into(),
            property: "references".into(),
        },
    )
    .expect("Error");
    return hit;
}

#[test]
fn it_should_remove_a_reference() {
    let mut hit = create_hit_with_reference();
    hit.remove_reference(
        "id2",
        IndexEntryProperty {
            id: "id".into(),
            property: "references".into(),
        },
    )
    .expect("Error");
    assert_eq!(
        hit.get_value("id", "references").unwrap(),
        ObjectValue::VecReference(vec![Reference { id: "id".into() }])
    );
}
#[test]
fn it_should_throw_error_for_invalid_references() {
    let mut hit = create_hit_with_reference();
    assert_eq!(
        hit.remove_reference(
            "id24",
            IndexEntryProperty {
                id: "id".into(),
                property: "references".into(),
            },
        )
        .err()
        .unwrap(),
        HitError::IDNotFound("id24".into())
    );
}
#[test]
fn it_should_throw_error_for_invalid_properties() {
    let mut hit = create_hit_with_reference();
    assert_eq!(
        hit.remove_reference(
            "id2",
            IndexEntryProperty {
                id: "id".into(),
                property: "referencesa".into(),
            },
        )
        .err()
        .unwrap(),
        HitError::PropertyNotFound("referencesa".into())
    );
}
