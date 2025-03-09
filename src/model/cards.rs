use std::fmt::Display;

use super::Edition;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct Card {
    pub suit: Suit,
    pub value: Value,
    pub edition: Edition,
    pub enhancement: Enhancement,
    pub seal: Seal,
}

impl Display for Card {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{} {} {} of {}s with {} seal",
            self.edition, self.enhancement, self.value, self.suit, self.seal
        )
    }
}

impl Card {
    pub fn increment(&mut self) {
        self.value = self.value.increment();
    }

    pub fn duplicate(&mut self, other: &Card) {
        self.suit = other.suit;
        self.value = other.value;
        self.edition = other.edition;
        self.enhancement = other.enhancement;
        self.seal = other.seal;
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
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

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Value {
    Ace,
    King,
    Queen,
    Jack,
    Ten,
    Nine,
    Eight,
    Seven,
    Six,
    Five,
    Four,
    Three,
    Two,
}

impl Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Ace => write!(f, "Ace"),
            Self::King => write!(f, "King"),
            Self::Queen => write!(f, "Queen"),
            Self::Jack => write!(f, "Jack"),
            Self::Ten => write!(f, "Ten"),
            Self::Nine => write!(f, "Nine"),
            Self::Eight => write!(f, "Eight"),
            Self::Seven => write!(f, "Seven"),
            Self::Six => write!(f, "Six"),
            Self::Five => write!(f, "Five"),
            Self::Four => write!(f, "Four"),
            Self::Three => write!(f, "Three"),
            Self::Two => write!(f, "Two"),
        }
    }
}

impl Value {
    fn increment(&self) -> Self {
        match self {
            Self::Ace => Self::Two,
            Self::King => Self::Ace,
            Self::Queen => Self::King,
            Self::Jack => Self::Queen,
            Self::Ten => Self::Jack,
            Self::Nine => Self::Ten,
            Self::Eight => Self::Nine,
            Self::Seven => Self::Eight,
            Self::Six => Self::Seven,
            Self::Five => Self::Six,
            Self::Four => Self::Five,
            Self::Three => Self::Four,
            Self::Two => Self::Three,
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
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

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
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
