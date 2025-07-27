use serde::{Deserialize, Serialize};
use std::fmt::Display;

use crate::model::State;

#[derive(Debug, sqlx::Type, Serialize, Deserialize)]
#[sqlx(type_name = "tag")]
pub enum Tag {
    Uncommon,
    Rare,
    Negative,
    Foil,
    Holographic,
    Polychrome,
    Investment,
    Voucher,
    Boss,
    Standard,
    Charm,
    Meteor,
    Buffoon,
    Handy,
    Garbage,
    Ethereal,
    Coupon,
    Double,
    Juggle,
    D6,
    #[sqlx(rename = "Top-up")]
    TopUp,
    Speed,
    Orbital,
}

impl Display for Tag {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} Tag", self.name())
    }
}

impl Tag {
    pub fn resolve(&self, state: &mut State) {
        todo!("implement")
    }

    pub fn name(&self) -> &'static str {
        match self {
            Self::Uncommon => "Uncommon",
            Self::Rare => "Rare",
            Self::Negative => "Negative",
            Self::Foil => "Foil",
            Self::Holographic => "Holographic",
            Self::Polychrome => "Polychrome",
            Self::Investment => "Investment",
            Self::Voucher => "Voucher",
            Self::Boss => "Boss",
            Self::Standard => "Standard",
            Self::Charm => "Charm",
            Self::Meteor => "Meteor",
            Self::Buffoon => "Buffoon",
            Self::Handy => "Handy",
            Self::Garbage => "Garbage",
            Self::Ethereal => "Ethereal",
            Self::Coupon => "Coupon",
            Self::Double => "Double",
            Self::Juggle => "Juggle",
            Self::D6 => "D6",
            Self::TopUp => "Top-up",
            Self::Speed => "Speed",
            Self::Orbital => "Orbital",
        }
    }
}
