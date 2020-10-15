use crate::model::Model;
use crate::plugins::Plugins;
use crate::HitError;
use std::rc::Rc;

pub trait Kernel {
    fn get_model(&self, name: &str) -> Result<Rc<Model>, HitError>;
    fn get_instantiable_models(&self) -> Vec<&Model>;
    fn get_plugins(&self) -> Plugins;
    fn get_models(&self) -> Vec<String>;
}
