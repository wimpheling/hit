use linked_hash_map::LinkedHashMap;

use crate::test_kernel::create_test_kernel;
use crate::Hit;
use crate::HitError;
use crate::IndexEntryProperty;
use crate::ObjectValue;
use crate::Reference;
use std::collections::HashMap;
use std::rc::Rc;

#[test]
fn it_should_insert_a_reference() {
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
    assert_eq!(
        hit.get_value("id", "references").unwrap(),
        ObjectValue::VecReference(vec![
            Reference { id: "id2".into() },
            Reference { id: "id".into() }
        ])
    )
}

#[test]
fn it_should_refuse_incorrect_references() {
    let kernel = Rc::new(create_test_kernel());
    let mut hit = Hit::new("id", "test/test", kernel).unwrap();
    let error = hit
        .insert_reference(
            "id2",
            IndexEntryProperty {
                id: "id".into(),
                property: "references".into(),
            },
        )
        .expect_err("Error");
    assert_eq!(error, HitError::InvalidReference("id2".into()));
}

#[test]
fn it_should_refuse_to_insert_references_in_other_fields() {
    let kernel = Rc::new(create_test_kernel());
    let mut hit = Hit::new("id", "test/test", kernel).unwrap();
    let error = hit
        .insert_reference(
            "id",
            IndexEntryProperty {
                id: "id".into(),
                property: "reference".into(),
            },
        )
        .expect_err("Error");
    assert_eq!(error, HitError::InvalidDataType());
    let error = hit
        .insert_reference(
            "id",
            IndexEntryProperty {
                id: "id".into(),
                property: "sub_items".into(),
            },
        )
        .expect_err("Error");
    assert_eq!(error, HitError::InvalidDataType());
    let error = hit
        .insert_reference(
            "id",
            IndexEntryProperty {
                id: "id".into(),
                property: "field_not_found".into(),
            },
        )
        .expect_err("Error");
    assert_eq!(error, HitError::PropertyNotFound("field_not_found".into()));
}
