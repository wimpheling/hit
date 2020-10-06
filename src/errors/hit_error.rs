use std::clone::Clone;
use thiserror::*;

#[derive(Error, Clone, Debug, PartialEq)]
pub enum HitError {
    #[error("There is no model entry for id `{0}`")]
    NoModelForId(String),
    #[error("Property `{0}` does not exist")]
    PropertyNotFound(String),
    #[error("Model `{0}` does not exist")]
    ModelDoesNotExist(String),
    #[error("Invalid move destination")]
    InvalidMoveDestination(),
    #[error("Invalid data type")]
    InvalidDataType(),
    #[error("Invalid date format")]
    InvalidDateFormat(),
    #[error("Invalid reference: `{0}`")]
    InvalidReference(String),
    // Todo : Is it useful for it to be different than invalid reference ?
    #[error("Invalid Reference in array")]
    InvalidReferenceInArray(),
    //TODO : should it be mandatory ?
    #[error("Invalid reference type")]
    InvalidReferenceType(),
    #[error("Invalid reference type in an array")]
    InvalidReferenceTypeInArray(),
    #[error("This field is required")]
    Required(),
    #[error("Model `{0}` is not allowed here")]
    ModelNotAllowed(String),
    #[error("ID not found: `{0}`")]
    IDNotFound(String),
    #[error("Listener Not Found: `{0}`")]
    ListenerNotFound(String),
    #[error("VALIDATION ERROR: TODO THIS SHOULD NOT BE A HITERROR")]
    ValidationError(),
    #[error("No parent (this is the main object)")]
    NoParent(),
    #[error("Invalid parent ID: `{0}`")]
    InvalidParentID(String),
    #[error("ID already exists in this document: `{0}`")]
    DuplicateID(String),
    #[error("An object cannot be its own parent: `{0}`")]
    CannotBeOwnParent(String),
    #[error("The root object cannot be moved")]
    CannotMoveRootObject(),
    #[error("An object cannot be moved to its child object")]
    CannotBeOwnChild(),
    #[error("Only scalar values can be set")]
    CanOnlySetScalarValues(),
    #[error("Only scalar values can be set in an inserted object")]
    CanOnlySetScalarValuesInInsertedObject(),
    #[error("An object cannot be inserted into a property that is not a subobject array")]
    CannotInsertObjectInThisDataType(),
    #[error("A reference cannot be inserted into a property that is not a reference array")]
    CannotInsertReferenceInThisDataType(),
    #[error("A reference cannot be deleted from a property that is not a reference array")]
    CannotRemoveReferenceFromThisDataType(),
    #[error("An object cannot be delete from a property that is not an object array")]
    CannotRemoveObjectFromThisDataType(),
    #[error("This object cannot be deleted because there are references to it: `{0}`")]
    CannotDeleteObjectWithReferences(String),
    #[error("The root object cannot be deleted")]
    CannotDeleteRootObject(),
    #[error("BeforeId is not present in this array: `{0}`")]
    InvalidBeforeId(String),
}
