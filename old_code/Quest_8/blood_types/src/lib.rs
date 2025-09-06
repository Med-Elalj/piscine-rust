#[derive(Debug, PartialEq, Eq, Clone, PartialOrd, Ord)]
pub enum Antigen {
    O,
    AB,
    A,
    B,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone)]
pub enum RhFactor {
    Positive,
    Negative,
}

#[derive(PartialEq, Eq, PartialOrd, Clone)]
pub struct BloodType {
    pub antigen: Antigen,
    pub rh_factor: RhFactor,
}

use std::str::FromStr;
use std::fmt;

impl FromStr for Antigen {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "AB" => Ok(Antigen::AB),
            "O" => Ok(Antigen::O),
            "A" => Ok(Antigen::A),
            "B" => Ok(Antigen::B),
            _ => Err(()),
        }
    }
}

impl FromStr for RhFactor {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "+" => Ok(RhFactor::Positive),
            "-" => Ok(RhFactor::Negative),
            _ => Err(()),
        }
    }
}

impl FromStr for BloodType {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = s.trim();
        let rh = &s[s.len() - 1..];
        let antigen = &s[..s.len() - 1];
        Ok(BloodType {
            antigen: antigen.parse()?,
            rh_factor: rh.parse()?,
        })
    }
}

use std::cmp::Ordering;

impl Ord for BloodType {
    fn cmp(&self, other: &Self) -> Ordering {
        self.antigen
            .cmp(&other.antigen)
            .then(self.rh_factor.cmp(&other.rh_factor))
    }
}

impl fmt::Debug for BloodType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let antigen = match self.antigen {
            Antigen::AB => "AB",
            Antigen::O => "O",
            Antigen::A => "A",
            Antigen::B => "B",
        };
        let rh = match self.rh_factor {
            RhFactor::Positive => "+",
            RhFactor::Negative => "-",
        };
        write!(f, "{}{}", antigen, rh)
    }
}

impl BloodType {
    pub fn can_receive_from(&self, other: &Self) -> bool {
        use Antigen::*;
        use RhFactor::*;

        let antigen_compatible = match self.antigen {
            O => other.antigen == O,
            A => matches!(other.antigen, O | A),
            B => matches!(other.antigen, O | B),
            AB => true,
        };

        let rh_compatible = match self.rh_factor {
            Positive => true, // can receive both + and -
            Negative => matches!(other.rh_factor, Negative),
        };

        antigen_compatible && rh_compatible
    }

    pub fn donors(&self) -> Vec<Self> {
        let all = Self::all();
        all.into_iter()
            .filter(|b| self.can_receive_from(b))
            .collect()
    }

    pub fn recipients(&self) -> Vec<BloodType> {
        let all = Self::all();
        all.into_iter()
            .filter(|b| b.can_receive_from(self))
            .collect()
    }

    fn all() -> Vec<BloodType> {
        use Antigen::*;
        use RhFactor::*;
        let antigens = vec![AB, O, A, B];
        let rh_factors = vec![Positive, Negative];

        let mut types = vec![];
        for antigen in &antigens {
            for rh in &rh_factors {
                types.push(BloodType {
                    antigen: antigen.clone(),
                    rh_factor: rh.clone(),
                });
            }
        }
        types
    }
}
