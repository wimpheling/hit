#[macro_use]
extern crate mopa;

mod errors;
mod events;
mod hit_mod;
mod index;
mod json;
mod kernel;
mod macros;
mod model;
mod object_data;
mod plugins;
#[cfg(test)]
mod test_kernel;
#[cfg(test)]
mod tests;

// TODO
// mod utils;

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

pub use plugins::{DeletePlugin, InitPlugin, ModelTypeIndexer, Plugin, Plugins};

pub use errors::HitError;
pub use model::helpers;
