mod messenger;
pub use messenger::*;
pub use std::cell::RefCell;
use std::{collections::HashMap, rc::Rc};

pub struct Worker {
    pub track_value: Rc<u64>,
    pub mapped_messages: RefCell<HashMap<String, String>>,
    pub all_messages: RefCell<Vec<String>>,
}

impl Worker {
    pub fn new(nbr: u64) -> Self {
        Self {
            track_value: Rc::new(nbr),
            mapped_messages: RefCell::new(HashMap::new()),
            all_messages: RefCell::new(vec![]),
        }
    }
}

impl Logger for Worker {
    fn warning(&self, msg: &str) {
        println!("{}", msg);
        self.mapped_messages
            .borrow_mut()
            .insert(String::from("Warning"), msg.to_string());
    }

    fn info(&self, msg: &str) {
        println!("{}", msg);
        self.mapped_messages
            .borrow_mut()
            .insert(String::from("Info"), msg.to_string());
    }

    fn error(&self, msg: &str) {
        println!("{}", msg);
        self.mapped_messages
            .borrow_mut()
            .insert(String::from("Error"), msg.to_string());
    }
}
