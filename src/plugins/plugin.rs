use std::rc::Rc;

use crate::{index::IndexEntryProperty, ObjectValue};
use crate::{object_data::ObjectValues, Hit};
use crate::{HitError, Model};

pub trait InitEntryPlugin<Entry> {
    fn on_init_add_entry(
        &mut self,
        model: Rc<Model>,
        id: &str,
        data: ObjectValues,
        parent: Option<IndexEntryProperty>,
    );
}

pub trait AfterImportPlugin<Entry> {
    fn after_import(&mut self, hit: &Hit);
}

pub trait InitEntryAfterIndexPlugin<Entry> {
    fn for_each_entry(
        &mut self,
        model: Rc<Model>,
        id: &str,
        data: ObjectValues,
        parent: Option<IndexEntryProperty>,
    );
}

pub trait DeletePlugin<Entry> {
    fn on_before_delete_entry(&mut self, entry: &Entry) -> Result<(), HitError>;
    fn on_after_delete_entry(&mut self, entry: &Entry) -> Result<(), HitError>;
}

pub trait Plugin<Entry> {
    fn on_before_add_entry(
        &mut self,
        model: Rc<Model>,
        id: &str,
        data: ObjectValues,
        parent: IndexEntryProperty,
        instance: &Hit,
    );
    fn on_after_add_entry(
        &mut self,
        model: Rc<Model>,
        id: &str,
        data: ObjectValues,
        parent: IndexEntryProperty,
        instance: &Hit,
    );

    fn on_before_set_value(
        &mut self,
        property: IndexEntryProperty,
        value: &ObjectValue,
        old_value: &Option<ObjectValue>,
        instance: &Hit,
    );
    fn on_after_set_value(
        &mut self,
        property: IndexEntryProperty,
        value: &ObjectValue,
        old_value: &Option<ObjectValue>,
        instance: &Hit,
    );
    fn on_before_move_subobject(&mut self);
    fn on_after_move_subobject(&mut self);
}
