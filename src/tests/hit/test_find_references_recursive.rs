use crate::test_kernel::create_test_kernel;
use crate::Hit;
use crate::HitError;
use crate::IndexEntryProperty;
use crate::ObjectValue;
use crate::Reference;
use std::collections::HashMap;
use std::rc::Rc;

fn create_hit_with_references() -> Hit {
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
    hit.insert_reference(
        "id2",
        IndexEntryProperty {
            id: "id4".into(),
            property: "references".into(),
        },
    )
    .expect("Error");
    hit.set(
        "id3",
        "reference",
        ObjectValue::Reference(Reference { id: "id2".into() }),
    )
    .expect("Error");
    return hit;
}

#[test]
fn it_should_find_all_references() {
    // TODO : more tests, redo the code etc
    let hit = create_hit_with_references();
    let references = hit.find_references_recursive("id2").expect("Error");

    let mut mock_object = HashMap::new();
    mock_object.insert(
        "id2".into(),
        vec![
            IndexEntryProperty {
                id: "id".into(),
                property: "references".into(),
            },
            IndexEntryProperty {
                id: "id4".into(),
                property: "references".into(),
            },
            IndexEntryProperty {
                id: "id3".into(),
                property: "reference".into(),
            },
        ],
    );
    assert_eq!(references, mock_object);
}

#[test]
fn it_should_find_all_references_after_removal() {
    // TODO : more tests, redo the code etc
    let mut hit = create_hit_with_references();
    hit.remove_reference(
        "id2".into(),
        IndexEntryProperty {
            id: "id4".into(),
            property: "references".into(),
        },
    )
    .expect("Error");
    let references = hit.find_references_recursive("id2").expect("Error");

    let mut mock_object = HashMap::new();
    mock_object.insert(
        "id2".into(),
        vec![
            IndexEntryProperty {
                id: "id".into(),
                property: "references".into(),
            },
            IndexEntryProperty {
                id: "id3".into(),
                property: "reference".into(),
            },
        ],
    );
    assert_eq!(references, mock_object);
}
