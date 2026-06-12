use strum_macros::EnumIter;

use crate::{cards::enhancements::Enhancement, scoring::ScoreModification};

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
    enhancement: Option<Enhancement>,
}

impl Card {
    pub fn new(rank: Rank, suit: Suit) -> Self {
        Self {
            rank,
            suit,
            enhancement: None,
        }
    }

    pub fn rank(&self) -> Rank {
        self.rank
    }

    pub fn suit(&self) -> Suit {
        self.suit
    }

    pub fn enhancement(&self) -> Option<Enhancement> {
        self.enhancement
    }

    pub fn add_enhancement(&mut self, enhancement: Enhancement) {
        self.enhancement = Some(enhancement);
    }

    pub fn on_scored(&self) -> Vec<ScoreModification<'_>> {
        let mut scoring = vec![ScoreModification::Chips(self.base_chips())];

        if let Some(e) = self.enhancement.as_ref() {
            scoring.extend(e.on_scored());
        }

        scoring
    }

    fn base_chips(&self) -> usize {
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

    #[test]
    fn test_add_enhancement() {
        let mut card = Card::new(Rank::Ace, Suit::Spade);
        assert!(card.enhancement().is_none());

        card.add_enhancement(Enhancement::Mult);
        assert!(card.enhancement().is_some_and(|e| e == Enhancement::Mult));
    }

    #[test]
    fn test_scoring_enhanced_card() {
        let mut card = Card::new(Rank::Ace, Suit::Spade);
        card.add_enhancement(Enhancement::Lucky);

        let scoring = card.on_scored();

        assert_eq!(scoring.len(), 3);
        assert!(scoring.contains(&ScoreModification::Chips(11)));
        assert!(scoring.contains(&ScoreModification::Chance(
            1,
            5,
            &ScoreModification::Mult(20)
        )));
        assert!(scoring.contains(&ScoreModification::Chance(
            1,
            15,
            &ScoreModification::Money(20)
        )));
    }
}
