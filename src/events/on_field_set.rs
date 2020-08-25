use crate::ObjectValue;
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

pub struct Listeners(HashMap<String, Vec<FieldListenerRef>>);

impl Listeners {
    pub fn new() -> Self {
        Listeners(HashMap::new())
    }

    fn get_or_create_property_listeners(&mut self, property: &str) -> &mut Vec<FieldListenerRef> {
        // TODO: vérifier que la property est présente dans le modèle de l'entry
        self.0.entry(property.to_string()).or_insert(vec![])
    }

    fn get_property_listeners_mut(&mut self, property: &str) -> Option<&mut Vec<FieldListenerRef>> {
        // TODO: vérifier que la property est présente dans le modèle de l'entry
        self.0.get_mut(property)
    }

    pub fn insert(&mut self, property: &str, value: FieldListenerRef) {
        let listeners = self.get_or_create_property_listeners(property);
        listeners.push(value);
    }

    pub fn remove(&mut self, property: &str, listener_id: &str) -> Result<(), &str> {
        let listeners = self
            .get_property_listeners_mut(property)
            .ok_or("Listener not found")?;
        listeners.retain(|listener2| listener2.borrow().get_unique_id() != listener_id);
        Ok(())
    }

    pub fn dispatch_value(&mut self, property: &str, value: &ObjectValue) {
        match self.get_property_listeners_mut(property) {
            Some(listeners) => {
                for listener in listeners.iter_mut() {
                    listener.borrow_mut().on_update(value);
                }
            }
            None => {}
        }
    }
}

pub type FieldListenerRef = Rc<RefCell<dyn FieldListener>>;

pub trait FieldListener {
    fn on_update(&mut self, value: &ObjectValue);
    fn get_unique_id(&self) -> &str;
    fn on_delete(&mut self);
}

impl PartialEq for dyn FieldListener {
    fn eq(&self, other: &Self) -> bool {
        self.get_unique_id() == other.get_unique_id()
    }
}
