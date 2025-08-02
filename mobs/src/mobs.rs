pub mod boss;
pub mod member;

pub use boss::Boss;
pub use member::{Member, Role};
use std::collections::{HashMap, HashSet};

#[derive(Debug, PartialEq)]
pub struct Mob {
    name: String,
    boss: Boss,
    members: HashMap<String, Member>,
    cities: HashSet<String>,
    wealth: u64,
}

impl Mob {
    pub fn new(name: &str, boss: Boss, wealth: u64) -> Self {
        Self {
            name: name.to_string(),
            boss,
            members: HashMap::new(),
            cities: HashSet::new(),
            wealth,
        }
    }

    pub fn recruit(&mut self, (name, age): (&str, u32)) {
        self.members.insert(
            name.to_string(),
            Member {
                role: Role::Associate,
                age,
            },
        );
    }

    fn combat_score(&self) -> u32 {
        self.members
            .values()
            .map(|m| match m.role {
                Role::Underboss => 4,
                Role::Caporegime => 3,
                Role::Soldier => 2,
                Role::Associate => 1,
            })
            .sum()
    }

    pub fn attack(&mut self, target: &mut Mob) {
        let self_score = self.combat_score();
        let target_score = target.combat_score();

        enum Loser {
            SelfMob,
            TargetMob,
        }

        let loser = if self_score > target_score {
            Loser::TargetMob
        } else if self_score < target_score {
            Loser::SelfMob
        } else {
            // Draw: attacker loses
            Loser::SelfMob
        };

        let (loser_mob, winner_mob) = match loser {
            Loser::SelfMob => (self, target),
            Loser::TargetMob => (target, self),
        };

        // Remove youngest member(s) from the loser mob
        if let Some((youngest_name, _)) = loser_mob
            .members
            .iter()
            .min_by_key(|(_, m)| m.age)
            .map(|(k, v)| (k.clone(), v.clone()))
        {
            loser_mob.members.remove(&youngest_name);
        }

        // If loser has no members left, winner takes cities and wealth
        if loser_mob.members.is_empty() {
            winner_mob.cities.extend(loser_mob.cities.drain());
            winner_mob.wealth += loser_mob.wealth;
            loser_mob.wealth = 0;
        }
    }

    pub fn steal(&mut self, target: &mut Mob, amount: u64) {
        let stolen = target.wealth.min(amount);
        target.wealth -= stolen;
        self.wealth += stolen;
    }

    pub fn conquer_city(&mut self, other_mobs: &[&Mob], city: String) {
        if other_mobs.iter().all(|mob| !mob.cities.contains(&city)) {
            self.cities.insert(city);
        }
    }
}