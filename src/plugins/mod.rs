mod model_type_indexer;
mod plugin;

pub use model_type_indexer::ModelTypeIndexer;
pub use plugin::{AfterImportPlugin, DeletePlugin, InitEntryPlugin, Plugin};
use std::cell::RefCell;
use std::clone::Clone;
use std::rc::Rc;

#[derive(Clone)]
pub struct Plugins<T, U> {
    pub init_plugins: Vec<Rc<RefCell<dyn InitEntryPlugin<T, U>>>>,
    pub after_import_plugins: Vec<Rc<RefCell<dyn AfterImportPlugin<T, U>>>>,
    pub plugins: Vec<Rc<RefCell<dyn Plugin<T, U>>>>,
    pub delete_plugins: Vec<Rc<RefCell<dyn DeletePlugin<U>>>>,
}

impl<T, U> Plugins<T, U> {
    pub fn new() -> Self {
        Plugins {
            init_plugins: vec![],
            after_import_plugins: vec![],
            plugins: vec![],
            delete_plugins: vec![],
        }
    }
}
