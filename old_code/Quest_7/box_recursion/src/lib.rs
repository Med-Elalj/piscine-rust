#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Role {
    CEO,
    Manager,
    Worker,
}

impl From<&str> for Role {
    fn from(role: &str) -> Self {
        match role {
            "CEO" => Role::CEO,
            "Manager" => Role::Manager,
            _ => Role::Worker,
        }
    }
}

pub type Link = Option<Box<Worker>>;

#[derive(Debug)]
pub struct Worker {
    pub role: Role,
    pub name: String,
    pub next: Link,
}

#[derive(Default, Debug)]
pub struct WorkEnvironment {
    pub grade: Link,
}

impl WorkEnvironment {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add_worker(&mut self, name: &str, role: &str) {
        let new_worker = Worker {
            name: name.to_string(),
            role: Role::from(role),
            next: self.grade.take(),
        };
        self.grade = Some(Box::new(new_worker));
    }

    pub fn remove_worker(&mut self) -> Option<String> {
        self.grade.take().map(|node| {
            self.grade = node.next;
            node.name
        })
    }

    pub fn last_worker(&self) -> Option<(String, Role)> {
        self.grade
            .as_ref()
            .map(|node| (node.name.clone(), node.role))
    }
}
