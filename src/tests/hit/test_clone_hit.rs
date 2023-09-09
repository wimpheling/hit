use crate::{duplicate_hit, export, ObjectValue};

use super::test_copy_object::create_hit_with_subobjects;

#[test]
fn it_should_clone_a_hit_instance() {
    let mut hit = create_hit_with_subobjects();
    let duplicate = duplicate_hit(&mut hit).unwrap();

    let json1 = export(&hit);
    let json2 = export(&duplicate);
    assert_eq!(json1, json2);


}

#[test]
fn instances_should_be_distinct() {
    let mut hit = create_hit_with_subobjects();
    let duplicate = duplicate_hit(&mut hit).unwrap();

    hit.set("id5", "name", ObjectValue::String("ein zwei drei".into()))
    .expect("Error");
    assert_eq!(hit.get_value("id5", "name").unwrap(), ObjectValue::String("ein zwei drei".into()));
    assert_eq!(duplicate.get_value("id5", "name").unwrap(), ObjectValue::String("hello".into()));
}