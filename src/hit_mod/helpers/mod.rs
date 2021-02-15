mod can_create_object;
mod hit_copy_helper;
mod hit_move_helper;

pub use can_create_object::get_all_permissions;
pub use can_create_object::get_all_targets;
pub use can_create_object::ObjectPermissions;
pub use hit_copy_helper::copy_object;
pub use hit_move_helper::can_move_object;
