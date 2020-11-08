use std::rc::Rc;

use crate::{index::IndexEntryProperty, HitEntry, ObjectValue};
use crate::{object_data::ObjectValues, Hit};
use crate::{HitError, Model};

pub trait InitEntryPlugin {
    fn on_init_add_entry(
        &mut self,
        model: Rc<Model>,
        id: &str,
        data: ObjectValues,
        parent: Option<IndexEntryProperty>,
    );
}

pub trait AfterImportPlugin {
    fn after_import(&mut self, hit: &Hit);
}

pub trait InitEntryAfterIndexPlugin {
    fn for_each_entry(
        &mut self,
        model: Rc<Model>,
        id: &str,
        data: ObjectValues,
        parent: Option<IndexEntryProperty>,
    );
}

pub trait DeletePlugin {
    fn on_before_delete_entry(&mut self, entry: &HitEntry, instance: &Hit) -> Result<(), HitError>;
    fn on_after_delete_entry(&mut self, entry: &HitEntry, instance: &Hit) -> Result<(), HitError>;
}

pub trait Plugin {
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
        instance: &mut Hit,
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
        instance: &mut Hit,
    );
    fn on_before_move_subobject(
        &mut self,
        id: &str,
        target: IndexEntryProperty,
        before_id: Option<String>,
        instance: &Hit,
    ) -> Result<(), HitError>;
    fn on_after_move_subobject(
        &mut self,
        id: &str,
        target: IndexEntryProperty,
        original_parent: IndexEntryProperty,
        before_id: Option<String>,
        instance: &mut Hit,
    ) -> Result<(), HitError>;
}
