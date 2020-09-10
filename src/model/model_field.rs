use crate::model::validators::ValidatorContext;
use crate::model::Model;
use crate::object_data::ObjectValue;
use crate::HitError;
use mopa;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::rc::Rc;

pub trait ModelField: mopa::Any {
    fn get_name(&self) -> String;
    fn validate(
        &self,
        value: &ObjectValue,
        context: &ValidatorContext,
    ) -> Result<(), Vec<HitError>>;
    fn accepts(&self, value: &ObjectValue, context: &ValidatorContext) -> bool;
    fn accepts_model(&self, model: &Model) -> bool;
}
mopafy!(ModelField);

pub type ModelFieldRef = Rc<RefCell<dyn ModelField>>;

pub type Fields = BTreeMap<String, ModelFieldRef>;
