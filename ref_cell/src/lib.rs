mod messenger;
pub use messenger::*;
pub use std::cell::RefCell;
use std::{collections::HashMap, rc::Rc};

pub struct Worker {
    pub track_value: Rc<usize>,
    pub mapped_messages: RefCell<HashMap<String, String>>,
    pub all_messages: RefCell<Vec<String>>,
}

impl Worker {
    pub fn new(nbr: usize) -> Self {
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
            .insert(String::from("Warning"), msg.strip_prefix("Warning: ").unwrap().to_string());
        self.all_messages.borrow_mut().push(format!("{}", msg));
    }

    fn info(&self, msg: &str) {
        println!("{}", msg);
        self.mapped_messages
            .borrow_mut()
            .insert(String::from("Info"), msg.strip_prefix("Info: ").unwrap().to_string());
        self.all_messages.borrow_mut().push(format!("{}", msg));
    }

    fn error(&self, msg: &str) {
        println!("Error: {}", msg);
        self.mapped_messages
            .borrow_mut()
            .insert(String::from("Error"), msg.strip_prefix("Error: ").unwrap().to_string());
        self.all_messages.borrow_mut().push(format!("{}", msg));
    }
}
