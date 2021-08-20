use std::rc::Rc;

use crate::{index::IndexEntryProperty, HitEntry, ObjectValue};
use crate::{object_data::ObjectValues, Hit};
use crate::{HitError, Id, Model};

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
    fn after_import(&mut self, hit: &Hit) -> Result<(), HitError>;
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
    fn on_before_delete_entry(
        &mut self,
        entry: &HitEntry,
        instance: &mut Hit,
    ) -> Result<(), HitError>;
    fn on_after_delete_entry(
        &mut self,
        entry: &HitEntry,
        instance: &mut Hit,
    ) -> Result<(), HitError>;
}
pub trait ReferencePlugin {
    fn on_before_add_reference(
        &mut self,
        instance: &mut Hit,
        reference_id: &Id,
        target: &IndexEntryProperty,
        before_id: &Option<String>,
    ) -> Result<(), HitError>;
    fn on_after_add_reference(
        &mut self,
        instance: &mut Hit,
        reference_id: &Id,
        target: &IndexEntryProperty,
        before_id: &Option<String>,
    ) -> Result<(), HitError>;
    fn on_before_move_reference(
        &mut self,
        instance: &mut Hit,
        reference_id: &Id,
        target: &IndexEntryProperty,
        before_id: &Option<String>,
    ) -> Result<(), HitError>;
    fn on_after_move_reference(
        &mut self,
        instance: &mut Hit,
        reference_id: &Id,
        target: &IndexEntryProperty,
        before_id: &Option<String>,
    ) -> Result<(), HitError>;
    fn on_before_remove_reference(
        &mut self,
        instance: &mut Hit,
        reference_id: &Id,
        target: &IndexEntryProperty,
    ) -> Result<(), HitError>;
    fn on_after_remove_reference(
        &mut self,
        instance: &mut Hit,
        reference_id: &Id,
        target: &IndexEntryProperty,
    ) -> Result<(), HitError>;
}

pub trait Plugin {
    fn on_before_add_entry(
        &mut self,
        model: Rc<Model>,
        id: &str,
        data: ObjectValues,
        parent: IndexEntryProperty,
        before_id: &Option<Id>,
        instance: &Hit,
    ) -> Result<(), HitError>;

    fn on_after_add_entry(
        &mut self,
        model: Rc<Model>,
        id: &str,
        data: ObjectValues,
        parent: IndexEntryProperty,
        before_id: &Option<Id>,
        instance: &mut Hit,
    ) -> Result<(), HitError>;

    fn on_before_set_value(
        &mut self,
        property: IndexEntryProperty,
        value: &ObjectValue,
        old_value: &Option<ObjectValue>,
        instance: &Hit,
    ) -> Result<(), HitError>;

    fn on_after_set_value(
        &mut self,
        property: IndexEntryProperty,
        value: &ObjectValue,
        old_value: &Option<ObjectValue>,
        instance: &mut Hit,
    ) -> Result<(), HitError>;

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
