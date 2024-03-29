#![feature(hash_set_entry)]
#[macro_use]
extern crate mopa;

mod errors;
pub mod events;
mod hit_mod;
mod index;
mod json;
mod kernel;
mod macros;
mod model;
mod object_data;
mod plugins;
mod import;
#[cfg(test)]
mod test_kernel;
#[cfg(test)]
mod tests;

pub mod utils;

pub mod prelude;

pub use index::IndexEntryProperty;

pub use kernel::Kernel;
pub use model::*;

pub use object_data::Id;
pub use object_data::ObjectValue;
pub use object_data::ObjectValues;
pub use object_data::Reference;

pub use hit_mod::{Hit, HitEntry, HitKernel};

pub use events::FieldListener;
pub use events::FieldListenerRef;

pub use json::export::export;
pub use json::import::import;
pub use json::import::import_from_string;

pub use plugins::{
    AfterImportPlugin, DeletePlugin, InitEntryPlugin, ModelTypeIndexer, Plugin, Plugins,
    ReferencePlugin,
};

pub use errors::HitError;
pub use errors::ValidationError;
pub use errors::ValidationErrorLevel;
pub use model::helpers;
pub use linked_hash_map::LinkedHashMap;
pub use import::*;