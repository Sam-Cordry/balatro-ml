use jokers::Joker;
use rand::{
    distr::{Distribution, StandardUniform},
    Rng,
};
use std::fmt::Display;
use strum::EnumIter;

use crate::model::{
    cards::Card,
    scoring::{Score, Scoring},
    traits::Consumable,
};

pub mod blinds;
pub mod cards;
pub mod db;
pub mod jokers;
pub mod planets;
pub mod scoring;
pub mod spectrals;
pub mod tags;
pub mod tarots;
pub mod traits;

pub struct State {
    pub scoring: Scoring,
    pub last_tarot_planet_used: Box<dyn Consumable>,
    pub selected_cards: Vec<Card>,
    pub consumables: Vec<Box<dyn Consumable>>,
    pub consumable_slots: usize,
    pub money: usize,
    pub hand: Vec<Card>,
    pub hand_size: usize,
    pub deck: Vec<Card>,
    pub remaining_deck: Vec<Card>,
    pub current_score: Score,
    pub hand_type_played: HandType,
    pub played_hand: Vec<Card>,
    pub scoring_cards: Vec<Card>,
    pub jokers: Vec<Joker>,
    pub joker_slots: usize,
    pub hands: usize,
    pub hands_remaining: usize,
    pub discards: usize,
    pub discards_remaining: usize,
    pub poker_hands_played: Vec<HandType>,
    pub tarots_used: usize,
    pub unique_planets_used: usize,
    pub redeemed_vouchers: Vec<Voucher>,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash, sqlx::Type)]
#[sqlx(type_name = "joker_edition", rename_all = "lowercase")]
pub enum JokerEdition {
    Base,
    Foil,
    Holographic,
    Polychrome,
    Negative,
}

impl Display for JokerEdition {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Base => write!(f, "base"),
            Self::Foil => write!(f, "foil"),
            Self::Holographic => write!(f, "holographic"),
            Self::Polychrome => write!(f, "polychrome"),
            Self::Negative => write!(f, "negative"),
        }
    }
}

impl JokerEdition {
    pub fn on_scored(&self) -> ScoreModification {
        todo!("implement")
    }
}

#[derive(Debug, PartialEq, Eq, Hash, EnumIter)]
pub enum HandType {
    High,
    Pair,
    TwoPair,
    ThreeOfAKind,
    Straight,
    Flush,
    FullHouse,
    FourOfAKind,
    StraightFlush,
    FiveOfAKind,
    FlushHouse,
    FlushFive,
}

impl Display for HandType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::High => write!(f, "high card"),
            Self::Pair => write!(f, "pair"),
            Self::TwoPair => write!(f, "two pair"),
            Self::ThreeOfAKind => write!(f, "three of a kind"),
            Self::Straight => write!(f, "straight"),
            Self::Flush => write!(f, "flush"),
            Self::FullHouse => write!(f, "full house"),
            Self::FourOfAKind => write!(f, "four of a kind"),
            Self::StraightFlush => write!(f, "stright flush"),
            Self::FiveOfAKind => write!(f, "five of a kind"),
            Self::FlushHouse => write!(f, "flush house"),
            Self::FlushFive => write!(f, "flush five"),
        }
    }
}

impl Distribution<HandType> for StandardUniform {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> HandType {
        match rng.random_range(0..12) {
            0 => HandType::High,
            1 => HandType::Pair,
            2 => HandType::TwoPair,
            3 => HandType::ThreeOfAKind,
            4 => HandType::Straight,
            5 => HandType::Flush,
            6 => HandType::FullHouse,
            7 => HandType::FourOfAKind,
            8 => HandType::StraightFlush,
            9 => HandType::FiveOfAKind,
            10 => HandType::FlushHouse,
            11 => HandType::FlushFive,
            _ => {
                panic!("Random range produced out of range result");
            }
        }
    }
}

#[derive(Debug)]
pub enum Voucher {
    Overstock,
    OverstockPlus,
    ClearanceSale,
    Liquidation,
    Hone,
    GlowUp,
    RerollSurplus,
    RerollGlut,
    CrystalBall,
    OmenGlobe,
    Telescope,
    Observatory,
    Grabber,
    NachoTong,
    Wasteful,
    Recyclomancy,
    TarotMerchant,
    TarotTycoon,
    PlanetMerchant,
    PlanetTycoon,
    SeedMoney,
    MoneyTree,
    Blank,
    Antimatter,
    MagicTrick,
    Illusion,
    Hieroglyph,
    Petroglyph,
    DirectorsCut,
    Retcon,
    PaintBrush,
    Palette,
}

impl Display for Voucher {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Overstock => write!(f, "Overstock"),
            Self::OverstockPlus => write!(f, "Overstock Plus"),
            Self::ClearanceSale => write!(f, "Clearance Sale"),
            Self::Liquidation => write!(f, "Liquidation"),
            Self::Hone => write!(f, "Hone"),
            Self::GlowUp => write!(f, "Glow Up"),
            Self::RerollSurplus => write!(f, "Reroll Surplus"),
            Self::RerollGlut => write!(f, "Reroll Glut"),
            Self::CrystalBall => write!(f, "Crystall Ball"),
            Self::OmenGlobe => write!(f, "Omen Globe"),
            Self::Telescope => write!(f, "Telescope"),
            Self::Observatory => write!(f, "Observatory"),
            Self::Grabber => write!(f, "Grabber"),
            Self::NachoTong => write!(f, "Nacho Tong"),
            Self::Wasteful => write!(f, "Wasteful"),
            Self::Recyclomancy => write!(f, "Recyclomancy"),
            Self::TarotMerchant => write!(f, "Tarot Merchant"),
            Self::TarotTycoon => write!(f, "Tarot Tycoon"),
            Self::PlanetMerchant => write!(f, "Planet Merchant"),
            Self::PlanetTycoon => write!(f, "Planet Tycoon"),
            Self::SeedMoney => write!(f, "Seed Money"),
            Self::MoneyTree => write!(f, "Money Tree"),
            Self::Blank => write!(f, "Blank"),
            Self::Antimatter => write!(f, "Antimatter"),
            Self::MagicTrick => write!(f, "Magic Trick"),
            Self::Illusion => write!(f, "Illusion"),
            Self::Hieroglyph => write!(f, "Hieroglyph"),
            Self::Petroglyph => write!(f, "Petroglyph"),
            Self::DirectorsCut => write!(f, "Director's Cut"),
            Self::Retcon => write!(f, "Retcon"),
            Self::PaintBrush => write!(f, "Paint Brush"),
            Self::Palette => write!(f, "Palette"),
        }
    }
}

pub enum PackType {
    Normal,
    Jumbo,
    Mega,
}

pub struct ScoreModification {
    pub chips: isize,
    pub mult: isize,
    pub xmult: f64,
    pub money: isize,
    pub triggers: isize,
}

impl Default for ScoreModification {
    fn default() -> Self {
        Self {
            chips: 0,
            mult: 0,
            xmult: 1.0,
            money: 0,
            triggers: 1,
        }
    }
}

impl ScoreModification {
    pub fn add(&mut self, other: ScoreModification) {
        self.chips += other.chips;
        self.mult += other.mult;
        self.xmult *= other.xmult;
        self.money += other.money;
        self.triggers += other.triggers;
    }
}
