use linked_hash_map::LinkedHashMap;

use crate::test_kernel::create_test_kernel;
use crate::Hit;
use crate::IndexEntryProperty;
use crate::ObjectValue;
use crate::Reference;

use std::rc::Rc;

#[test]
fn it_should_move_a_reference() {
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
        None,
    )
    .expect("Error");
    hit.insert_reference(
        "id",
        IndexEntryProperty {
            id: "id".into(),
            property: "references".into(),
        },
        None,
    )
    .expect("Error");
    hit.move_reference(
        "id",
        IndexEntryProperty {
            id: "id".into(),
            property: "references".into(),
        },
        Some("id2".to_string()),
    )
    .expect("Error");
    assert_eq!(
        hit.get_value("id", "references").unwrap(),
        ObjectValue::VecReference(vec![
            Reference { id: "id".into() },
            Reference { id: "id2".into() }
        ])
    )
}
