use crate::index::IndexEntryProperty;

use crate::object_data::ObjectValues;

pub trait InitPlugin<ExtraData, Entry> {
    fn on_init_add_entry(
        &mut self,
        extra_data: ExtraData,
        id: &str,
        data: ObjectValues,
        parent: Option<IndexEntryProperty>,
    );
}

pub trait DeletePlugin<Entry> {
    fn on_before_delete_entry(&mut self, entry: &Entry) -> Result<(), String>;
    fn on_after_delete_entry(&mut self, entry: &Entry) -> Result<(), String>;
}

pub trait Plugin<ExtraData, Entry> {
    fn on_before_add_entry(
        &mut self,
        extra_data: ExtraData,
        id: &str,
        data: ObjectValues,
        parent: IndexEntryProperty,
    );
    fn on_after_add_entry(
        &mut self,
        extra_data: ExtraData,
        id: &str,
        data: ObjectValues,
        parent: IndexEntryProperty,
    );

    fn on_before_set_value(&mut self);
    fn on_after_set_value(&mut self);
    fn on_before_move_subobject(&mut self);
    fn on_after_move_subobject(&mut self);
}
