mod model_type_indexer;
mod plugin;

pub use model_type_indexer::ModelTypeIndexer;
pub use plugin::{AfterImportPlugin, DeletePlugin, InitEntryPlugin, Plugin};
use std::cell::RefCell;
use std::clone::Clone;
use std::rc::Rc;

#[derive(Clone)]
pub struct Plugins<U> {
    pub init_plugins: Vec<Rc<RefCell<dyn InitEntryPlugin<U>>>>,
    pub after_import_plugins: Vec<Rc<RefCell<dyn AfterImportPlugin<U>>>>,
    pub plugins: Vec<Rc<RefCell<dyn Plugin<U>>>>,
    pub delete_plugins: Vec<Rc<RefCell<dyn DeletePlugin<U>>>>,
}

impl<U> Plugins<U> {
    pub fn new() -> Self {
        Plugins {
            init_plugins: vec![],
            after_import_plugins: vec![],
            plugins: vec![],
            delete_plugins: vec![],
        }
    }
}
