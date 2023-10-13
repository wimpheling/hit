use std::rc::Rc;

use linked_hash_map::LinkedHashMap;

use crate::{
    duplicate_hit, export, test_kernel::create_test_kernel, Hit, IndexEntryProperty, ObjectValue, Reference,
};

use super::test_copy_object::create_hit_with_subobjects;

#[test]
fn it_should_import_a_hit_instance() {
    let mut hit = create_hit_with_subobjects();
    let kernel = Rc::new(create_test_kernel());
    let mut hit_to_import = Hit::new("hid2", "test/test", kernel).unwrap();
    hit_to_import
        .insert(
            "test/test",
            "hid4",
            LinkedHashMap::new(),
            IndexEntryProperty {
                id: "hid2".into(),
                property: "sub_items".into(),
            },
            None,
        )
        .unwrap();
    hit.import(
        hit_to_import,
        IndexEntryProperty {
            id: "id".to_string(),
            property: "sub_items".to_string(),
        },
    )
    .unwrap();

    let entry = hit.get("hid4").unwrap();
    assert_eq!(entry.get_parent_id(), Some("hid2".to_string()));
    assert_eq!(entry.get_parent_property(), Some("sub_items".to_string()));

    let entry = hit.get("hid2").unwrap();
    assert_eq!(entry.get_parent_id(), Some("id".to_string()));
    assert_eq!(entry.get_parent_property(), Some("sub_items".to_string()));

    let value = hit.get_value("id", "sub_items").unwrap();
    assert_eq!(value, ObjectValue::VecSubObjects(vec![
        Reference { id: "id2".into() },
        Reference { id: "hid2".into() },
    ]));

    let refs = hit.find_references_recursive("hid2").unwrap();
    println!("refs: {:?}", refs);
}
