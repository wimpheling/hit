use crate::{field_types::*, modele, FieldListener, Hit, ObjectValue, Reference};
use std::{cell::RefCell, rc::Rc};

use crate::{HitEntry, HitError, Kernel, Model, Plugins};

pub struct TestEventsKernel {
    model: Rc<Model>,
}

impl Kernel<Rc<Model>, HitEntry> for TestEventsKernel {
    fn get_model(&self, name: &str) -> Result<Rc<Model>, HitError> {
        return Ok(self.model.clone());
    }

    fn get_instantiable_models(&self) -> Vec<&Model> {
        return vec![&self.model];
    }

    fn get_plugins(&self) -> Plugins<Rc<Model>, HitEntry> {
        Plugins::new()
    }

    fn get_models(&self) -> Vec<String> {
        vec!["test/test".to_string()]
    }
}

fn create_test_events_model() -> Rc<Model> {
    modele!("test/test", "Filesystem" =>
        "name": FieldTypeString {
            required: true
        },
        "subitems": FieldTypeSubobjectArray {
            authorized_models: vec!["test/test".to_string()]
        },
        "references": FieldTypeReferenceArray {
            authorized_models: vec!["test/test".to_string()]
        },
        "reference": FieldTypeReference {
            authorized_models: vec!["test/test".to_string()]
        }
    )
}

fn create_test_events_kernel() -> TestEventsKernel {
    TestEventsKernel {
        model: create_test_events_model(),
    }
}

struct TestPropertyListener {
    event_count: i32,
    id: String,
}

impl FieldListener<ObjectValue> for TestPropertyListener {
    fn on_update(&mut self, value: &ObjectValue) {
        self.event_count += 1;
    }

    fn get_unique_id(&self) -> &str {
        &self.id
    }
}

#[test]
fn it_should_send_an_event_when_property_is_set() {
    let listener = Rc::new(RefCell::new(TestPropertyListener {
        event_count: 0,
        id: "a".into(),
    }));
    let listener2 = Rc::new(RefCell::new(TestPropertyListener {
        event_count: 0,
        id: "b".into(),
    }));
    let mut hit = Hit::new(
        "id".into(),
        "test/test".into(),
        Rc::new(create_test_events_kernel()),
    )
    .expect("Error creating instance");
    hit.subscribe_field("id", "name", listener.clone())
        .expect("Error");
    hit.subscribe_field("id", "reference", listener2.clone())
        .expect("Error");
    hit.set("id", "name", ObjectValue::String("test".into()))
        .expect("Error");
    hit.set("id", "name", ObjectValue::String("test2".into()))
        .expect("Error");
    hit.set(
        "id",
        "reference",
        ObjectValue::Reference(Reference { id: "id".into() }),
    )
    .expect("Error");
    hit.set("id", "reference", ObjectValue::Null)
        .expect("Error");

    let listener = listener.borrow();
    let listener2 = listener2.borrow();
    assert_eq!(listener.event_count, 2);
    assert_eq!(listener2.event_count, 2);
}

#[test]
fn it_should_not_send_an_event_when_a_listener_is_unsubscribed() {
    let listener = Rc::new(RefCell::new(TestPropertyListener {
        event_count: 0,
        id: "a".into(),
    }));
    let mut hit = Hit::new(
        "id".into(),
        "test/test".into(),
        Rc::new(create_test_events_kernel()),
    )
    .expect("Error creating instance");
    hit.subscribe_field("id", "name", listener.clone())
        .expect("Error");
    hit.set("id", "name", ObjectValue::String("test".into()))
        .expect("Error");
    let listener = listener.borrow();
    assert_eq!(listener.event_count, 1);
    hit.unsubscribe_field("id", "name", "a").expect("Error");
    hit.set("id", "name", ObjectValue::String("test2".into()))
        .expect("Error");
    assert_eq!(listener.event_count, 1);
}

#[test]
fn it_should_return_an_error_when_a_non_existing_listener_is_unsubscribed() {
    let hit = Hit::new(
        "id".into(),
        "test/test".into(),
        Rc::new(create_test_events_kernel()),
    )
    .expect("Error creating instance");
    let error = hit.unsubscribe_field("id", "name", "a");
    assert_eq!(error.err().unwrap(), HitError::ListenerNotFound("a".into()));
}
