#[derive(Debug, PartialEq, Clone)]
pub enum Role {
    Underboss,
    Caporegime,
    Soldier,
    Associate,
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
