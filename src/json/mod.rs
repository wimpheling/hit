pub mod export;
pub mod import;
mod utils;

#[cfg(test)]
mod tests {
    use crate::json::import::import;
    use crate::object_data::ObjectValue;
    use crate::test_kernel::create_test_kernel;
    use serde_json::json;
    use std::rc::Rc;
    #[test]
    pub fn no_duplicate_ids() {
        let json_data = json!({
            "data": [{
                "model": "test/test",
                "id": "id1",
                "data": {
                    "name": "Hello",
                    "age": 12,
                    "sub_items": {
                        "type": "subobject_array",
                        "value": [{"id": "id1"}]
                    }
                },
                "parent": null
            },  {
                "model": "test/test",
                "id": "id1",
                "data": {
                    "name": "Hello2",
                },
                "parent": {
                    "id": "id1",
                    "property": "sub_items",
                }
            }],
           "id": "id1"
        });
        let kernel = create_test_kernel();
        let result = import(&json_data, Rc::new(kernel));
        match result {
            Ok(_index) => assert!(false),
            Err(error) => assert_eq!(error, "Id already exists in this document."),
        }
    }

    #[test]
    pub fn test_json_import() {
        let json_data = json!({
            "data": [{
                "model": "test/test",
                "id": "id1",
                "data": {
                    "name": "Hello",
                    "age": 12,
                    "sub_items": {
                        "type": "subobject_array",
                        "value": [{"id": "id2"}]
                    }
                },
                "parent": null
            }, {
                "model": "test/test",
                "id": "id2",
                "data": {
                    "name": "Hello2",
                    "age": 123,
                    "sub_items": {
                        "type": "subobject_array",
                        "value": [{"id": "id3"}]
                    }
                },
                "parent": {
                    "id": "id1",
                    "property": "sub_items",
                    "position": 0
                }
            }, {
                "model": "test/test",
                "id": "id3",
                "data": {
                    "name": "Hello3",
                    "age": 123
                },
                "parent": {
                    "id": "id2",
                    "property": "sub_items",
                    "position": 0
                }
            }],
            "id": "id1"
        });

        let kernel = create_test_kernel();
        let index = import(&json_data, Rc::new(kernel)).expect("Import failed");
        let id2 = index.get("id2").expect("id2 not found");
        let parent_id = id2
            .entry
            .borrow()
            .get_parent_id()
            .expect("parent for id2 not found");
        assert_eq!(parent_id, "id1");

        let entry = id2.entry.clone();
        let entry = entry.borrow();
        let name = entry.get("name");
        match name {
            ObjectValue::String(name) => assert_eq!(name, "Hello2"),
            _ => panic!("Name of id2 not found"),
        }

        let clone = id2.entry.clone();
        let clone = clone.borrow();
        let sub_items = clone.get("sub_items");
        match sub_items {
            ObjectValue::VecSubObjects(sub_items) => {
                let sub_item = sub_items.get(0).expect("Sub item id3 not found in id 2");
                assert_eq!(sub_item.id, "id3");
            }
            _ => panic!("Wrong data type for subitems"),
        }
    }
}
