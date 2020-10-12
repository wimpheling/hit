use crate::HitError;
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

pub struct Listeners<T>(HashMap<String, Vec<FieldListenerRef<T>>>);

impl<T> Listeners<T> {
    pub fn new() -> Self {
        Listeners(HashMap::new())
    }

    fn get_or_create_property_listeners(
        &mut self,
        property: &str,
    ) -> &mut Vec<FieldListenerRef<T>> {
        self.0.entry(property.to_string()).or_insert(vec![])
    }

    fn get_property_listeners_mut(
        &mut self,
        property: &str,
    ) -> Option<&mut Vec<FieldListenerRef<T>>> {
        self.0.get_mut(property)
    }

    pub fn insert(&mut self, property: &str, value: FieldListenerRef<T>) {
        let listeners = self.get_or_create_property_listeners(property);
        listeners.push(value);
    }

    pub fn remove(&mut self, property: &str, listener_id: &str) -> Result<(), HitError> {
        let listeners = self
            .get_property_listeners_mut(property)
            .ok_or(HitError::ListenerNotFound(listener_id.to_string()))?;
        listeners.retain(|listener2| listener2.borrow().get_unique_id() != listener_id);
        Ok(())
    }

    pub fn dispatch_value(&mut self, property: &str, value: &T) {
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

pub type FieldListenerRef<T> = Rc<RefCell<dyn FieldListener<T>>>;

pub trait FieldListener<T> {
    fn on_update(&mut self, value: &T);
    fn get_unique_id(&self) -> &str;
}

// TODO: is this useful ?
/* impl PartialEq for dyn FieldListener {
    fn eq(&self, other: &Self) -> bool {
        self.get_unique_id() == other.get_unique_id()
    }
} */
