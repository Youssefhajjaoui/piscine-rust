use rand::Rng;
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
    King,
    Queen,
    Jack,
    Number(u8),
}

impl Suit {
    pub fn random() -> Suit {
        let random = rand::thread_rng().gen_range(1..=4);
        Self::translate(random)
    }

    pub fn translate(value: u8) -> Suit {
        return match value {
            1 => Suit::Heart,
            2 => Suit::Diamond,
            3 => Suit::Spade,
            4 => Suit::Club,
            _ => unreachable!(),
        };
    }
}

impl Rank {
    pub fn random() -> Rank {
        let random = rand::thread_rng().gen_range(1..=4);
        Self::translate(random)
    }

    pub fn translate(value: u8) -> Rank {
        return match value {
            1 => Rank::Ace,
            11 => Rank::Jack,
            12 => Rank::Queen,
            13 => Rank::King,
            _ => Rank::Number(value),
        };
    }
}

#[derive(Debug, PartialEq)]
pub struct Card {
    pub suit: Suit,
    pub rank: Rank,
}

pub fn winner_card(card: &Card) -> bool {
    if card.rank == Rank::Ace && card.suit == Suit::Spade {
        return true;
    }
    return false;
}