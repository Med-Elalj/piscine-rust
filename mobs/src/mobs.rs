use std::{collections::{HashMap, HashSet}, fmt::Debug};
mod boss;
pub use boss::Boss;
mod member;
pub use member::Member;

#[derive(Debug)]
pub struct Mob {
    pub name: String,
    pub boss: Boss,
    pub members: HashMap<String, Member>,
    pub cities: HashSet<String>,
    pub wealth: u64,
}
