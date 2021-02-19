mod model_type_indexer;
mod plugin;

pub use model_type_indexer::ModelTypeIndexer;
pub use plugin::{AfterImportPlugin, DeletePlugin, InitEntryPlugin, Plugin, ReferencePlugin};
use std::cell::RefCell;
use std::clone::Clone;
use std::rc::Rc;

#[derive(Clone)]
pub struct Plugins {
    pub init_plugins: Vec<Rc<RefCell<dyn InitEntryPlugin>>>,
    pub after_import_plugins: Vec<Rc<RefCell<dyn AfterImportPlugin>>>,
    pub plugins: Vec<Rc<RefCell<dyn Plugin>>>,
    pub delete_plugins: Vec<Rc<RefCell<dyn DeletePlugin>>>,
    pub reference_plugins: Vec<Rc<RefCell<dyn ReferencePlugin>>>,
}

impl Plugins {
    pub fn new() -> Self {
        Plugins {
            init_plugins: vec![],
            after_import_plugins: vec![],
            plugins: vec![],
            delete_plugins: vec![],
            reference_plugins: vec![],
        }
    }
}
