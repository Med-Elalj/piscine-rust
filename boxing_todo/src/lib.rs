mod err;

use err::{ParseErr, ReadErr};

use std::error::Error;
use std::fs;

#[derive(Debug, Eq, PartialEq)]
pub struct Task {
    pub id: u32,
    pub description: String,
    pub level: u32,
}

#[derive(Debug, Eq, PartialEq)]
pub struct TodoList {
    pub title: String,
    pub tasks: Vec<Task>,
}

impl TodoList {
    pub fn get_todo(path: &str) -> Result<TodoList, Box<dyn Error>> {
        let content = fs::read_to_string(path)
            .map_err(|e| Box::new(ReadErr { child_err: Box::new(e) }) as Box<dyn Error>)?;

        let parsed = json::parse(&content)
            .map_err(|e| Box::new(ParseErr::Malformed(Box::new(e))) as Box<dyn Error>)?;

        let title = parsed["title"]
            .as_str()
            .ok_or_else(|| Box::new(ParseErr::Malformed("Missing or invalid title".into())))?;

        let tasks_json = parsed["tasks"]
            .members()
            .map(|task_json| {
                let id = task_json["id"]
                    .as_u32()
                    .ok_or_else(|| "Invalid id field")?;
                let description = task_json["description"]
                    .as_str()
                    .ok_or_else(|| "Invalid description field")?;
                let level = task_json["level"]
                    .as_u32()
                    .ok_or_else(|| "Invalid level field")?;

                Ok(Task {
                    id,
                    description: description.to_string(),
                    level,
                })
            })
            .collect::<Result<Vec<_>, _>>()
            .map_err(|e: &str| Box::new(ParseErr::Malformed(Box::<dyn Error>::from(e.to_string()))) as Box<dyn Error>)?;

        if tasks_json.is_empty() {
            return Err(Box::new(ParseErr::Empty));
        }

        Ok(TodoList {
            title: title.to_string(),
            tasks: tasks_json,
        })
    }
}
