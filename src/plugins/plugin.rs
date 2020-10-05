use crate::object_data::ObjectValues;
use crate::HitError;
use crate::{index::IndexEntryProperty, ObjectValue};

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
    fn on_before_delete_entry(&mut self, entry: &Entry) -> Result<(), HitError>;
    fn on_after_delete_entry(&mut self, entry: &Entry) -> Result<(), HitError>;
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

    fn on_before_set_value(
        &mut self,
        property: IndexEntryProperty,
        value: &ObjectValue,
        old_value: &Option<ObjectValue>,
    );
    fn on_after_set_value(
        &mut self,
        property: IndexEntryProperty,
        value: &ObjectValue,
        old_value: &Option<ObjectValue>,
    );
    fn on_before_move_subobject(&mut self);
    fn on_after_move_subobject(&mut self);
}
