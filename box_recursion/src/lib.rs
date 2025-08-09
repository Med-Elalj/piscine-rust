#[derive(Debug, PartialEq)]
pub enum Role {
    CEO,
    Manager,
    Worker,
}

impl From<&str> for Role {
    fn from(s: &str) -> Self {
        match s.to_lowercase().as_str() {
            "ceo" => Role::CEO,
            "manager" => Role::Manager,
            _ => Role::Worker, // Treat everything else as Worker
        }
    }
}

// Link is an alias for the recursive pointer structure
pub type Link = Option<Box<Worker>>;

#[derive(Debug)]
pub struct Worker {
    pub role: String,
    pub name: String,
    pub next: Link,
}

#[derive(Debug)]
pub struct WorkEnvironment {
    pub grade: Link,
}

impl WorkEnvironment {
    pub fn new() -> Self {
        WorkEnvironment { grade: None }
    }

    pub fn add_worker(&mut self, name: &str, role: &str) {
        let new_worker = Worker {
            name: name.to_string(),
            role: Role::from(role).to_string(),
            next: self.grade.take(),
        };
        self.grade = Some(Box::new(new_worker));
    }

    pub fn remove_worker(&mut self) -> Option<String> {
        match self.grade.take() {
            Some(worker_box) => {
                self.grade = worker_box.next;
                Some(worker_box.name)
            }
            None => None,
        }
    }

    pub fn last_worker(&self) -> Option<(String, Role)> {
        self.grade.as_ref().map(|worker| {
            (
                worker.name.clone(),
                Role::from(worker.role.as_str()),
            )
        })
    }
}

// Implement Display logic (not necessary, but helpful for clarity)
impl ToString for Role {
    fn to_string(&self) -> String {
        match self {
            Role::CEO => "CEO",
            Role::Manager => "Manager",
            Role::Worker => "Worker",
        }
        .to_string()
    }
}
