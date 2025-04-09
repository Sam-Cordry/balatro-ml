use std::fmt::Display;
use strum::{EnumIter, IntoEnumIterator};

use crate::model::{db::CardRow, jokers::Joker, ScoreModification};

pub fn get_standard_deck() -> Vec<Card> {
    let mut deck: Vec<Card> = vec![];
    for suit in Suit::iter() {
        for rank in Rank::iter() {
            deck.push(Card {
                suit,
                rank,
                chips: rank.get_value() as usize,
                ..Default::default()
            });
        }
    }
    deck
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub struct Card {
    pub suit: Suit,
    pub rank: Rank,
    pub edition: CardEdition,
    pub enhancement: Option<Enhancement>,
    pub seal: Option<Seal>,
    pub chips: usize,
}

impl Display for Card {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match (self.enhancement, self.seal) {
            (Some(e), Some(s)) => write!(
                f,
                "{} {} {} of {}s with a {} seal",
                self.edition, e, self.rank, self.suit, s
            ),
            (Some(e), None) => write!(f, "{} {} {} of {}s", self.edition, e, self.rank, self.suit),
            (None, Some(s)) => write!(
                f,
                "{} {} of {}s with a {} seal",
                self.edition, self.rank, self.suit, s
            ),
            (None, None) => write!(f, "{} {} of {}", self.edition, self.rank, self.suit),
        }
    }
}

impl Default for Card {
    fn default() -> Self {
        Self {
            suit: Suit::Spade,
            rank: Rank::Ace,
            edition: CardEdition::Base,
            enhancement: None,
            seal: None,
            chips: 11,
        }
    }
}

impl From<CardRow> for Card {
    fn from(value: CardRow) -> Self {
        Self {
            suit: value.suit,
            rank: value.rank,
            edition: value.edition,
            enhancement: value.enhancement,
            seal: value.seal,
            chips: value.chips as usize,
        }
    }
}

impl Card {
    pub fn increment(&mut self) {
        self.rank = self.rank.increment();
    }

    pub fn duplicate(&mut self, other: &Card) {
        self.suit = other.suit;
        self.rank = other.rank;
        self.edition = other.edition;
        self.enhancement = other.enhancement;
        self.seal = other.seal;
    }

    pub fn is_face_card(&self, jokers: &[Joker]) -> bool {
        jokers.iter().filter(|j| j.name() == "Pareidolia").count() > 0
            || self.rank == Rank::King
            || self.rank == Rank::Queen
            || self.rank == Rank::Jack
    }

    pub fn is_suit(&self, suit: Suit, jokers: &[Joker]) -> bool {
        self.suit == suit
            || (jokers.iter().any(|j| j.name() == "Smeared Joker")
                && ((suit == Suit::Spade && self.suit == Suit::Club)
                    || (suit == Suit::Club && self.suit == Suit::Spade)
                    || (suit == Suit::Heart && self.suit == Suit::Diamond)
                    || (suit == Suit::Diamond && self.suit == Suit::Heart)))
    }

    pub fn on_scored(&self) -> ScoreModification {
        let mut modification = ScoreModification::default();
        modification.chips += self.rank.get_value() as isize;
        modification
    }

    pub fn on_held(&self) -> ScoreModification {
        todo!("implement")
    }

    pub fn on_discard(&self) -> ScoreModification {
        todo!("implement")
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash, sqlx::Type)]
#[sqlx(type_name = "card_edition", rename_all = "lowercase")]
pub enum CardEdition {
    Base,
    Foil,
    Holographic,
    Polychrome,
}

impl Display for CardEdition {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Base => write!(f, "base"),
            Self::Foil => write!(f, "foil"),
            Self::Holographic => write!(f, "holographic"),
            Self::Polychrome => write!(f, "polychrome"),
        }
    }
}

impl CardEdition {
    pub fn on_scored(&self) -> ScoreModification {
        todo!("implement")
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash, EnumIter, sqlx::Type)]
#[sqlx(type_name = "card_suit", rename_all = "lowercase")]
pub enum Suit {
    Spade,
    Heart,
    Club,
    Diamond,
}

impl Display for Suit {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Spade => write!(f, "Spade"),
            Self::Heart => write!(f, "Heart"),
            Self::Club => write!(f, "Club"),
            Self::Diamond => write!(f, "Diamond"),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash, PartialOrd, Ord, EnumIter, sqlx::Type)]
#[sqlx(type_name = "card_rank", rename_all = "lowercase")]
pub enum Rank {
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Jack,
    Queen,
    King,
    Ace,
}

impl Display for Rank {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Two => write!(f, "Two"),
            Self::Three => write!(f, "Three"),
            Self::Four => write!(f, "Four"),
            Self::Five => write!(f, "Five"),
            Self::Six => write!(f, "Six"),
            Self::Seven => write!(f, "Seven"),
            Self::Eight => write!(f, "Eight"),
            Self::Nine => write!(f, "Nine"),
            Self::Ten => write!(f, "Ten"),
            Self::Jack => write!(f, "Jack"),
            Self::Queen => write!(f, "Queen"),
            Self::King => write!(f, "King"),
            Self::Ace => write!(f, "Ace"),
        }
    }
}

impl Rank {
    pub fn increment(&mut self) -> Self {
        match self {
            Self::Two => Self::Three,
            Self::Three => Self::Four,
            Self::Four => Self::Five,
            Self::Five => Self::Six,
            Self::Six => Self::Seven,
            Self::Seven => Self::Eight,
            Self::Eight => Self::Nine,
            Self::Nine => Self::Ten,
            Self::Ten => Self::Jack,
            Self::Jack => Self::Queen,
            Self::Queen => Self::King,
            Self::King => Self::Ace,
            Self::Ace => Self::Two,
        }
    }

    pub fn get_value(&self) -> u8 {
        match self {
            Self::Two => 2,
            Self::Three => 3,
            Self::Four => 4,
            Self::Five => 5,
            Self::Six => 6,
            Self::Seven => 7,
            Self::Eight => 8,
            Self::Nine => 9,
            Self::Ten | Self::Jack | Self::Queen | Self::King => 10,
            Self::Ace => 11,
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash, sqlx::Type)]
#[sqlx(type_name = "card_enhancement", rename_all = "lowercase")]
pub enum Enhancement {
    Bonus,
    Mult,
    Wild,
    Glass,
    Steel,
    Stone,
    Gold,
    Lucky,
}

impl Display for Enhancement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Bonus => write!(f, "bonus"),
            Self::Mult => write!(f, "mult"),
            Self::Wild => write!(f, "wild"),
            Self::Glass => write!(f, "glass"),
            Self::Steel => write!(f, "steel"),
            Self::Stone => write!(f, "stone"),
            Self::Gold => write!(f, "gold"),
            Self::Lucky => write!(f, "lucky"),
        }
    }
}

impl Enhancement {
    fn on_scored(&self) -> ScoreModification {
        todo!("implement")
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash, sqlx::Type)]
#[sqlx(type_name = "card_seal", rename_all = "lowercase")]
pub enum Seal {
    Red,
    Purple,
    Blue,
    Gold,
}

impl Display for Seal {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Red => write!(f, "red"),
            Self::Purple => write!(f, "purple"),
            Self::Blue => write!(f, "blue"),
            Self::Gold => write!(f, "gold"),
        }
    }
}
