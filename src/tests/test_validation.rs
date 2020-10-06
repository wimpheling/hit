use crate::{
    field_types::*, modele, validators::Validator, Hit, IndexEntryProperty, ObjectValue, Reference,
};
use std::{cell::RefCell, collections::HashMap, rc::Rc};

use crate::{HitEntry, HitError, Kernel, Model, Plugins};

pub struct TestEventsKernel {
    model: Rc<Model>,
}

impl Kernel<Rc<Model>, HitEntry> for TestEventsKernel {
    fn get_model(&self, _name: &str) -> Result<Rc<Model>, HitError> {
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
#[derive(thiserror::Error, Clone, Debug, PartialEq)]
enum TestCustomError {
    #[error("TEST_ERROR")]
    MyTestError(),
    #[error("TEST_CONTEXT_ERROR")]
    MyTestContextError(),
}

struct IsNotId2Validator {}

impl Validator<Reference> for IsNotId2Validator {
    fn validate(
        &self,
        value: &Reference,
        _context: &crate::validators::ValidatorContext,
    ) -> Result<(), Vec<anyhow::Error>> {
        if value.id == "id2" {
            return Err(vec![anyhow::anyhow!(TestCustomError::MyTestError())]);
        }
        Ok(())
    }
}
struct OnlyIdInReferenceValidator {}

impl Validator<Reference> for OnlyIdInReferenceValidator {
    fn validate(
        &self,
        _value: &Reference,
        context: &crate::validators::ValidatorContext,
    ) -> Result<(), Vec<anyhow::Error>> {
        if context.id == "id" && context.property == "reference" {
            return Err(vec![anyhow::anyhow!(TestCustomError::MyTestContextError())]);
        }
        Ok(())
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
            authorized_models: vec!["test/test".to_string()],
            validators: vec![Rc::new(RefCell::new(IsNotId2Validator {}))],
        },
        "reference": FieldTypeReference {
            authorized_models: vec!["test/test".to_string()],
            validators: vec![Rc::new(RefCell::new(IsNotId2Validator {}))],
        },
        "reference2": FieldTypeReference {
            authorized_models: vec!["test/test".to_string()],
            validators: vec![Rc::new(RefCell::new(OnlyIdInReferenceValidator {}))],
        }
    )
}

fn create_test_events_kernel() -> TestEventsKernel {
    TestEventsKernel {
        model: create_test_events_model(),
    }
}

fn get_test_hit() -> Hit {
    let mut hit = Hit::new(
        "id".into(),
        "test/test".into(),
        Rc::new(create_test_events_kernel()),
    )
    .expect("Error creating instance");

    hit.insert(
        "test/test".into(),
        "id2".into(),
        HashMap::new(),
        IndexEntryProperty {
            id: "id".into(),
            property: "subitems".into(),
        },
        None,
    )
    .expect("Error");
    hit
}

// #[test]
fn it_should_return_an_error_on_reference_arrays_when_validator_detects_it() {
    let mut hit = get_test_hit();
    hit.insert_reference(
        "id",
        IndexEntryProperty {
            id: "id".into(),
            property: "references".into(),
        },
    )
    .expect("Error");

    assert!(hit.errors.get("id", "references").is_none());

    hit.insert_reference(
        "id2",
        IndexEntryProperty {
            id: "id".into(),
            property: "references".into(),
        },
    )
    .expect("Error");
    assert_eq!(hit.errors.get("id", "references").unwrap(), &vec!["a"]);
}

#[test]
fn it_should_return_an_error_on_set_when_validator_detects_it() {
    let mut hit = get_test_hit();
    hit.set(
        "id",
        "reference",
        ObjectValue::Reference(Reference { id: "id2".into() }),
    )
    .expect("Error");
    assert_eq!(
        hit.errors.get("id", "reference").unwrap(),
        &vec!["TEST_ERROR"]
    );
    hit.set(
        "id",
        "reference",
        ObjectValue::Reference(Reference { id: "id".into() }),
    )
    .expect("Error");
    assert!(hit.errors.get("id", "reference").is_none());
}
