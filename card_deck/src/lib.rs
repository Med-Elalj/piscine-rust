use rand::Rng;

#[derive(PartialEq)]
pub enum Suit {
    Heart,
    Diamond,
    Spade,
    Club,
}

#[derive(PartialEq)]
pub enum Rank {
    Ace,
    Number(u8),
    King,
    Queen,
    Jack,
}

impl Suit {
    pub fn random() -> Suit {
        let value = rand::rng().random_range(1..=4);//.gen_range(1..=4);
        Suit::translate(value)
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
        let value = rand::rng().random_range(1..=13);
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

pub struct Card {
    pub suit: Suit,
    pub rank: Rank,
}

pub fn winner_card(card: &Card) -> bool {
    card.rank == Rank::Ace || card.suit == Suit::Spade
}
