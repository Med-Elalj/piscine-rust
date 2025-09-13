use std::{collections::{HashMap, HashSet}, fmt::Debug};
pub mod boss;
use boss::Boss;
pub mod member;
use member::Member;

#[derive(Debug)]
pub struct Mob {
    pub name: String,
    pub boss: Boss,
    pub members: HashMap<String, Member>,
    pub cities: HashSet<String>,
    pub wealth: u64,
}
