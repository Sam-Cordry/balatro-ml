use strum_macros::EnumIter;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Hash, EnumIter)]
#[repr(u8)]
pub enum Rank {
    Ace = 1,
    King = 2,
    Queen = 3,
    Jack = 4,
    Ten = 5,
    Nine = 6,
    Eight = 7,
    Seven = 8,
    Six = 9,
    Five = 10,
    Four = 11,
    Three = 12,
    Two = 13,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Hash, EnumIter)]
pub enum Suit {
    Spade,
    Heart,
    Club,
    Diamond,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub struct Card {
    rank: Rank,
    suit: Suit,
}

impl Card {
    pub fn new(rank: Rank, suit: Suit) -> Self {
        Self { rank, suit }
    }

    pub fn base_chips(&self) -> usize {
        match self.rank {
            Rank::Two => 2,
            Rank::Three => 3,
            Rank::Four => 4,
            Rank::Five => 5,
            Rank::Six => 6,
            Rank::Seven => 7,
            Rank::Eight => 8,
            Rank::Nine => 9,
            Rank::Ten | Rank::Jack | Rank::Queen | Rank::King => 10,
            Rank::Ace => 11,
        }
    }

    pub fn rank(&self) -> Rank {
        self.rank
    }

    pub fn suit(&self) -> Suit {
        self.suit
    }
}

#[cfg(test)]
mod tests {
    use strum::IntoEnumIterator;

    use super::*;

    #[test]
    fn test_card_creation() {
        let card = Card::new(Rank::Ace, Suit::Spade);

        assert_eq!(card.rank(), Rank::Ace);
        assert_eq!(card.suit(), Suit::Spade);
    }

    #[test]
    fn test_scoring_basic_cards() {
        Rank::iter().for_each(|rank| {
            Suit::iter().for_each(move |suit| {
                let card = Card::new(rank, suit);

                match rank {
                    Rank::Two => assert_eq!(card.base_chips(), 2),
                    Rank::Three => assert_eq!(card.base_chips(), 3),
                    Rank::Four => assert_eq!(card.base_chips(), 4),
                    Rank::Five => assert_eq!(card.base_chips(), 5),
                    Rank::Six => assert_eq!(card.base_chips(), 6),
                    Rank::Seven => assert_eq!(card.base_chips(), 7),
                    Rank::Eight => assert_eq!(card.base_chips(), 8),
                    Rank::Nine => assert_eq!(card.base_chips(), 9),
                    Rank::Ten | Rank::Jack | Rank::Queen | Rank::King => {
                        assert_eq!(card.base_chips(), 10)
                    }
                    Rank::Ace => assert_eq!(card.base_chips(), 11),
                }
            })
        })
    }
}
