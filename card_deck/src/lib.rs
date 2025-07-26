use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Debug, PartialEq)]
pub enum Suit {
    Heart,
    Diamond,
    Spade,
    Club,
}

#[derive(Debug, PartialEq)]
pub enum Rank {
    Ace,
    Number(u8),
    King,
    Queen,
    Jack,
}
fn random() -> u64 {
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards");
    now.subsec_nanos() as u64 + now.as_secs()
}

impl Suit {
    pub fn random() -> Suit {
        let value: u8 = (random() % 4+1) as u8;
        Suit::translate(value as u8)
    }

    pub fn translate(value: u8) -> Suit {
        match value {
            1 => Suit::Heart,
            2 => Suit::Diamond,
            3 => Suit::Spade,
            4 => Suit::Club,
            _ => panic!("Invalid value for Suit: {}", value),
        }
    }
}

impl Rank {
    pub fn random() -> Rank {
        let value: u8 = (random() % 13+1) as u8;
        Rank::translate(value)
    }

    pub fn translate(value: u8) -> Rank {
        match value {
            1 => Rank::Ace,
            2..=10 => Rank::Number(value),
            11 => Rank::Jack,
            12 => Rank::Queen,
            13 => Rank::King,
            _ => panic!("Invalid value for Rank: {}", value),
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct Card {
    pub suit: Suit,
    pub rank: Rank,
}

pub fn winner_card(card: &Card) -> bool {
    card.rank == Rank::Ace || card.suit == Suit::Spade
}
