use std::{collections::HashMap, rc::Rc};

use crate::Worker;

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

    pub fn set_value<T>(&self, worker: &Rc<T>) {
    let count = Rc::strong_count(worker) as u64;
    let percent = count * 100 / self.max;

    if percent >= 100 {
        self.logger
            .error("Error: you are over your quota!");
    } else if percent >= 70 {
        self.logger
            .warning(&format!(
                "Warning: you have used up over {}% of your quota! Proceeds with precaution",
                percent
            ));
    }
}

    pub fn peek<T>(&self, worker: &Rc<T>) {
    let count = Rc::strong_count(worker) as u64;
    let percent = count * 100 / self.max;
    self.logger
        .info(&format!("Info: you are using up to {}% of your quota", percent));
    }

}
