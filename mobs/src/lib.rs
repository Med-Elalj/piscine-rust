use std::collections::{HashMap, HashSet};

mod mobs;
pub use crate::mobs::*;

impl Mob {
    pub fn new(name: String, boss: Boss, wealth: u64) -> Self {
        Mob { name: name, boss: boss,
         members: HashMap::new(), cities: HashSet::new(), wealth: wealth }
    }
    pub fn recruit(&mut self, data: (&str, u32)) {
        self.members.insert(data.0.to_string(),Member::new(data.1));
    }
    fn power(&self) -> usize {
        self.members.values().map(|m| m.power()).sum()
    }
    pub fn attack(&mut self, target: &mut Mob) {
        let (winers, loosers) = if self.power() > target.power() {
            (self, target)
        } else {
            (target, self)
        };

        {
            let youngest_age = loosers.members.values().map(|m| m.age).min().unwrap();   
            let mut f = false;
            loosers.members.retain(|_, m| {
                if f || m.age > youngest_age {true} else {f= true; false}
            });
        }

        if loosers.members.is_empty() {
            winers.cities.extend(loosers.cities.drain());
            winers.wealth += loosers.wealth;
            loosers.wealth = 0;
        }
    }

    pub fn steal(&mut self , target: &mut Mob, v: u64) {
        if self.power() > target.power() {
            let v = v.min(target.wealth);
            self.wealth += v;
            target.wealth -= v;
        };
    }

    pub fn conquer_city(&mut self, others: &[&Mob], wanted_city: String) {
        if !others.iter().flat_map(|m| &m.cities).cloned().collect::<HashSet<_>>().contains(&wanted_city)
        {
            self.cities.insert(wanted_city);
        }
    }
}