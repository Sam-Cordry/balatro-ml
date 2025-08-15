use rand::distr::{Distribution, StandardUniform};
use std::fmt::Display;
use strum::EnumIter;

use crate::model::{cards::Card, Consumable, HandType, State};

#[derive(Debug, PartialEq, Eq, EnumIter, Hash)]
pub enum Planet {
    Pluto(bool),
    Mercury(bool),
    Uranus(bool),
    Venus(bool),
    Saturn(bool),
    Jupiter(bool),
    Earth(bool),
    Mars(bool),
    Neptune(bool),
    PlanetX(bool),
    Eris(bool),
    Ceres(bool),
}

impl Display for Planet {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Pluto(negative) => write!(f, "Pluto ({negative})"),
            Self::Mercury(negative) => write!(f, "Mercury ({negative})"),
            Self::Uranus(negative) => write!(f, "Uranus ({negative})"),
            Self::Venus(negative) => write!(f, "Venus ({negative})"),
            Self::Saturn(negative) => write!(f, "Saturn ({negative})"),
            Self::Jupiter(negative) => write!(f, "Jupiter ({negative})"),
            Self::Earth(negative) => write!(f, "Earth ({negative})"),
            Self::Mars(negative) => write!(f, "Mars ({negative})"),
            Self::Neptune(negative) => write!(f, "Neptune ({negative})"),
            Self::PlanetX(negative) => write!(f, "Planet X ({negative})"),
            Self::Eris(negative) => write!(f, "Eris ({negative})"),
            Self::Ceres(negative) => write!(f, "Ceres ({negative})"),
        }
    }
}

impl Consumable for Planet {
    fn can_use(&self, _state: &State, _selected_cards: &mut [Card]) -> bool {
        true
    }

    fn consume(&self, state: &mut State, _selected_cards: &mut [Card]) {
        let hand_type: HandType = match self {
            Self::Pluto { .. } => HandType::High,
            Self::Mercury { .. } => HandType::Pair,
            Self::Uranus { .. } => HandType::TwoPair,
            Self::Venus { .. } => HandType::ThreeOfAKind,
            Self::Saturn { .. } => HandType::Straight,
            Self::Jupiter { .. } => HandType::Flush,
            Self::Earth { .. } => HandType::FullHouse,
            Self::Mars { .. } => HandType::FourOfAKind,
            Self::Neptune { .. } => HandType::StraightFlush,
            Self::PlanetX { .. } => HandType::FiveOfAKind,
            Self::Eris { .. } => HandType::FlushHouse,
            Self::Ceres { .. } => HandType::FlushFive,
        };
        state.scoring.level_hand(&hand_type, 1);
    }

    fn is_negative(&self) -> bool {
        match self {
            Self::Pluto(negative)
            | Self::Mercury(negative)
            | Self::Uranus(negative)
            | Self::Venus(negative)
            | Self::Saturn(negative)
            | Self::Jupiter(negative)
            | Self::Earth(negative)
            | Self::Mars(negative)
            | Self::Neptune(negative)
            | Self::PlanetX(negative)
            | Self::Eris(negative)
            | Self::Ceres(negative) => *negative,
        }
    }

    fn make_negative(&mut self) {
        match *self {
            Self::Pluto(ref mut negative)
            | Self::Mercury(ref mut negative)
            | Self::Venus(ref mut negative)
            | Self::Uranus(ref mut negative)
            | Self::Saturn(ref mut negative)
            | Self::Jupiter(ref mut negative)
            | Self::Earth(ref mut negative)
            | Self::Mars(ref mut negative)
            | Self::Neptune(ref mut negative)
            | Self::PlanetX(ref mut negative)
            | Self::Eris(ref mut negative)
            | Self::Ceres(ref mut negative) => {
                *negative = true;
            }
        }
    }

    fn name(&self) -> &'static str {
        match self {
            Self::Pluto(_) => "Pluto",
            Self::Mercury(_) => "Mercury",
            Self::Uranus(_) => "Uranus",
            Self::Venus(_) => "Venus",
            Self::Saturn(_) => "Saturn",
            Self::Jupiter(_) => "Jupiter",
            Self::Earth(_) => "Earth",
            Self::Mars(_) => "Mars",
            Self::Neptune(_) => "Neptune",
            Self::PlanetX(_) => "Planet X",
            Self::Eris(_) => "Eris",
            Self::Ceres(_) => "Ceres",
        }
    }
}

impl Planet {
    pub fn sample<R: rand::Rng + ?Sized>(rng: &mut R) -> Self {
        match rng.random_range(0..12) {
            0 => Self::Pluto(false),
            1 => Self::Mercury(false),
            2 => Self::Uranus(false),
            3 => Self::Venus(false),
            4 => Self::Saturn(false),
            5 => Self::Jupiter(false),
            6 => Self::Earth(false),
            7 => Self::Mars(false),
            8 => Self::Neptune(false),
            9 => Self::PlanetX(false),
            10 => Self::Eris(false),
            _ => Self::Ceres(false),
        }
    }
}
