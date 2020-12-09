use linked_hash_map::LinkedHashMap;

use crate::{
    field_types::*, modele, validators::Validator, Hit, IndexEntryProperty, ObjectValue, Reference,
    ValidationError, ValidationErrorLevel,
};
use std::{cell::RefCell, rc::Rc};

use crate::{HitError, Kernel, Model, Plugins};

pub struct TestEventsKernel {
    model: Rc<Model>,
}

impl Kernel for TestEventsKernel {
    fn get_model(&self, _name: &str) -> Result<Rc<Model>, HitError> {
        return Ok(self.model.clone());
    }

    fn get_instantiable_models(&self) -> Vec<&Model> {
        return vec![&self.model];
    }

    fn get_plugins(&self) -> Plugins {
        Plugins::new()
    }

    fn get_models(&self) -> Vec<String> {
        vec!["test/test".to_string()]
    }
}
struct IsNotId2Validator {}

impl Validator<Reference> for IsNotId2Validator {
    fn validate(
        &self,
        value: &Reference,
        _context: &crate::validators::ValidatorContext,
    ) -> Result<Option<Vec<ValidationError>>, HitError> {
        if value.id == "id2" {
            return Ok(Some(vec![ValidationError {
                key: "TEST_ERROR".into(),
                level: ValidationErrorLevel::Error,
                arguments: Some(vec![("id".into(), value.id.clone())].into_iter().collect()),
            }]));
        }
        Ok(None)
    }

    fn on_kernel_init(&mut self, field_name: &str, model_name: &str) -> Result<(), HitError> {
        Ok(())
    }
}
struct OnlyIdInReferenceValidator {}

impl Validator<Reference> for OnlyIdInReferenceValidator {
    fn validate(
        &self,
        _value: &Reference,
        context: &crate::validators::ValidatorContext,
    ) -> Result<Option<Vec<ValidationError>>, HitError> {
        if context.id == "id" && context.property == "reference" {
            return Ok(Some(vec![ValidationError {
                key: "TEST__CUSTOM_ERROR".into(),
                level: ValidationErrorLevel::Error,
                arguments: None,
            }]));
        }
        Ok(None)
    }

    fn on_kernel_init(&mut self, field_name: &str, model_name: &str) -> Result<(), HitError> {
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
            validators: vec![Box::new(IsNotId2Validator {})],
        },
        "reference": FieldTypeReference {
            authorized_models: vec!["test/test".to_string()],
            validators: vec![Box::new(IsNotId2Validator {})],
        },
        "reference2": FieldTypeReference {
            authorized_models: vec!["test/test".to_string()],
            validators: vec![Box::new(OnlyIdInReferenceValidator {})],
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
        LinkedHashMap::new(),
        IndexEntryProperty {
            id: "id".into(),
            property: "subitems".into(),
        },
        None,
    )
    .expect("Error");
    hit
}

// TODO
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
    assert_eq!(
        hit.errors.get("id", "references").unwrap(),
        &vec![ValidationError {
            key: "TEST_ERROR".into(),
            level: ValidationErrorLevel::Error,
            arguments: Some(vec![("id".into(), "id2".into())].into_iter().collect()),
        }]
    );
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
        &vec![ValidationError {
            key: "TEST_ERROR".into(),
            level: ValidationErrorLevel::Error,
            arguments: Some(vec![("id".into(), "id2".into())].into_iter().collect()),
        }]
    );
    hit.set(
        "id",
        "reference",
        ObjectValue::Reference(Reference { id: "id".into() }),
    )
    .expect("Error");
    assert!(hit.errors.get("id", "reference").is_none());
}
