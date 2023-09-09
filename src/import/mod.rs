pub(crate) mod generic_import;
mod patches;
mod diff;

pub use diff::{Patch,AddedEntry,PatchPropertyDifference, create_patch};
pub use patches::{import_from_patches, apply_patches, duplicate_hit};
