mod err;

use std::error::Error;
use std::fs::File;
use std::io::{BufReader , Read};
use serde::Deserialize;


#[derive(Debug, Eq, PartialEq , Deserialize)]
pub struct Task {
    pub id: u32,
    pub description: String,
    pub level: u32,
}

#[derive(Debug, Eq, PartialEq, Deserialize)]
pub struct TodoList {
    pub title: String,
    pub tasks: Vec<Task>, 
}

impl TodoList {
  pub fn get_todo(path: &str) -> Result<TodoList, Box<dyn Error>> {
    let mut input = String::new();
    let file = File::open(path)?;
    let mut buffer_reader = BufReader::new(file);
    buffer_reader.read_to_string(&mut input)?;

    println!("Raw file content:\n{}", input);

    let todo_list: TodoList = serde_json::from_str(&input)?;
    Ok(todo_list)
}
}