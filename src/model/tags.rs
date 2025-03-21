use std::fmt::Display;

use super::State;

#[derive(Debug)]
pub enum Tag {
    Uncommon(bool),
    Rare(bool),
    Negative(bool),
    Foil(bool),
    Holographic(bool),
    Polychrome(bool),
    Investment(bool),
    Voucher(bool),
    Boss(bool),
    Standard(bool),
    Charm(bool),
    Meteor(bool),
    Buffoon(bool),
    Handy(bool),
    Garbage(bool),
    Ethereal(bool),
    Coupon(bool),
    Double(bool),
    Juggle(bool),
    D6(bool),
    TopUp(bool),
    Speed(bool),
    Orbital(bool),
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
            Self::Uncommon(_) => "Uncommon",
            Self::Rare(_) => "Rare",
            Self::Negative(_) => "Negative",
            Self::Foil(_) => "Foil",
            Self::Holographic(_) => "Holographic",
            Self::Polychrome(_) => "Polychrome",
            Self::Investment(_) => "Investment",
            Self::Voucher(_) => "Voucher",
            Self::Boss(_) => "Boss",
            Self::Standard(_) => "Standard",
            Self::Charm(_) => "Charm",
            Self::Meteor(_) => "Meteor",
            Self::Buffoon(_) => "Buffoon",
            Self::Handy(_) => "Handy",
            Self::Garbage(_) => "Garbage",
            Self::Ethereal(_) => "Ethereal",
            Self::Coupon(_) => "Coupon",
            Self::Double(_) => "Double",
            Self::Juggle(_) => "Juggle",
            Self::D6(_) => "D6",
            Self::TopUp(_) => "Top-up",
            Self::Speed(_) => "Speed",
            Self::Orbital(_) => "Orbital",
        }
    }
}
