#[derive(Debug)]
pub struct WorkEnvironment {
    pub grade: Link,
}

pub type Link = Option<Box<Worker>>;

#[derive(Debug, PartialEq, Clone)]
pub struct Worker {
    pub role: String,
    pub name: String,
    pub next: Link,
}

impl WorkEnvironment {
    pub fn new() -> WorkEnvironment {
        Self { grade: None }
    }
    pub fn add_worker(&mut self, role: String, name: String) {
        if self.grade == None {
            self.grade = Some(Box::new(Worker {
                role,
                name,
                next: None,
            }))
        } else {
            self.grade = Some(Box::new(Worker {
                role,
                name,
                next: self.grade.clone(),
            }))
        }
    }
    pub fn remove_worker(&mut self) -> Option<String> {
        let last = self.grade.clone().unwrap();
        self.grade = self.grade.clone().unwrap().next;
        Some(last.name)
    }
    pub fn last_worker(&self) -> Option<(String, String)> {
        let last = self.grade.clone().unwrap();
        Some((last.name, last.role))
    }
}
