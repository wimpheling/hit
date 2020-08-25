use crate::model::Model;
use crate::model::ModelError;
use crate::plugins::Plugins;
use std::rc::Rc;

pub trait Kernel<T, U> {
    fn get_model(&self, name: &str) -> Result<Rc<Model>, ModelError>;
    fn get_instantiable_models(&self) -> Vec<&Model>;
    fn get_plugins(&self) -> Plugins<T, U>;
    fn get_models(&self) -> Vec<String>;
}
