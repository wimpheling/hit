use crate::test_kernel::create_test_kernel;
use crate::Hit;
use crate::ObjectValue;
use crate::ObjectValues;
use std::collections::HashMap;
use std::rc::Rc;

#[test]
fn it_should_create_a_new_hit_instance() {
    let kernel = Rc::new(create_test_kernel());
    let hit = Hit::new("id", "test/test", kernel).unwrap();
    assert!(hit.get("id").is_some());
    assert_eq!(hit.get_main_object_id(), "id");
}
#[test]
fn it_should_create_a_new_hit_instance_with_values() {
    let kernel = Rc::new(create_test_kernel());
    let mut values: ObjectValues = HashMap::new();
    values.insert("name".into(), ObjectValue::String("my_hit".into()));
    let hit = Hit::new_with_values("id", kernel, values, "test/test").unwrap();
    assert!(hit.get("id").is_some());
    assert_eq!(hit.get_main_object_id(), "id");
    assert_eq!(
        hit.get_value("id", "name").unwrap(),
        ObjectValue::String("my_hit".into())
    );
}
