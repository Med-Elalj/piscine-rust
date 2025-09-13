use std::{collections::{HashMap, HashSet}, fmt::Debug};

pub mod member;
pub use member::Role;

#[derive(Debug)]
pub struct Mob {
    pub name: String,
    pub boss: Boss,
    pub members: HashMap<String, Member>,
    pub cities: HashSet<String>,
    pub wealth: u64,
}


#[derive(Debug, PartialEq, Clone)]
pub struct Boss {
    pub name: String,
    pub age: u32,
}

impl Boss {
    pub fn new(name: &str, age: u32) -> Self {
        Self {
            name: name.to_string(),
            age,
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct Member {
    pub role: Role,
    pub age: u32,
}

impl Member {
    pub fn get_promotion(&mut self) {
        self.role = match self.role {
            Role::Associate => Role::Soldier,
            Role::Soldier => Role::Caporegime,
            Role::Caporegime => Role::Underboss,
            Role::Underboss => panic!("Cannot promote beyond Underboss!"),
        };
    }
    pub fn new(age: u32) -> Self {
        Self{role :Role::Associate,age:age}
    }
    pub fn power(&self) -> usize {
        match self.role {
                Role::Underboss => 4,
                Role::Caporegime => 3,
                Role::Soldier => 2,
                Role::Associate => 1,
            }
    }
}
