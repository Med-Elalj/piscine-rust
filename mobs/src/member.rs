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
}
