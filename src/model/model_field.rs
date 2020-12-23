use crate::model::Model;
use crate::object_data::ObjectValue;
use crate::HitError;
use crate::{errors::ValidationError, model::validators::ValidatorContext};
use linked_hash_map::LinkedHashMap;
use mopa;
use std::cell::RefCell;
use std::rc::Rc;

pub trait ModelField: mopa::Any {
    fn get_name(&self) -> String;
    fn validate(
        &self,
        value: &ObjectValue,
        context: &ValidatorContext,
    ) -> Result<Option<Vec<ValidationError>>, HitError>;
    fn accepts_for_set(&self, value: &ObjectValue, context: &ValidatorContext) -> bool;
    fn accepts_model(&self, model: &Model) -> bool;
    fn is_vec_reference(&self) -> bool;
    fn is_vec_subobject(&self) -> bool;
    fn on_kernel_init(&mut self, model_name: &str) -> Result<(), HitError>;
}
mopafy!(ModelField);

pub type ModelFieldRef = Rc<RefCell<dyn ModelField>>;

pub type Fields = LinkedHashMap<String, ModelFieldRef>;
