use crate::test_kernel::create_test_kernel;
use crate::Hit;
use std::rc::Rc;

#[test]
fn it_should_check_if_a_key_exists() {
    let kernel = Rc::new(create_test_kernel());
    let hit = Hit::new("id", "test/test", kernel).unwrap();
    assert!(hit.contains_key("id"));
    assert!(!hit.contains_key("failure"));
}
