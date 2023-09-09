use crate::{ObjectValue, Reference};

#[test]
pub fn it_should_not_be_equal_when_values_are_not_equal() {
    let obj1 = ObjectValue::F32(1.0);
    let obj2 = ObjectValue::F32(2.0);
    assert_ne!(&obj1, &obj2);

    let obj1 = ObjectValue::String("hello".into());
    let obj2 = ObjectValue::String("world".into());
    assert_ne!(&obj1, &obj2);

    let obj1 = ObjectValue::Bool(true);
    let obj2 = ObjectValue::Bool(false);
    assert_ne!(&obj1, &obj2);

    let obj1 = ObjectValue::VecString(vec!["hello".into(), "world".into()]);
    let obj2 = ObjectValue::VecString(vec!["world".into(), "hello".into()]);
    assert_ne!(&obj1, &obj2);

    let obj1 = ObjectValue::VecReference(vec![
        Reference { id: "a".into() },
        Reference { id: "b".into() },
    ]);
    let obj2 = ObjectValue::VecReference(vec![]);
    assert_ne!(&obj1, &obj2);

    let obj1 = ObjectValue::VecSubObjects(vec![
        Reference { id: "a".into() },
        Reference { id: "b".into() },
    ]);
    let obj2 = ObjectValue::VecSubObjects(vec![Reference { id: "a".into() }]);
    assert_ne!(&obj1, &obj2);
}
