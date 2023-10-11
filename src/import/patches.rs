use std::rc::Rc;

use linked_hash_map::LinkedHashMap;

use crate::{hit_mod::IndexModelImporter, HitError, Hit, HitKernel, Patch, generic_import::finish_import, AddedEntry};

use super::generic_import::import_data_object_values;


pub fn import_from_patches<'a>(
    patches: Vec<Patch>,
    id: &str,
    kernel: Rc<HitKernel>
) -> Result<Hit, HitError> {
    let mut entries = LinkedHashMap::new();
    import_from_patches_and_entries(patches, id, kernel, &mut entries)
}

pub fn import_from_patches_and_entries<'a>(
    patches: Vec<Patch>,
    id: &str,
    kernel: Rc<HitKernel>,
    entries: &mut LinkedHashMap<String, AddedEntry>
) -> Result<Hit, HitError> {
    for patch in patches {
        import_patch(patch, entries)?;
    }

    let mut importer = IndexModelImporter::new(id, kernel.clone());
    for entry in entries.iter() {
        let id = entry.0;
        let added_entry = entry.1;
        let model = {
            kernel
                .get_model(&added_entry.model)?
        };
        import_data_object_values(
            model,
            id.clone(),
            added_entry.parent.clone(),
            &mut importer,
            added_entry.data.clone(),
        )?;
    }

    let hit = finish_import(importer, kernel)?;
    return Ok(hit);
}

pub fn apply_patches(hit: &Hit, patches: Vec<Patch>) -> Result<Hit, HitError> {
    let mut entries = LinkedHashMap::new();
    for entry in hit.index.iter() {
        let model = hit.get_model(&entry.0).unwrap();
        entries.insert(entry.0.to_string(), AddedEntry {
            id: entry.0.clone(),
            data: entry.1.borrow().data.clone(),
            parent: entry.1.borrow().get_parent(),
            model: model.get_name().to_string(),
        });
    }
    import_from_patches_and_entries(patches, hit.get_main_object_id(), hit.kernel.clone(), &mut entries)
}

pub fn duplicate_hit(hit: &Hit) -> Result<Hit, HitError> {
    apply_patches(hit, vec![])
}

fn import_patch<'a>(
    patch: Patch,
    entries: &mut LinkedHashMap<String, AddedEntry>,
) -> Result<(), HitError> {
    for deleted in patch.deleted.iter() {
        entries.remove(deleted);
    }
    for added in patch.added.iter() {
        entries.insert(added.id.clone(), added.clone());
    }
    for changed in patch.differences.iter() {
        let entry = entries.get_mut(&changed.id).unwrap();
        entry.data.remove(&changed.property);
        entry
            .data
            .insert(changed.property.clone(), changed.new_value.clone());
    }
    return Ok(());
}
