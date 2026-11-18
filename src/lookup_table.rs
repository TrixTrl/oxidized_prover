use crate::{math_execution::Execution, math_function::Function};
use std::{
    cell::{Cell, RefCell},
    collections::HashMap,
    rc::Rc,
};

#[derive(Default)]
pub struct LookupTable<T> {
    current_id: Cell<u64>,
    pub map: RefCell<HashMap<u64, T>>,
}

impl<T> LookupTable<T> {
    pub fn add_value(self: &Rc<LookupTable<T>>, value: T) -> LookupID<T> {
        let mut map = self.map.borrow_mut();
        let current_id = self.current_id.take();

        assert!(
            map.insert(current_id, value).is_none(),
            "Failed to correctly add new value to map for some reason, current_id is {current_id}"
        );
        self.current_id.set(current_id + 1);

        LookupID(self.current_id.get() - 1, self.clone())
    }
}

pub fn create_lookup_table<T>() -> LookupTable<T> {
    return LookupTable {
        current_id: Cell::new(0),
        map: RefCell::new(HashMap::new()),
    };
}

#[derive(Clone)]
pub struct LookupID<T>(pub u64, pub Rc<LookupTable<T>>);

impl<T> LookupID<T> {
    pub fn update_value(&mut self, value: T) {
        self.1.map.borrow_mut().insert(self.0, value);
    }
}

impl<T> Drop for LookupID<T> {
    fn drop(&mut self) {
        assert!(self.1.map.borrow_mut().remove(&self.0).is_some(), "Somehow tried dropping an already dropped reference");
    }
}

impl<T: Clone> LookupID<T> {
    pub fn get_value(&self) -> T {
        self.1
            .map
            .borrow()
            .get(&self.0)
            .expect("Tried reading from a dead lookup value")
            .clone()
    }
}
