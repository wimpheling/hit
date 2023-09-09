use linked_hash_map::LinkedHashMap;
use serde::{Serialize, Deserialize};

use crate::{Hit, ObjectValue, Id, ObjectValues, IndexEntryProperty};

#[derive(Debug, Serialize, Deserialize)]
pub struct PatchPropertyDifference {
    pub id: String,
    pub property: String,
    pub old_value: ObjectValue,
    pub new_value: ObjectValue,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct AddedEntry {
    pub id: Id,
    pub data: ObjectValues,
    pub parent: Option<IndexEntryProperty>,
    pub model: String,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct Patch {
    pub differences: Vec<PatchPropertyDifference>,
    pub deleted: Vec<String>,
    pub added: Vec<AddedEntry>,
}

pub fn create_patch(old: Hit, new: Hit) -> Patch {
    let mut deleted = Vec::new();
    let mut added = Vec::new();
    let mut differences = Vec::new();
    for entry in old.index.iter() {
        let id = entry.0.clone();
        let old_entry = entry.1;
        if !new.index.contains(&id) {
            deleted.push(id);
            continue;
        }
        let old_entry = old_entry.borrow();
        let old_model = old.get_model(&id.clone()).unwrap();
        
        for field in old_model.fields.iter() {
            let old_value = old_entry.get(&field.0);
            let new_value = new.get_value(&id.clone(), field.0).unwrap();
            if &new_value != old_value {
                differences.push(PatchPropertyDifference {
                    id: id.clone(),
                    property: field.0.clone(),
                    old_value: old_value.clone(),
                    new_value,
                });
            }
        }
    }

    // find added items
    for entry in new.index.iter() {
        let id = entry.0.clone();
        if !old.index.contains(&id) {
            add_entry(entry, &new, id, &mut added);
        }
    }
    return Patch {
        differences,
        deleted,
        added,
    };
}

pub(super) fn add_entry(entry: (&String, &std::rc::Rc<std::cell::RefCell<crate::index::IndexEntry>>), new: &Hit, id: String, added: &mut Vec<AddedEntry>) {
    let index_entry = entry.1;
    let index_entry = index_entry.borrow();
    let model = new.get_model(&id).unwrap();

    // filter data to remove null values
    let mut data = LinkedHashMap::new();
    for (key, value) in index_entry.data.iter() {
        if value != &ObjectValue::Null {
            data.insert(String::from(key), value.clone());
        }
    }

    added.push(AddedEntry {
        data,
        id: index_entry.get_id().clone(),
        parent: index_entry.get_parent().clone(),
        model: model.get_name().clone(),
    });
}

// pub fn export_entries(hit: Hit) -> LinkedHashMap<String, AddedEntry> {
//     let mut entries = LinkedHashMap::new();
//     for entry in hit.index.iter() {
//         let id = entry.0.clone();
//         let model = hit.get_model(&id).unwrap();
//         entries.insert(id.clone(), AddedEntry {
//             id,
//             data: entry.1.borrow().data.clone(),
//             parent: entry.1.borrow().get_parent(),
//             model: model.get_name().clone(),
//         });
//     }
//     entries
// }

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{Hit, IndexEntryProperty, LinkedHashMap, Reference, test_kernel::create_test_kernel};
    
    use std::rc::Rc;

    #[test]
    fn test_create_patch() {
        let kernel = create_test_kernel();
        let kernel = Rc::new(kernel);
        let mut old = Hit::new("main", "test/test", kernel.clone()).unwrap();
        let mut fields = LinkedHashMap::new();
        fields.insert(
            "name".to_string(),
            ObjectValue::String("model 1".to_string()),
        );
        old.insert(
            "test/test",
            "id",
            fields,
            IndexEntryProperty {
                id: "main".to_string(),
                property: "sub_items".to_string(),
            },
            None,
        )
        .unwrap();
        let mut fields = LinkedHashMap::new();
        fields.insert(
            "deleted_folder".to_string(),
            ObjectValue::String("model 1".to_string()),
        );
        old.insert(
            "test/test",
            "deleted_folder",
            fields,
            IndexEntryProperty {
                id: "main".to_string(),
                property: "sub_items".to_string(),
            },
            None,
        )
        .unwrap();
        let mut new = Hit::new("main", "test/test", kernel).unwrap();
        let mut fields = LinkedHashMap::new();
        fields.insert(
            "name".to_string(),
            ObjectValue::String("other_name".to_string()),
        );
        new.insert(
            "test/test",
            "id",
            fields,
            IndexEntryProperty {
                id: "main".to_string(),
                property: "sub_items".to_string(),
            },
            None,
        )
        .unwrap();
        let mut fields = LinkedHashMap::new();
        fields.insert(
            "name".to_string(),
            ObjectValue::String("added_folder".to_string()),
        );
        new.insert(
            "test/test",
            "added_folder",
            fields.clone(),
            IndexEntryProperty {
                id: "main".to_string(),
                property: "sub_items".to_string(),
            },
            None,
        )
        .unwrap();

        let patch = create_patch(old, new);
        assert_eq!(patch.added, vec![AddedEntry {
            id: "added_folder".to_string(),
            data: fields.clone(),
            parent: Some(IndexEntryProperty {
                id: "main".to_string(),
                property: "sub_items".to_string(),
            }),
            model: "test/test".to_string(),
        }]);
        assert_eq!(patch.deleted, vec!["deleted_folder"]);

        assert_eq!(patch.differences.len(), 2);
        assert_eq!(patch.differences[0].id, "id");
        assert_eq!(patch.differences[0].property, "name");
        assert_eq!(patch.differences[0].old_value, ObjectValue::String("model 1".to_string()));
        assert_eq!(patch.differences[0].new_value, ObjectValue::String("other_name".to_string()));


        assert_eq!(patch.differences[1].id, "main");
        assert_eq!(patch.differences[1].property, "sub_items");
        assert_eq!(patch.differences[1].old_value, ObjectValue::VecSubObjects(vec![
            Reference {
                id: "id".to_string(),
            },
            Reference {
                id: "deleted_folder".to_string(),
            }
        ]));
        assert_eq!(patch.differences[1].new_value, ObjectValue::VecSubObjects(vec![
            Reference {
                id: "id".to_string(),
            },
            Reference {
                id: "added_folder".to_string(),
            }
        ]));
    }
}
