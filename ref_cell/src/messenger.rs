use std::{collections::HashMap, rc::Rc};

pub trait Logger {
    fn warning(&self, msg: &str);
    fn info(&self, msg: &str);
    fn error(&self, msg: &str);
}

pub struct Tracker<'a> {
    pub logger: &'a dyn Logger,
    pub value: u64,
    pub max: u64,
}

impl<'a> Tracker<'a> {
    pub fn new(logger: &'a dyn Logger, nbr: u64) -> Self {
        Self {
            logger,
            value: 0,
            max: nbr,
        }
    }
    pub fn set_value(&mut self, value: &Rc<u64>) {
        if self.value + **value < self.max {
            self.value += **value;
        }
    }
    pub fn peek(&self) {
        self.logger.info("")
    }
}
