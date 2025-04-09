use std::{cmp::max, collections::HashSet, fmt::Display};

use crate::model::{
    cards::{Card, Enhancement, Rank, Suit},
    db::{JokerRow, JokerType},
    HandType, JokerEdition, ScoreModification, State,
};

#[derive(Debug, PartialEq, Eq)]
pub enum Joker {
    Joker {
        sell_value: usize,
        edition: JokerEdition,
    },
    Greedy {
        sell_value: usize,
        edition: JokerEdition,
    },
    Lusty {
        sell_value: usize,
        edition: JokerEdition,
    },
    Wrathful {
        sell_value: usize,
        edition: JokerEdition,
    },
    Gluttonous {
        sell_value: usize,
        edition: JokerEdition,
    },
    Jolly {
        sell_value: usize,
        edition: JokerEdition,
    },
    Zany {
        sell_value: usize,
        edition: JokerEdition,
    },
    Mad {
        sell_value: usize,
        edition: JokerEdition,
    },
    Crazy {
        sell_value: usize,
        edition: JokerEdition,
    },
    Droll {
        sell_value: usize,
        edition: JokerEdition,
    },
    Sly {
        sell_value: usize,
        edition: JokerEdition,
    },
    Wily {
        sell_value: usize,
        edition: JokerEdition,
    },
    Clever {
        sell_value: usize,
        edition: JokerEdition,
    },
    Devious {
        sell_value: usize,
        edition: JokerEdition,
    },
    Crafty {
        sell_value: usize,
        edition: JokerEdition,
    },
    Half {
        sell_value: usize,
        edition: JokerEdition,
    },
    Stencil {
        sell_value: usize,
        edition: JokerEdition,
    },
    Fingers {
        sell_value: usize,
        edition: JokerEdition,
    },
    Mime {
        sell_value: usize,
        edition: JokerEdition,
    },
    Credit {
        sell_value: usize,
        edition: JokerEdition,
    },
    Dagger {
        sell_value: usize,
        edition: JokerEdition,
        mult: usize,
    },
    Banner {
        sell_value: usize,
        edition: JokerEdition,
    },
    Mystic {
        sell_value: usize,
        edition: JokerEdition,
    },
    Marble {
        sell_value: usize,
        edition: JokerEdition,
    },
    Loyalty {
        sell_value: usize,
        edition: JokerEdition,
        hands: u8,
    },
    Ball {
        sell_value: usize,
        edition: JokerEdition,
    },
    Misprint {
        sell_value: usize,
        edition: JokerEdition,
    },
    Dusk {
        sell_value: usize,
        edition: JokerEdition,
    },
    Fist {
        sell_value: usize,
        edition: JokerEdition,
        min: Option<Rank>,
    },
    Chaos {
        sell_value: usize,
        edition: JokerEdition,
    },
    Fibonacci {
        sell_value: usize,
        edition: JokerEdition,
    },
    Steel {
        sell_value: usize,
        edition: JokerEdition,
    },
    Scary {
        sell_value: usize,
        edition: JokerEdition,
    },
    Abstract {
        sell_value: usize,
        edition: JokerEdition,
    },
    Gratification {
        sell_value: usize,
        edition: JokerEdition,
    },
    Hack {
        sell_value: usize,
        edition: JokerEdition,
    },
    Pareidolia {
        sell_value: usize,
        edition: JokerEdition,
    },
    Michel {
        sell_value: usize,
        edition: JokerEdition,
    },
    Steven {
        sell_value: usize,
        edition: JokerEdition,
    },
    Todd {
        sell_value: usize,
        edition: JokerEdition,
    },
    Scholar {
        sell_value: usize,
        edition: JokerEdition,
    },
    Business {
        sell_value: usize,
        edition: JokerEdition,
    },
    Supernova {
        sell_value: usize,
        edition: JokerEdition,
    },
    Bus {
        sell_value: usize,
        edition: JokerEdition,
        mult: usize,
    },
    Space {
        sell_value: usize,
        edition: JokerEdition,
    },
    Egg {
        sell_value: usize,
        edition: JokerEdition,
    },
    Burglar {
        sell_value: usize,
        edition: JokerEdition,
    },
    Blackboard {
        sell_value: usize,
        edition: JokerEdition,
    },
    Runner {
        sell_value: usize,
        edition: JokerEdition,
        chips: usize,
    },
    Cream {
        sell_value: usize,
        edition: usize,
        chips: usize,
    },
    DNA {
        sell_value: usize,
        edition: JokerEdition,
    },
    Splash {
        sell_value: usize,
        edition: JokerEdition,
    },
    Blue {
        sell_value: usize,
        edition: JokerEdition,
    },
    Sixth {
        sell_value: usize,
        edition: JokerEdition,
    },
    Constellation {
        sell_value: usize,
        edition: JokerEdition,
        mult: usize,
    },
    Hiker {
        sell_value: usize,
        edition: JokerEdition,
    },
    Faceless {
        sell_value: usize,
        edition: JokerEdition,
    },
    Green {
        sell_value: usize,
        edition: JokerEdition,
        mult: usize,
    },
    Superposition {
        sell_value: usize,
        edition: JokerEdition,
    },
    List {
        sell_value: usize,
        edition: JokerEdition,
        hand_type: HandType,
    },
    Cavendish {
        sell_value: usize,
        edition: JokerEdition,
    },
    Sharp {
        sell_value: usize,
        edition: JokerEdition,
    },
    Red {
        sell_value: usize,
        edition: JokerEdition,
        mult: usize,
    },
    Madness {
        sell_value: usize,
        edition: JokerEdition,
        xmult: usize,
    },
    Square {
        sell_value: usize,
        edition: JokerEdition,
        chips: usize,
    },
    Seance {
        sell_value: usize,
        edition: JokerEdition,
    },
    Riff {
        sell_value: usize,
        edition: JokerEdition,
    },
    Vampire {
        sell_value: usize,
        edition: JokerEdition,
        xmult: usize,
    },
    Shortcut {
        sell_value: usize,
        edition: JokerEdition,
    },
    Hologram {
        sell_value: usize,
        edition: JokerEdition,
        xmult: usize,
    },
    Vagabond {
        sell_value: usize,
        ediiton: JokerEdition,
    },
    Baron {
        sell_value: usize,
        edition: JokerEdition,
    },
    Cloud {
        sell_value: usize,
        edition: JokerEdition,
    },
    Rocket {
        sell_value: usize,
        edition: JokerEdition,
        money: usize,
    },
    Obelisk {
        sell_value: usize,
        edition: JokerEdition,
        xmult: usize,
    },
    Midas {
        sell_value: usize,
        edition: JokerEdition,
    },
    Luchador {
        sell_value: usize,
        edition: JokerEdition,
    },
    Photograph {
        sell_value: usize,
        edition: JokerEdition,
        used: bool,
    },
    Gift {
        sell_value: usize,
        edition: JokerEdition,
    },
    Turtle {
        sell_value: usize,
        edition: JokerEdition,
        hand_size: u8,
    },
    Erosion {
        sell_value: usize,
        edition: JokerEdition,
    },
    Parking {
        sell_value: usize,
        edition: JokerEdition,
    },
    Rebate {
        sell_value: usize,
        edition: JokerEdition,
        rank: Rank,
    },
    Moon {
        sell_value: usize,
        edition: JokerEdition,
    },
    Hallucination {
        sell_value: usize,
        edition: JokerEdition,
    },
    Fortune {
        sell_value: usize,
        edition: JokerEdition,
    },
    Juggler {
        sell_value: usize,
        edition: JokerEdition,
    },
    Drunkard {
        sell_value: usize,
        edition: JokerEdition,
    },
    Stone {
        sell_value: usize,
        edition: JokerEdition,
    },
    Golden {
        sell_value: usize,
        edition: JokerEdition,
    },
    Lucky {
        sell_value: usize,
        edition: JokerEdition,
        xmult: usize,
    },
    Baseball {
        sell_value: usize,
        edition: JokerEdition,
    },
    Bull {
        sell_value: usize,
        edition: JokerEdition,
    },
    Cola {
        sell_value: usize,
        edition: JokerEdition,
    },
    Trading {
        sell_value: usize,
        edition: JokerEdition,
    },
    Flash {
        sell_value: usize,
        edition: JokerEdition,
        mult: usize,
    },
    Popcorn {
        sell_value: usize,
        edition: JokerEdition,
        mult: usize,
    },
    Trousers {
        sell_value: usize,
        edition: JokerEdition,
        mult: usize,
    },
    Ancient {
        sell_value: usize,
        edition: JokerEdition,
        suit: Suit,
    },
    Ramen {
        sell_value: usize,
        edition: JokerEdition,
        xmult: usize,
    },
    Walkie {
        sell_value: usize,
        edition: JokerEdition,
    },
    Seltzer {
        sell_value: usize,
        edition: JokerEdition,
    },
    Castle {
        sell_value: usize,
        edition: JokerEdition,
        suit: Suit,
        chips: usize,
    },
    Smiley {
        sell_value: usize,
        edition: JokerEdition,
    },
    Campfire {
        sell_value: usize,
        edition: JokerEdition,
        xmult: usize,
    },
    Ticket {
        sell_value: usize,
        edition: JokerEdition,
    },
    Bones {
        sell_value: usize,
        edition: JokerEdition,
    },
    Acrobat {
        sell_value: usize,
        edition: JokerEdition,
    },
    Sock {
        sell_value: usize,
        edition: JokerEdition,
    },
    Swashbuckler {
        sell_value: usize,
        edition: JokerEdition,
    },
    Troubadour {
        sell_value: usize,
        edition: JokerEdition,
    },
    Certificate {
        sell_value: usize,
        edition: JokerEdition,
    },
    Smeared {
        sell_value: usize,
        edition: JokerEdition,
    },
    Throwback {
        sell_value: usize,
        edition: JokerEdition,
        xmult: usize,
    },
    Chad {
        sell_value: usize,
        edition: JokerEdition,
    },
    Gem {
        sell_value: usize,
        edition: JokerEdition,
    },
    Bloodstone {
        sell_value: usize,
        edition: JokerEdition,
    },
    Arrowhead {
        sell_value: usize,
        edition: JokerEdition,
    },
    Onyx {
        sell_value: usize,
        edition: JokerEdition,
    },
    Glass {
        sell_value: usize,
        edition: JokerEdition,
        xmult: usize,
    },
    Showman {
        sell_value: usize,
        edition: JokerEdition,
    },
    Flower {
        sell_value: usize,
        edition: JokerEdition,
    },
    Blueprint {
        sell_value: usize,
        edition: JokerEdition,
    },
    Wee {
        sell_value: usize,
        edition: JokerEdition,
        chips: usize,
    },
    Andy {
        sell_value: usize,
        edition: JokerEdition,
    },
    Oops {
        sell_value: usize,
        edition: JokerEdition,
    },
    Idol {
        sell_value: usize,
        edition: JokerEdition,
        rank: Rank,
        suit: Suit,
    },
    Double {
        sell_value: usize,
        edition: JokerEdition,
    },
    Matador {
        sell_value: usize,
        edition: JokerEdition,
    },
    Road {
        sell_value: usize,
        edition: JokerEdition,
        xmult: usize,
    },
    Duo {
        sell_value: usize,
        edition: JokerEdition,
    },
    Trio {
        sell_value: usize,
        edition: JokerEdition,
    },
    Family {
        sell_value: usize,
        edition: JokerEdition,
    },
    Order {
        sell_value: usize,
        edition: JokerEdition,
    },
    Tribe {
        sell_value: usize,
        edition: JokerEdition,
    },
    Stuntman {
        sell_value: usize,
        edition: JokerEdition,
    },
    Invisible {
        sell_value: usize,
        edition: JokerEdition,
        rounds: usize,
    },
    Brainstorm {
        sell_value: usize,
        edition: JokerEdition,
    },
    Satellite {
        sell_value: usize,
        edition: JokerEdition,
    },
    Shoot {
        sell_value: usize,
        edition: JokerEdition,
    },
    License {
        sell_value: usize,
        edition: JokerEdition,
    },
    Cartomancer {
        sell_value: usize,
        edition: JokerEdition,
    },
    Astronomer {
        sell_value: usize,
        edition: JokerEdition,
    },
    Burnt {
        sell_value: usize,
        edition: JokerEdition,
    },
    Bootstraps {
        sell_value: usize,
        edition: JokerEdition,
    },
    Canio {
        sell_value: usize,
        edition: JokerEdition,
        xmult: usize,
    },
    Triboulet {
        sell_value: usize,
        edition: JokerEdition,
    },
    Yorick {
        sell_value: usize,
        edition: JokerEdition,
        discards: usize,
        xmult: usize,
    },
    Chicot {
        sell_value: usize,
        edition: JokerEdition,
    },
    Perkeo {
        sell_value: usize,
        edition: JokerEdition,
    },
}

impl Display for Joker {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name())
    }
}

impl From<JokerRow> for Joker {
    fn from(value: JokerRow) -> Self {
        match value.joker {
            JokerType::Joker => Self::Joker {
                sell_value: value.sell_value as usize,
                edition: value.edition,
            },
            JokerType::Greedy => Self::Greedy {
                sell_value: value.sell_value as usize,
                edition: value.edition,
            },
            JokerType::Lusty => Self::Lusty {
                sell_value: value.sell_value as usize,
                edition: value.edition,
            },
            JokerType::Wrathful => Self::Wrathful {
                sell_value: value.sell_value as usize,
                edition: value.edition,
            },
            JokerType::Gluttonous => Self::Gluttonous {
                sell_value: value.sell_value as usize,
                edition: value.edition,
            },
            JokerType::Jolly => Self::Jolly {
                sell_value: value.sell_value as usize,
                edition: value.edition,
            },
            JokerType::Zany => Self::Zany {
                sell_value: value.sell_value as usize,
                edition: value.edition,
            },
            JokerType::Mad => Self::Mad {
                sell_value: value.sell_value as usize,
                edition: value.edition,
            },
            JokerType::Crazy => Self::Crazy {
                sell_value: value.sell_value as usize,
                edition: value.edition,
            },
            JokerType::Droll => Self::Droll {
                sell_value: value.sell_value as usize,
                edition: value.edition,
            },
            JokerType::Sly => Self::Sly {
                sell_value: value.sell_value as usize,
                edition: value.edition,
            },
            JokerType::Wily => Self::Wily {
                sell_value: value.sell_value as usize,
                edition: value.edition,
            },
            JokerType::Clever => Self::Clever {
                sell_value: value.sell_value as usize,
                edition: value.edition,
            },
            JokerType::Devious => Self::Devious {
                sell_value: value.sell_value as usize,
                edition: value.edition,
            },
            JokerType::Half => Self::Half {
                sell_value: value.sell_value as usize,
                edition: value.edition,
            },
            JokerType::Stencil => Self::Stencil {
                sell_value: value.sell_value as usize,
                edition: value.edition,
            },
            JokerType::Fingers => Self::Fingers {
                sell_value: value.sell_value as usize,
                edition: value.edition,
            },
            JokerType::Mime => Self::Mime {
                sell_value: value.sell_value as usize,
                edition: value.edition,
            },
            JokerType::Credit => Self::Credit {
                sell_value: value.sell_value as usize,
                edition: value.edition,
            },
            JokerType::Dagger => Self::Dagger {
                sell_value: value.sell_value as usize,
                edition: value.edition,
                mult: value.mult.unwrap() as usize,
            },
            JokerType::Banner => Self::Banner {
                sell_value: value.sell_value as usize,
                edition: value.edition,
            },
            JokerType::Mystic => Self::Mystic {
                sell_value: value.sell_value as usize,
                edition: value.edition,
            },
            JokerType::Marble => Self::Marble {
                sell_value: value.sell_value as usize,
                edition: value.edition,
            },
            JokerType::Loyalty => Self::Loyalty {
                sell_value: value.sell_value as usize,
                edition: value.edition,
                hands: value.hands.unwrap() as usize,
            },
            JokerType::Ball => Self::Ball {
                sell_value: value.sell_value as usize,
                edition: value.edition,
            },
            JokerType::Misprint => Self::Misprint {
                sell_value: value.sell_value as usize,
                edition: value.edition,
            },
            JokerType::Dusk => Self::Dusk {
                sell_value: value.sell_value as usize,
                edition: value.edition,
            },
            JokerType::Fist => Self::Fist {
                sell_value: value.sell_value as usize,
                edition: value.edition,
            },
            JokerType::Chaos => Self::Chaos {
                sell_value: value.sell_value as usize,
                edition: value.edition,
            },
            JokerType::Fibonacci => Self::Fibonacci {
                sell_value: value.sell_value as usize,
                edition: value.edition,
            },
            JokerType::Steel => Self::Steel {
                sell_value: value.sell_value as usize,
                edition: value.edition,
            },
            JokerType::Scary => Self::Scary {
                sell_value: value.sell_value as usize,
                edition: value.edition,
            },
            JokerType::Abstract => Self::Abstract {
                sell_value: value.sell_value as usize,
                edition: value.edition,
            },
            JokerType::Gratification => Self::Gratification {
                sell_value: value.sell_value as usize,
                edition: value.edition,
            },
            JokerType::Hack => Self::Hack {
                sell_value: value.sell_value as usize,
                edition: value.edition,
            },
            JokerType::Pareidolia => Self::Pareidolia {
                sell_value: value.sell_value as usize,
                edition: value.edition,
            },
            JokerType::Michel => Self::Michel {
                sell_value: value.sell_value as usize,
                edition: value.edition,
            },
            JokerType::Steven => Self::Steven {
                sell_value: value.sell_value as usize,
                edition: value.edition,
            },
            JokerType::Todd => Self::Todd {
                sell_value: value.sell_value as usize,
                edition: value.edition,
            },
            JokerType::Scholar => Self::Scholar {
                sell_value: value.sell_value as usize,
                edition: value.edition,
            },
            JokerType::Business => Self::Business {
                sell_value: value.sell_value as usize,
                edition: value.edition,
            },
            JokerType::Supernova => Self::Supernova {
                sell_value: value.sell_value as usize,
                edition: value.edition,
            },
            JokerType::Bus => Self::Bus {
                sell_value: value.sell_value as usize,
                edition: value.edition,
                mult: value.mult.unwrap(),
            },
            JokerType::Space => Self::Space {
                sell_value: value.sell_value as usize,
                edition: value.edition,
            },
            JokerType::Egg => Self::Egg {
                sell_value: value.sell_value as usize,
                edition: value.edition,
            },
            JokerType::Burglar => Self::Burglar {
                sell_value: value.sell_value as usize,
                edition: value.edition,
            },
            JokerType::Blackboard => Self::Blackboard {
                sell_value: value.sell_value as usize,
                edition: value.edition,
            },
            JokerType::Runner => Self::Runner {
                sell_value: value.sell_value as usize,
                edition: value.edition,
                chips: value.chips.unwrap(),
            },
            JokerType::Cream => Self::Cream {
                sell_value: value.sell_value as usize,
                edition: value.edition,
                chips: value.chips.unwrap(),
            },
            JokerType::DNA => Self::DNA {
                sell_value: value.sell_value as usize,
                edition: value.edition,
            },
            JokerType::Splash => Self::Splash {
                sell_value: value.sell_value as usize,
                edition: value.edition,
            },
            JokerType::Blue => Self::Blue {
                sell_value: value.sell_value as usize,
                edition: value.edition,
            },
            JokerType::Sixth => Self::Sixth {
                sell_value: value.sell_value as usize,
                edition: value.edition,
            },
            JokerType::Constellation => Self::Constellation {
                sell_value: value.sell_value as usize,
                edition: value.edition,
                xmult: value.xmult.unwrap() as usize,
            },
            JokerType::Hiker => Self::Hiker {
                sell_value: value.sell_value as usize,
                edition: value.edition,
            },
            JokerType::Faceless => Self::Faceless {
                sell_value: value.sell_value as usize,
                edition: value.edition,
            },
            JokerType::Green => Self::Green {
                sell_value: value.sell_value as usize,
                edition: value.edition,
                mult: value.mult.unwrap() as usize,
            },
            JokerType::Superposition => Self::Superposition {
                sell_value: value.sell_value as usize,
                edition: value.edition,
            },
            JokerType::List => Self::List {
                sell_value: value.sell_value as usize,
                edition: value.edition,
                hand_type: value.hand_type.unwrap() as usize,
            },
            JokerType::Cavendish => Self::Cavendish {
                sell_value: value.sell_value as usize,
                edition: value.edition,
            },
            JokerType::Sharp => Self::Sharp {
                sell_value: value.sell_value as usize,
                edition: value.edition,
            },
            JokerType::Red => Self::Red {
                sell_value: value.sell_value as usize,
                edition: value.edition,
                mult: value.mult.unwrap() as usize,
            },
            JokerType::Madness => Self::Madness {
                sell_value: value.sell_value as usize,
                edition: value.edition,
                xmult: value.xmult.unwrap() as usize,
            },
            JokerType::Square => Self::Square {
                sell_value: value.sell_value as usize,
                edition: value.edition,
                chips: value.chips.unwrap() as usize,
            },
            JokerType::Seance => Self::Seance {
                sell_value: value.sell_value as usize,
                edition: value.edition,
            },
            JokerType::Riff => Self::Riff {
                sell_value: value.sell_value as usize,
                edition: value.edition,
            },
            JokerType::Vampire => Self::Vampire {
                sell_value: value.sell_value as usize,
                edition: value.edition,
                xmult: value.xmult.unwrap() as usize,
            },
            JokerType::Shortcut => Self::Shortcut {
                sell_value: value.sell_value as usize,
                edition: value.edition,
            },
            JokerType::Hologram => Self::Hologram {
                sell_value: value.sell_value as usize,
                edition: value.edition,
                xmult: value.xmult.unwrap() as usize,
            },
            JokerType::Vagabond => Self::Vagabond {
                sell_value: value.sell_value as usize,
                edition: value.edition,
            },
            JokerType::Baron => Self::Baron {
                sell_value: value.sell_value as usize,
                edition: value.edition,
            },
            JokerType::Cloud => Self::Cloud {
                sell_value: value.sell_value as usize,
                edition: value.edition,
            },
            JokerType::Rocket => Self::Rocket {
                sell_value: value.sell_value as usize,
                edition: value.edition,
                money: value.money.unwrap() as usize,
            },
            JokerType::Obelisk => Self::Obelisk {
                sell_value: value.sell_value as usize,
                edition: value.edition,
                xmult: value.xmult.unwrap() as usize,
            },
            JokerType::Midas => Self::Midas {
                sell_value: value.sell_value as usize,
                edition: value.edition,
            },
            JokerType::Luchador => Self::Luchador {
                sell_value: value.sell_value as usize,
                edition: value.edition,
            },
            JokerType::Photograph => Self::Photograph {
                sell_value: value.sell_value as usize,
                edition: value.edition,
            },
            JokerType::Gift => Self::Gift {
                sell_value: value.sell_value as usize,
                edition: value.edition,
            },
            JokerType::Turtle => Self::Turtle {
                sell_value: value.sell_value as usize,
                edition: value.edition,
                hand_size: value.hand_size.unwrap() as usize,
            },
            JokerType::Erosion => Self::Erosion {
                sell_value: value.sell_value as usize,
                edition: value.edition,
                mult: value.mult.unwrap() as usize,
            },
            JokerType::Parking => Self::Parking {
                sell_value: value.sell_value as usize,
                edition: value.edition,
            },
            JokerType::Rebate => Self::Rebate {
                sell_value: value.sell_value as usize,
                edition: value.edition,
                rank: value.rank.unwrap(),
            },
            JokerType::Moon => Self::Moon {
                sell_value: value.sell_value as usize,
                edition: value.edition,
            },
            JokerType::Hallucination => Self::Hallucination {
                sell_value: value.sell_value as usize,
                edition: value.edition,
            },
            JokerType::Fortune => Self::Fortune {
                sell_value: value.sell_value as usize,
                edition: value.edition,
            },
            JokerType::Juggler => Self::Juggler {
                sell_value: value.sell_value as usize,
                edition: value.edition,
            },
            JokerType::Drunkard => Self::Drunkard {
                sell_value: value.sell_value as usize,
                edition: value.edition,
            },
            JokerType::Stone => Self::Stone {
                sell_value: value.sell_value as usize,
                edition: value.edition,
            },
            JokerType::Golden => Self::Golden {
                sell_value: value.sell_value as usize,
                edition: value.edition,
            },
            JokerType::Lucky => Self::Lucky {
                sell_value: value.sell_value as usize,
                edition: value.edition,
                xmult: value.xmult.unwrap() as usize,
            },
            JokerType::Baseball => Self::Baseball {
                sell_value: value.sell_value as usize,
                edition: value.edition,
            },
            JokerType::Bull => Self::Bull {
                sell_value: value.sell_value as usize,
                edition: value.edition,
            },
            JokerType::Cola => Self::Cola {
                sell_value: value.sell_value as usize,
                edition: value.edition,
            },
            JokerType::Trading => Self::Trading {
                sell_value: value.sell_value as usize,
                edition: value.edition,
            },
            JokerType::Flash => Self::Flash {
                sell_value: value.sell_value as usize,
                edition: value.edition,
                mult: value.mult.unwrap() as usize,
            },
            JokerType::Popcorn => Self::Popcorn {
                sell_value: value.sell_value as usize,
                edition: value.edition,
                mult: value.mult.unwrap() as usize,
            },
            JokerType::Trousers => Self::Trousers {
                sell_value: value.sell_value as usize,
                edition: value.edition,
                mult: value.mult.unwrap() as usize,
            },
            JokerType::Ancient => Self::Ancient {
                sell_value: value.sell_value as usize,
                edition: value.edition,
                suit: value.suit.unwrap() as usize,
            },
            JokerType::Ramen => Self::Ramen {
                sell_value: value.sell_value as usize,
                edition: value.edition,
                xmult: value.xmult.unwrap() as usize,
            },
            JokerType::Walkie => Self::Walkie {
                sell_value: value.sell_value as usize,
                edition: value.edition,
            },
            JokerType::Seltzer => Self::Seltzer {
                sell_value: value.sell_value as usize,
                edition: value.edition,
            },
            JokerType::Castle => Self::Castle {
                sell_value: value.sell_value as usize,
                edition: value.edition,
                chips: value.chips.unwrap() as usize,
                suit: value.suit.unwrap(),
            },
            JokerType::Smiley => Self::Smiley {
                sell_value: value.sell_value as usize,
                edition: value.edition,
            },
            JokerType::Campfire => Self::Campfire {
                sell_value: value.sell_value as usize,
                edition: value.edition,
                xmult: value.xmult.unwrap() as usize,
            },
            JokerType::Ticket => Self::Ticket {
                sell_value: value.sell_value as usize,
                edition: value.edition,
            },
            JokerType::Bones => Self::Bones {
                sell_value: value.sell_value as usize,
                edition: value.edition,
            },
            JokerType::Acrobat => Self::Acrobat {
                sell_value: value.sell_value as usize,
                edition: value.edition,
            },
            JokerType::Sock => Self::Sock {
                sell_value: value.sell_value as usize,
                edition: value.edition,
            },
            JokerType::Swashbuckler => Self::Swashbuckler {
                sell_value: value.sell_value as usize,
                edition: value.edition,
            },
            JokerType::Troubadour => Self::Troubadour {
                sell_value: value.sell_value as usize,
                edition: value.edition,
            },
            JokerType::Certificate => Self::Certificate {
                sell_value: value.sell_value as usize,
                edition: value.edition,
            },
            JokerType::Smeared => Self::Smeared {
                sell_value: value.sell_value as usize,
                edition: value.edition,
            },
            JokerType::Throwback => Self::Throwback {
                sell_value: value.sell_value as usize,
                edition: value.edition,
                xmult: value.xmult.unwrap() as usize,
            },
            JokerType::Chad => Self::Chad {
                sell_value: value.sell_value as usize,
                edition: value.edition,
            },
            JokerType::Gem => Self::Gem {
                sell_value: value.sell_value as usize,
                edition: value.edition,
            },
            JokerType::Bloodstone => Self::Bloodstone {
                sell_value: value.sell_value as usize,
                edition: value.edition,
            },
            JokerType::Arrowhead => Self::Arrowhead {
                sell_value: value.sell_value as usize,
                edition: value.edition,
            },
            JokerType::Onyx => Self::Onyx {
                sell_value: value.sell_value as usize,
                edition: value.edition,
            },
            JokerType::Glass => Self::Glass {
                sell_value: value.sell_value as usize,
                edition: value.edition,
                xmult: value.xmult.unwrap() as usize,
            },
            JokerType::Showman => Self::Showman {
                sell_value: value.sell_value as usize,
                edition: value.edition,
            },
            JokerType::Flower => Self::Flower {
                sell_value: value.sell_value as usize,
                edition: value.edition,
            },
            JokerType::Blueprint => Self::Blueprint {
                sell_value: value.sell_value as usize,
                edition: value.edition,
            },
            JokerType::Wee => Self::Wee {
                sell_value: value.sell_value as usize,
                edition: value.edition,
                chips: value.chips.unwrap() as usize,
            },
            JokerType::Andy => Self::Andy {
                sell_value: value.sell_value as usize,
                edition: value.edition,
            },
            JokerType::Oops => Self::Oops {
                sell_value: value.sell_value as usize,
                edition: value.edition,
            },
            JokerType::Idol => Self::Idol {
                sell_value: value.sell_value as usize,
                edition: value.edition,
                rank: value.rank.unwrap(),
                suit: value.suit.unwrap(),
            },
            JokerType::Double => Self::Double {
                sell_value: value.sell_value as usize,
                edition: value.edition,
            },
            JokerType::Matador => Self::Matador {
                sell_value: value.sell_value as usize,
                edition: value.edition,
            },
            JokerType::Road => Self::Road {
                sell_value: value.sell_value as usize,
                edition: value.edition,
                xmult: value.xmult.unwrap() as usize,
            },
            JokerType::Duo => Self::Duo {
                sell_value: value.sell_value as usize,
                edition: value.edition,
            },
            JokerType::Trio => Self::Trio {
                sell_value: value.sell_value as usize,
                edition: value.edition,
            },
            JokerType => Self::Trio {
                sell_value: value.sell_value as usize,
                edition: value.edition,
            },
            JokerType::Family => Self::Family {
                sell_value: value.sell_value as usize,
                edition: value.edition,
            },
            JokerType::Order => Self::Order {
                sell_value: value.sell_value as usize,
                edition: value.edition,
            },
            JokerType::Tribe => Self::Tribe {
                sell_value: value.sell_value as usize,
                edition: value.edition,
            },
            JokerType::Stuntman => Self::Stuntman {
                sell_value: value.sell_value as usize,
                edition: value.edition,
            },
            JokerType::Invisible => Self::Invisible {
                sell_value: value.sell_value as usize,
                edition: value.edition,
            },
            JokerType::Brainstorm => Self::Brainstorm {
                sell_value: value.sell_value as usize,
                edition: value.edition,
            },
            JokerType::Satellite => Self::Satellite {
                sell_value: value.sell_value as usize,
                edition: value.edition,
            },
            JokerType::Shoot => Self::Shoot {
                sell_value: value.sell_value as usize,
                edition: value.edition,
            },
            JokerType::License => Self::License {
                sell_value: value.sell_value as usize,
                edition: value.edition,
            },
            JokerType::Cartomancer => Self::Cartomancer {
                sell_value: value.sell_value as usize,
                edition: value.edition,
            },
            JokerType::Astronomer => Self::Astronomer {
                sell_value: value.sell_value as usize,
                edition: value.edition,
            },
            JokerType::Burnt => Self::Burnt {
                sell_value: value.sell_value as usize,
                edition: value.edition,
            },
            JokerType::Bootstraps => Self::Bootstraps {
                sell_value: value.sell_value as usize,
                edition: value.edition,
            },
            JokerType::Canio => Self::Canio {
                sell_value: value.sell_value as usize,
                edition: value.edition,
                xmult: value.xmult.unwrap() as usize,
            },
            JokerType::Triboulet => Self::Triboulet {
                sell_value: value.sell_value as usize,
                edition: value.edition,
            },
            JokerType::Yorick => Self::Yorick {
                sell_value: value.sell_value as usize,
                edition: value.edition,
                xmult: value.xmult.unwrap() as usize,
            },
            JokerType::Chicot => Self::Chicot {
                sell_value: value.sell_value as usize,
                edition: value.edition,
            },
            JokerType::Perkeo => Self::Perkeo {
                sell_value: value.sell_value as usize,
                edition: value.edition,
            },
        }
    }
}

impl Joker {
    pub fn on_blind_select(&mut self, state: &mut State) {
        match self {
            Self::Dagger { mut mult, .. } => {
                if state.jokers.iter().position(|j| j == self).unwrap() + 1 < state.jokers.len() {
                    mult += state
                        .jokers
                        .remove(state.jokers.iter().position(|j| j == self).unwrap() + 1)
                        .get_sell_value();
                }
            }
            Self::Marble { .. } => {
                todo!("randomness needed")
            }
            Self::Burglar { .. } => {
                state.hands_remaining += 3;
                state.discards_remaining = 0;
            }
            Self::Madness { mut xmult, .. } => todo!("blinds"),
            Self::Riff { .. } => todo!("randomness"),
            Self::Certificate { .. } => todo!("randomness"),
            Self::Cartomancer { .. } => todo!("randomness"),
            _ => {}
        }
    }

    pub fn on_played(
        &mut self,
        state: &mut State,
        played_cards: &mut [Card],
        scoring_cards: &mut [Card],
        hand_type_played: &HandType,
    ) {
        match self {
            Self::Fist { mut min, .. } => {
                min = Some(state.hand.iter().map(|c| c.rank).min().unwrap());
            }
            Self::Bus { mut mult, .. } => {
                if scoring_cards
                    .iter()
                    .filter(|c| c.is_face_card(&state.jokers))
                    .count()
                    == 0
                {
                    mult += 1;
                } else {
                    mult = 0;
                }
            }
            Self::Space { .. } => todo!("randomness"),
            Self::Runner { mut chips, .. } => {
                if *hand_type_played == HandType::Straight
                    || *hand_type_played == HandType::StraightFlush
                {
                    chips += 15;
                }
            }
            Self::DNA { .. } => {
                if state.hands_remaining == state.hands - 1 && played_cards.len() == 1 {
                    state.deck.push(played_cards.get(0).unwrap().clone());
                    state.hand.push(played_cards.get(0).unwrap().clone());
                }
            }
            // Self::Splash { .. } => {
            //     scoring_cards = played_cards;
            // }
            Self::Sixth { .. } => {
                if played_cards.len() == 1 && played_cards.get(0).unwrap().rank == Rank::Six {
                    state.deck.remove(
                        state
                            .deck
                            .iter()
                            .position(|c| c == played_cards.get(0).unwrap())
                            .unwrap(),
                    );
                }
            }
            Self::Green { mut mult, .. } => mult += 1,
            Self::List { hand_type, .. } => {
                if *hand_type == *hand_type_played {
                    state.money += 4;
                }
            }
            Self::Square { mut chips, .. } => {
                if played_cards.len() == 4 {
                    chips += 4;
                }
            }
            Self::Vampire { mut xmult, .. } => {
                scoring_cards
                    .iter_mut()
                    .filter(|c| c.enhancement.is_some())
                    .for_each(|c| {
                        c.enhancement = None;
                        xmult += 1;
                    });
            }
            Self::Obelisk { mut xmult, .. } => {
                if state
                    .scoring
                    .scoring_times
                    .iter()
                    .filter(|e| {
                        e.1 == state
                            .scoring
                            .scoring_times
                            .iter()
                            .map(|e| e.1)
                            .max_by(|x, y| x.cmp(y))
                            .unwrap()
                    })
                    .map(|(k, _)| k)
                    .any(|k| *k == *hand_type_played)
                {
                    xmult = 5;
                } else {
                    xmult += 1;
                }
            }
            Self::Midas { .. } => {
                played_cards
                    .iter_mut()
                    .filter(|c| c.is_face_card(&state.jokers))
                    .for_each(|c| c.enhancement = Some(Enhancement::Gold));
            }
            Self::Photograph { mut used, .. } => used = false,
            Self::Trousers { mut mult, .. } => {
                let s: HashSet<Rank> = HashSet::from_iter(played_cards.iter().map(|c| c.rank));
                let mut result: u8 = 0;
                if s.len() < played_cards.len() - 1 {
                    for v in s {
                        if played_cards
                            .iter()
                            .filter(|c| c.rank == v)
                            .collect::<Vec<&Card>>()
                            .len()
                            >= 2
                        {
                            result += 1;
                        }
                    }
                }

                if result == 2 {
                    mult += 2;
                }
            }
            _ => {}
        }
    }

    pub fn on_scored(&mut self, state: &mut State, card: &mut Card) -> ScoreModification {
        let mut modification = ScoreModification::default();
        match self {
            Self::Greedy { .. } => {
                if card.is_suit(Suit::Diamond, &state.jokers) {
                    modification.mult += 3;
                }
            }
            Self::Lusty { .. } => {
                if card.is_suit(Suit::Heart, &state.jokers) {
                    modification.mult += 3;
                }
            }
            Self::Wrathful { .. } => {
                if card.is_suit(Suit::Spade, &state.jokers) {
                    modification.mult += 3;
                }
            }
            Self::Gluttonous { .. } => {
                if card.is_suit(Suit::Club, &state.jokers) {
                    modification.mult += 3;
                }
            }
            Self::Ball { .. } => {
                if card.rank == Rank::Eight {
                    todo!("randomness")
                }
            }
            Self::Dusk { .. } => {
                if state.hands_remaining == 0 {
                    modification.triggers += 1;
                }
            }
            Self::Fibonacci { .. } => {
                if card.rank == Rank::Ace
                    || card.rank == Rank::Two
                    || card.rank == Rank::Three
                    || card.rank == Rank::Five
                    || card.rank == Rank::Eight
                {
                    modification.mult += 9;
                }
            }
            Self::Scary { .. } => {
                modification.chips += 30;
            }
            Self::Hack { .. } => {
                if card.rank == Rank::Two
                    || card.rank == Rank::Three
                    || card.rank == Rank::Four
                    || card.rank == Rank::Five
                {
                    modification.triggers += 1;
                }
            }
            Self::Steven { .. } => {
                if card.rank == Rank::Two
                    || card.rank == Rank::Four
                    || card.rank == Rank::Six
                    || card.rank == Rank::Eight
                    || card.rank == Rank::Ten
                {
                    modification.mult += 4;
                }
            }
            Self::Todd { .. } => {
                if card.rank == Rank::Ace
                    || card.rank == Rank::Three
                    || card.rank == Rank::Five
                    || card.rank == Rank::Seven
                    || card.rank == Rank::Nine
                {
                    modification.chips += 31;
                }
            }
            Self::Scholar { .. } => {
                if card.rank == Rank::Ace {
                    modification.chips += 20;
                    modification.mult += 4;
                }
            }
            Self::Business { .. } => {
                if card.is_face_card(&state.jokers) {
                    todo!("randomness");
                }
            }
            Self::Hiker { .. } => {
                card.chips += 5;
            }
            Self::Photograph { mut used, .. } => {
                if !used && card.is_face_card(&state.jokers) {
                    used = true;
                    modification.xmult += 2.0;
                }
            }
            Self::Ancient { suit, .. } => {
                if card.is_suit(*suit, &state.jokers) {
                    modification.xmult += 1.5;
                }
            }
            Self::Walkie { .. } => {
                if card.rank == Rank::Ten || card.rank == Rank::Four {
                    modification.chips += 10;
                    modification.mult += 4;
                }
            }
            Self::Seltzer { .. } => modification.triggers += 1,
            Self::Smiley { .. } => {
                if card.is_face_card(&state.jokers) {
                    modification.mult += 5;
                }
            }
            Self::Ticket { .. } => {
                if card.enhancement == Some(Enhancement::Gold) {
                    state.money += 4;
                }
            }
            Self::Sock { .. } => {
                if card.is_face_card(&state.jokers) {
                    modification.triggers += 1;
                }
            }
            Self::Chad { .. } => {
                modification.triggers += 2;
            }
            Self::Gem { .. } => {
                if card.is_suit(Suit::Diamond, &state.jokers) {
                    state.money += 1;
                }
            }
            Self::Bloodstone { .. } => todo!("randomness"),
            Self::Arrowhead { .. } => {
                if card.is_suit(Suit::Spade, &state.jokers) {
                    modification.chips += 50;
                }
            }
            Self::Onyx { .. } => {
                if card.is_suit(Suit::Club, &state.jokers) {
                    modification.mult += 7;
                }
            }
            Self::Wee { mut chips, .. } => {
                if card.rank == Rank::Two {
                    chips += 8;
                }
            }
            Self::Idol { rank, suit, .. } => {
                if card.is_suit(*suit, &state.jokers) && card.rank == *rank {
                    modification.xmult += 2.0;
                }
            }
            Self::Triboulet { .. } => {
                if card.rank == Rank::King || card.rank == Rank::Queen {
                    modification.xmult += 2.0;
                }
            }
            _ => {}
        }
        modification
    }

    pub fn on_held(&mut self, state: &mut State, card: &Card) -> ScoreModification {
        let mut modification = ScoreModification::default();
        match self {
            Self::Mime { .. } => modification.triggers += 1,
            Self::Fist { mut min, .. } => {
                if let Some(m) = min {
                    if m == card.rank {
                        modification.mult += (card.rank.get_value() * 2) as isize;
                    }
                    min = None;
                }
            }
            Self::Baron { .. } => {
                if card.rank == Rank::King {
                    modification.xmult += 1.5;
                }
            }
            Self::Parking { .. } => todo!("randomness"),
            Self::Shoot { .. } => {
                if card.rank == Rank::Queen {
                    modification.mult += 13;
                }
            }
            _ => {}
        }
        modification
    }

    pub fn on_independent(
        &mut self,
        state: &mut State,
        played_cards: &mut [Card],
        scoring_cards: &mut [Card],
        hand_type_played: &HandType,
    ) -> ScoreModification {
        let mut modification = ScoreModification::default();
        match self {
            Self::Joker { .. } => modification.mult += 4,
            Self::Jolly { .. } => {
                if HashSet::<Rank>::from_iter(played_cards.iter().cloned().map(|c| c.rank)).len()
                    < played_cards.len()
                {
                    modification.mult += 8;
                }
            }
            Self::Zany { .. } => {
                let s: HashSet<Rank> = HashSet::from_iter(played_cards.iter().map(|c| c.rank));
                let mut result: bool = false;
                if s.len() < played_cards.len() - 1 {
                    for v in s {
                        result |= played_cards
                            .iter()
                            .filter(|c| c.rank == v)
                            .collect::<Vec<&Card>>()
                            .len()
                            >= 3;
                    }
                }

                if result {
                    modification.mult += 12;
                }
            }
            Self::Mad { .. } => {
                let s: HashSet<Rank> = HashSet::from_iter(played_cards.iter().map(|c| c.rank));
                let mut result: u8 = 0;
                if s.len() < played_cards.len() - 1 {
                    for v in s {
                        if played_cards
                            .iter()
                            .filter(|c| c.rank == v)
                            .collect::<Vec<&Card>>()
                            .len()
                            >= 2
                        {
                            result += 1;
                        }
                    }
                }

                if result == 2 {
                    modification.mult += 10;
                }
            }
            Self::Crazy { .. } => {
                if *hand_type_played == HandType::Straight
                    || *hand_type_played == HandType::StraightFlush
                {
                    modification.mult += 12;
                }
            }
            Self::Droll { .. } => {
                let s: HashSet<Suit> = HashSet::from_iter(played_cards.iter().map(|c| c.suit));
                if s.len() == 1 {
                    modification.mult += 10;
                }
            }
            Self::Sly { .. } => {
                if HashSet::<Rank>::from_iter(played_cards.iter().cloned().map(|c| c.rank)).len()
                    < played_cards.len()
                {
                    modification.chips += 50;
                }
            }
            Self::Wily { .. } => {
                let s: HashSet<Rank> = HashSet::from_iter(played_cards.iter().map(|c| c.rank));
                let mut result: bool = false;
                if s.len() < played_cards.len() - 1 {
                    for v in s {
                        result |= played_cards
                            .iter()
                            .filter(|c| c.rank == v)
                            .collect::<Vec<&Card>>()
                            .len()
                            >= 3;
                    }
                }

                if result {
                    modification.chips += 100;
                }
            }
            Self::Clever { .. } => {
                let s: HashSet<Rank> = HashSet::from_iter(played_cards.iter().map(|c| c.rank));
                let mut result: u8 = 0;
                if s.len() < played_cards.len() - 1 {
                    for v in s {
                        if played_cards
                            .iter()
                            .filter(|c| c.rank == v)
                            .collect::<Vec<&Card>>()
                            .len()
                            >= 2
                        {
                            result += 1;
                        }
                    }
                }

                if result == 2 {
                    modification.chips += 80;
                }
            }
            Self::Devious { .. } => {
                if *hand_type_played == HandType::Straight
                    || *hand_type_played == HandType::StraightFlush
                {
                    modification.chips += 100;
                }
            }
            Self::Crafty { .. } => {
                let s: HashSet<Suit> = HashSet::from_iter(played_cards.iter().map(|c| c.suit));
                if s.len() == 1 {
                    modification.chips += 80;
                }
            }
            Self::Half { .. } => {
                if played_cards.len() <= 3 {
                    // state.current_score.update(None, Some(20.0), None);
                    modification.mult += 20;
                }
            }
            Self::Stencil { .. } => {
                modification.xmult += (state.jokers.len() - state.joker_slots + 1) as f64;
            }
            Self::Dagger { mult, .. } => modification.mult += *mult as isize,
            Self::Banner { .. } => {
                modification.chips += (state.discards_remaining * 30) as isize;
            }
            Self::Mystic { .. } => {
                if state.discards_remaining == 0 {
                    modification.mult += 15;
                }
            }
            Self::Loyalty { mut hands, .. } => {
                if hands == 0 {
                    modification.xmult += 4.0;
                    hands = 6;
                }
                hands -= 1;
            }
            Self::Misprint { .. } => {
                todo!("randomness")
            }
            Self::Steel { .. } => {
                modification.xmult += 0.2
                    * state
                        .deck
                        .iter()
                        .filter(|c| c.enhancement == Some(Enhancement::Steel))
                        .count() as f64;
            }
            Self::Abstract { .. } => {
                modification.mult += 3 * state.jokers.len() as isize;
            }
            Self::Michel { .. } => modification.mult += 15,
            Self::Supernova { .. } => {
                modification.mult +=
                    *state.scoring.scoring_times.get(hand_type_played).unwrap() as isize;
            }
            Self::Bus { mult, .. } => modification.mult += *mult as isize,
            Self::Blackboard { .. } => {
                if state
                    .hand
                    .iter()
                    .filter(|c| {
                        !(c.is_suit(Suit::Spade, &state.jokers)
                            || c.is_suit(Suit::Club, &state.jokers))
                    })
                    .count()
                    == 0
                {
                    modification.xmult += 3.0;
                }
            }
            Self::Runner { chips, .. } => {
                modification.chips += *chips as isize;
            }
            Self::Cream { mut chips, .. } => {
                modification.chips += chips as isize;
                chips -= 5;
            }
            Self::Blue { .. } => modification.chips += 2 * state.remaining_deck.len() as isize,
            Self::Constellation { mult, .. } => modification.xmult += (*mult / 10) as f64,
            Self::Green { mult, .. } => modification.mult += *mult as isize,
            Self::Superposition { .. } => {
                if (*hand_type_played == HandType::Straight
                    || *hand_type_played == HandType::StraightFlush)
                    && played_cards.iter().filter(|c| c.rank == Rank::Ace).count() > 0
                    && state.consumables.len() < state.consumable_slots
                {
                    todo!("randomness")
                }
            }
            Self::Cavendish { .. } => modification.xmult += 3.0,
            Self::Sharp { .. } => {
                if state
                    .scoring
                    .scoring_times
                    .iter()
                    .filter(|e| *e.1 != 0)
                    .map(|e| e.0)
                    .any(|t| *t == *hand_type_played)
                {
                    modification.xmult += 3.0;
                }
            }
            Self::Red { mult, .. } => modification.mult += *mult as isize,
            Self::Madness { xmult, .. } => modification.xmult += (*xmult / 4) as f64,
            Self::Square { chips, .. } => modification.chips += *chips as isize,
            Self::Seance { .. } => {
                if *hand_type_played == HandType::StraightFlush
                    && state.consumables.len() < state.consumable_slots
                {
                    todo!("randomness")
                }
            }
            Self::Vampire { xmult, .. } => modification.xmult += (*xmult / 10) as f64,
            Self::Hologram { xmult, .. } => modification.xmult += (*xmult / 4) as f64,
            Self::Vagabond { .. } => {
                if state.money < 5 {
                    todo!("randomness")
                }
            }
            Self::Obelisk { xmult, .. } => modification.xmult += (*xmult / 4) as f64,
            Self::Erosion { .. } => modification.mult += max(52 - state.deck.len(), 0) as isize,
            Self::Fortune { .. } => modification.mult += state.tarots_used as isize,
            Self::Stone { .. } => {
                modification.chips += (state
                    .deck
                    .iter()
                    .filter(|c| c.enhancement == Some(Enhancement::Stone))
                    .count()
                    * 25) as isize;
            }
            Self::Lucky { xmult, .. } => modification.xmult += (*xmult / 4) as f64,
            Self::Bull { .. } => modification.chips += 2 * state.money as isize,
            Self::Flash { mult, .. } => modification.mult += *mult as isize,
            Self::Popcorn { mult, .. } => modification.mult += *mult as isize,
            Self::Trousers { mult, .. } => modification.mult += *mult as isize,
            Self::Ramen { xmult, .. } => modification.xmult += (*xmult / 100) as f64,
            Self::Castle { chips, .. } => modification.chips += *chips as isize,
            Self::Campfire { xmult, .. } => modification.xmult += (*xmult / 4) as f64,
            Self::Acrobat { .. } => {
                if state.hands_remaining == state.hands - 1 {
                    modification.xmult += 3.0;
                }
            }
            Self::Swashbuckler { .. } => {
                modification.mult += (state
                    .jokers
                    .iter()
                    .map(|j| j.get_sell_value())
                    .sum::<usize>()
                    - self.get_sell_value()) as isize
            }
            Self::Throwback { xmult, .. } => modification.xmult += (*xmult / 4) as f64,
            Self::Glass { xmult, .. } => modification.xmult += (*xmult / 4) as f64,
            Self::Flower { .. } => {
                if scoring_cards
                    .iter()
                    .any(|c| c.is_suit(Suit::Spade, &state.jokers))
                    && scoring_cards
                        .iter()
                        .any(|c| c.is_suit(Suit::Heart, &state.jokers))
                    && scoring_cards
                        .iter()
                        .any(|c| c.is_suit(Suit::Club, &state.jokers))
                    && scoring_cards
                        .iter()
                        .any(|c| c.is_suit(Suit::Diamond, &state.jokers))
                {
                    modification.xmult += 3.0;
                }
            }
            Self::Wee { chips, .. } => modification.chips += *chips as isize,
            Self::Double { .. } => {
                if scoring_cards
                    .iter()
                    .any(|c| c.is_suit(Suit::Club, &state.jokers))
                    && scoring_cards
                        .iter()
                        .any(|c| !c.is_suit(Suit::Club, &state.jokers))
                {
                    modification.xmult += 2.0;
                }
            }
            Self::Road { xmult, .. } => modification.xmult += (*xmult / 2) as f64,
            Self::Duo { .. } => {
                if HashSet::<Rank>::from_iter(scoring_cards.iter().map(|c| c.rank)).len()
                    < scoring_cards.len()
                {
                    modification.xmult += 2.0;
                }
            }
            Self::Trio { .. } => {
                let s: HashSet<Rank> = HashSet::from_iter(scoring_cards.iter().map(|c| c.rank));
                if s.len() < scoring_cards.len() - 1
                    && scoring_cards
                        .iter()
                        .filter(|c| c.rank == *s.iter().max().unwrap())
                        .count()
                        >= 3
                {
                    modification.xmult += 3.0;
                }
            }
            Self::Family { .. } => {
                let s: HashSet<Rank> = HashSet::from_iter(scoring_cards.iter().map(|c| c.rank));
                if s.len() < scoring_cards.len() - 2 {
                    modification.xmult += 4.0;
                }
            }
            Self::Order { .. } => {
                if *hand_type_played == HandType::Straight
                    || *hand_type_played == HandType::StraightFlush
                {
                    modification.xmult += 3.0;
                }
            }
            Self::Tribe { .. } => {
                if *hand_type_played == HandType::Flush
                    || *hand_type_played == HandType::FlushHouse
                    || *hand_type_played == HandType::FlushFive
                {
                    modification.xmult += 2.0;
                }
            }
            Self::Stuntman { .. } => modification.chips += 100,
            Self::License { .. } => {
                if state
                    .deck
                    .iter()
                    .filter(|c| c.enhancement.is_some())
                    .count()
                    >= 16
                {
                    modification.xmult += 3.0;
                }
            }
            Self::Bootstraps { .. } => modification.mult += ((state.money / 5) * 2) as isize,
            Self::Canio { xmult, .. } => modification.xmult += *xmult as f64,
            Self::Yorick { xmult, .. } => modification.xmult += *xmult as f64,
            _ => {}
        }
        modification
    }

    pub fn on_discard(&mut self, state: &mut State, played_cards: &mut [Card]) {
        match self {
            Self::Faceless { .. } => {
                if played_cards
                    .iter()
                    .filter(|c| c.is_face_card(&state.jokers))
                    .count()
                    >= 3
                {
                    state.money += 5;
                }
            }
            Self::Green { mut mult, .. } => {
                if mult > 0 {
                    mult -= 1;
                }
            }
            Self::Rebate { rank, .. } => played_cards
                .iter()
                .filter(|c| c.rank == *rank)
                .for_each(|_| state.money += 5),
            Self::Trading { .. } => {
                if state.discards_remaining == state.discards - 1 && played_cards.len() == 1 {
                    state.deck.remove(
                        state
                            .deck
                            .iter()
                            .position(|c| c == played_cards.get(0).unwrap())
                            .unwrap(),
                    );
                    state.money += 3;
                }
            }
            Self::Ramen { mut xmult, .. } => {
                xmult -= played_cards.len();
                if xmult <= 100 {
                    state
                        .jokers
                        .remove(state.jokers.iter().position(|j| j == self).unwrap());
                }
            }
            Self::Castle {
                suit, mut chips, ..
            } => {
                played_cards
                    .iter()
                    .filter(|c| c.is_suit(*suit, &state.jokers))
                    .for_each(|_| chips += 3);
            }
            Self::Road { mut xmult, .. } => xmult += 1,
            Self::Yorick {
                mut discards,
                mut xmult,
                ..
            } => {
                discards -= played_cards.len();
                if discards <= 0 {
                    discards += 23;
                    xmult += 1;
                }
            }
            _ => {}
        }
    }

    pub fn on_round_end(&mut self, state: &mut State) {
        match self {
            Self::Gratification { .. } => {
                if state.discards_remaining == state.discards {
                    state.money += 2 * state.discards;
                }
            }
            Self::Michel { .. } => todo!("randomness"),
            Self::Egg { mut sell_value, .. } => sell_value += 3,
            Self::List { hand_type, .. } => todo!("randomness"),
            Self::Cavendish { .. } => todo!("randomness"),
            Self::Cloud { .. } => {
                state.money += state.deck.iter().filter(|c| c.rank == Rank::Nine).count();
            }
            Self::Rocket { money, .. } => {
                state.money += *money;
                todo!("blinds")
            }
            Self::Gift { .. } => {
                state
                    .jokers
                    .iter_mut()
                    .for_each(|j| j.increase_sell_value());
            }
            Self::Turtle { mut hand_size, .. } => {
                hand_size -= 1;
                if hand_size == 0 {
                    state
                        .jokers
                        .remove(state.jokers.iter().position(|j| j == self).unwrap());
                }
            }
            Self::Rebate { mut rank, .. } => todo!("randomness"),
            Self::Golden { .. } => state.money += 4,
            Self::Popcorn { mut mult, .. } => {
                mult -= 4;
                if mult == 0 {
                    state
                        .jokers
                        .remove(state.jokers.iter().position(|j| j == self).unwrap());
                }
            }
            Self::Ancient { mut suit, .. } => todo!("randomness"),
            Self::Castle { mut suit, .. } => todo!("randomness"),
            Self::Campfire { mut xmult, .. } => todo!("blinds"),
            Self::Idol {
                mut rank, mut suit, ..
            } => todo!("randomness"),
            Self::Road { mut xmult, .. } => xmult = 2,
            Self::Invisible { mut rounds, .. } => rounds += 1,
            Self::Satellite { .. } => state.money += state.planets_used.len(),
            _ => {}
        }
    }

    pub fn on_shop_close(&mut self, state: &mut State) {}

    pub fn on_shop_reroll(&mut self, state: &mut State) {
        match self {
            Self::Flash { mut mult, .. } => mult += 2,
            _ => {}
        }
    }

    pub fn on_pack_open(&mut self, state: &mut State) {
        match self {
            Self::Hallucination { .. } => todo!("randomness"),
            _ => {}
        }
    }

    pub fn on_pack_skip(&mut self, state: &State) {
        match self {
            Self::Red { mut mult, .. } => mult += 3,
            _ => {}
        }
    }

    pub fn on_buy(&self, state: &mut State) {
        match self {
            Self::Juggler { .. } => state.hand_size += 1,
            Self::Drunkard { .. } => {
                state.discards += 1;
                state.discards_remaining += 1;
            }
            Self::Troubadour { .. } => {
                state.hand_size += 2;
                state.hands -= 1;
                state.hands_remaining -= 1;
            }
            Self::Andy { .. } => {
                state.discards += 3;
                state.discards_remaining += 3;
                state.hand_size -= 1;
            }
            Self::Stuntman { .. } => state.hand_size -= 2,
            _ => {}
        }
    }

    pub fn on_sell(self, state: &mut State) {
        match self {
            Self::Luchador { .. } => todo!("blinds"),
            Self::Juggler { .. } => state.hand_size -= 1,
            Self::Drunkard { .. } => {
                state.discards -= 1;
                state.discards_remaining -= 1;
            }
            Self::Cola { .. } => todo!("tags"),
            Self::Troubadour { .. } => {
                state.hand_size -= 2;
                state.hands += 1;
                state.hands_remaining += 1;
            }
            Self::Andy { .. } => {
                state.discards -= 3;
                state.discards_remaining -= 3;
                state.hand_size -= 1;
            }
            Self::Stuntman { .. } => state.hand_size += 2,
            _ => {}
        }
    }

    pub fn on_planet_use(&mut self) {
        match self {
            Self::Constellation { mut mult, .. } => {
                mult += 1;
            }
            _ => {}
        }
    }

    pub fn on_card_add(&mut self) {
        match self {
            Self::Hologram { mut xmult, .. } => xmult += 1,
            _ => {}
        }
    }

    pub fn on_card_sell(&mut self) {
        match self {
            Self::Campfire { mut xmult, .. } => xmult += 1,
            _ => {}
        }
    }

    pub fn on_blind_skip(&mut self) {
        match self {
            Self::Throwback { mut xmult, .. } => xmult += 1,
            _ => {}
        }
    }

    pub fn on_card_destroy(&mut self, card: &Card) {
        match self {
            Self::Glass { mut xmult, .. } => {
                if card.enhancement == Some(Enhancement::Glass) {
                    xmult += 3;
                }
            }
            _ => {}
        }
    }

    pub fn get_cost(&self) -> usize {
        match self {
            Self::Joker { .. } => 2,
            Self::Greedy { .. } => 5,
            Self::Lusty { .. } => 5,
            Self::Wrathful { .. } => 5,
            Self::Gluttonous { .. } => 5,
            Self::Jolly { .. } => 3,
            Self::Zany { .. } => 4,
            Self::Mad { .. } => 4,
            Self::Crazy { .. } => 4,
            Self::Droll { .. } => 4,
            Self::Sly { .. } => 3,
            Self::Wily { .. } => 4,
            Self::Clever { .. } => 4,
            Self::Devious { .. } => 4,
            Self::Crafty { .. } => 4,
            Self::Half { .. } => 5,
            Self::Stencil { .. } => 8,
            Self::Fingers { .. } => 7,
            Self::Mime { .. } => 5,
            Self::Credit { .. } => 1,
            Self::Dagger { .. } => 6,
            Self::Banner { .. } => 5,
            Self::Mystic { .. } => 5,
            Self::Marble { .. } => 6,
            Self::Loyalty { .. } => 5,
            Self::Ball { .. } => 5,
            Self::Misprint { .. } => 4,
            Self::Dusk { .. } => 5,
            Self::Fist { .. } => 5,
            Self::Chaos { .. } => 4,
            Self::Fibonacci { .. } => 8,
            Self::Steel { .. } => 7,
            Self::Scary { .. } => 4,
            Self::Abstract { .. } => 4,
            Self::Gratification { .. } => 4,
            Self::Hack { .. } => 6,
            Self::Pareidolia { .. } => 5,
            Self::Michel { .. } => 5,
            Self::Steven { .. } => 5,
            Self::Todd { .. } => 4,
            Self::Scholar { .. } => 4,
            Self::Business { .. } => 4,
            Self::Supernova { .. } => 5,
            Self::Bus { .. } => 6,
            Self::Space { .. } => 5,
            Self::Egg { .. } => 4,
            Self::Burglar { .. } => 6,
            Self::Blackboard { .. } => 6,
            Self::Runner { .. } => 5,
            Self::Cream { .. } => 5,
            Self::DNA { .. } => 8,
            Self::Splash { .. } => 3,
            Self::Blue { .. } => 5,
            Self::Sixth { .. } => 6,
            Self::Constellation { .. } => 6,
            Self::Hiker { .. } => 5,
            Self::Faceless { .. } => 4,
            Self::Green { .. } => 4,
            Self::Superposition { .. } => 4,
            Self::List { .. } => 4,
            Self::Cavendish { .. } => 4,
            Self::Sharp { .. } => 6,
            Self::Red { .. } => 5,
            Self::Madness { .. } => 7,
            Self::Square { .. } => 4,
            Self::Seance { .. } => 6,
            Self::Riff { .. } => 6,
            Self::Vampire { .. } => 7,
            Self::Shortcut { .. } => 7,
            Self::Hologram { .. } => 7,
            Self::Vagabond { .. } => 8,
            Self::Baron { .. } => 8,
            Self::Cloud { .. } => 7,
            Self::Rocket { .. } => 6,
            Self::Obelisk { .. } => 8,
            Self::Midas { .. } => 7,
            Self::Luchador { .. } => 5,
            Self::Photograph { .. } => 5,
            Self::Gift { .. } => 6,
            Self::Turtle { .. } => 6,
            Self::Erosion { .. } => 6,
            Self::Parking { .. } => 6,
            Self::Rebate { .. } => 4,
            Self::Moon { .. } => 5,
            Self::Hallucination { .. } => 4,
            Self::Fortune { .. } => 6,
            Self::Juggler { .. } => 4,
            Self::Drunkard { .. } => 4,
            Self::Stone { .. } => 6,
            Self::Golden { .. } => 6,
            Self::Lucky { .. } => 6,
            Self::Baseball { .. } => 8,
            Self::Bull { .. } => 6,
            Self::Cola { .. } => 6,
            Self::Trading { .. } => 6,
            Self::Flash { .. } => 5,
            Self::Popcorn { .. } => 5,
            Self::Trousers { .. } => 6,
            Self::Ancient { .. } => 8,
            Self::Ramen { .. } => 6,
            Self::Walkie { .. } => 4,
            Self::Seltzer { .. } => 6,
            Self::Castle { .. } => 6,
            Self::Smiley { .. } => 4,
            Self::Campfire { .. } => 9,
            Self::Ticket { .. } => 5,
            Self::Bones { .. } => 5,
            Self::Acrobat { .. } => 6,
            Self::Sock { .. } => 6,
            Self::Swashbuckler { .. } => 4,
            Self::Troubadour { .. } => 6,
            Self::Certificate { .. } => 6,
            Self::Smeared { .. } => 7,
            Self::Throwback { .. } => 6,
            Self::Chad { .. } => 4,
            Self::Gem { .. } => 7,
            Self::Bloodstone { .. } => 7,
            Self::Arrowhead { .. } => 7,
            Self::Onyx { .. } => 7,
            Self::Glass { .. } => 6,
            Self::Showman { .. } => 5,
            Self::Flower { .. } => 6,
            Self::Blueprint { .. } => 10,
            Self::Wee { .. } => 8,
            Self::Andy { .. } => 7,
            Self::Oops { .. } => 4,
            Self::Idol { .. } => 6,
            Self::Double { .. } => 6,
            Self::Matador { .. } => 7,
            Self::Road { .. } => 8,
            Self::Duo { .. } => 8,
            Self::Trio { .. } => 8,
            Self::Family { .. } => 8,
            Self::Order { .. } => 8,
            Self::Tribe { .. } => 8,
            Self::Stuntman { .. } => 7,
            Self::Invisible { .. } => 8,
            Self::Brainstorm { .. } => 10,
            Self::Satellite { .. } => 6,
            Self::Shoot { .. } => 5,
            Self::License { .. } => 7,
            Self::Cartomancer { .. } => 6,
            Self::Astronomer { .. } => 8,
            Self::Burnt { .. } => 8,
            Self::Bootstraps { .. } => 7,
            Self::Canio { .. } => 20,
            Self::Triboulet { .. } => 20,
            Self::Yorick { .. } => 20,
            Self::Chicot { .. } => 20,
            Self::Perkeo { .. } => 20,
        }
    }

    pub fn get_sell_value(&self) -> usize {
        match self {
            Self::Joker { sell_value, .. }
            | Self::Greedy { sell_value, .. }
            | Self::Lusty { sell_value, .. }
            | Self::Wrathful { sell_value, .. }
            | Self::Gluttonous { sell_value, .. }
            | Self::Jolly { sell_value, .. }
            | Self::Zany { sell_value, .. }
            | Self::Mad { sell_value, .. }
            | Self::Crazy { sell_value, .. }
            | Self::Droll { sell_value, .. }
            | Self::Sly { sell_value, .. }
            | Self::Wily { sell_value, .. }
            | Self::Clever { sell_value, .. }
            | Self::Devious { sell_value, .. }
            | Self::Crafty { sell_value, .. }
            | Self::Half { sell_value, .. }
            | Self::Stencil { sell_value, .. }
            | Self::Fingers { sell_value, .. }
            | Self::Mime { sell_value, .. }
            | Self::Credit { sell_value, .. }
            | Self::Dagger { sell_value, .. }
            | Self::Banner { sell_value, .. }
            | Self::Mystic { sell_value, .. }
            | Self::Marble { sell_value, .. }
            | Self::Loyalty { sell_value, .. }
            | Self::Ball { sell_value, .. }
            | Self::Misprint { sell_value, .. }
            | Self::Dusk { sell_value, .. }
            | Self::Fist { sell_value, .. }
            | Self::Chaos { sell_value, .. }
            | Self::Fibonacci { sell_value, .. }
            | Self::Steel { sell_value, .. }
            | Self::Scary { sell_value, .. }
            | Self::Abstract { sell_value, .. }
            | Self::Gratification { sell_value, .. }
            | Self::Hack { sell_value, .. }
            | Self::Pareidolia { sell_value, .. }
            | Self::Michel { sell_value, .. }
            | Self::Steven { sell_value, .. }
            | Self::Todd { sell_value, .. }
            | Self::Scholar { sell_value, .. }
            | Self::Business { sell_value, .. }
            | Self::Supernova { sell_value, .. }
            | Self::Bus { sell_value, .. }
            | Self::Space { sell_value, .. }
            | Self::Egg { sell_value, .. }
            | Self::Burglar { sell_value, .. }
            | Self::Blackboard { sell_value, .. }
            | Self::Runner { sell_value, .. }
            | Self::Cream { sell_value, .. }
            | Self::DNA { sell_value, .. }
            | Self::Splash { sell_value, .. }
            | Self::Blue { sell_value, .. }
            | Self::Sixth { sell_value, .. }
            | Self::Constellation { sell_value, .. }
            | Self::Hiker { sell_value, .. }
            | Self::Faceless { sell_value, .. }
            | Self::Green { sell_value, .. }
            | Self::Superposition { sell_value, .. }
            | Self::List { sell_value, .. }
            | Self::Cavendish { sell_value, .. }
            | Self::Sharp { sell_value, .. }
            | Self::Red { sell_value, .. }
            | Self::Madness { sell_value, .. }
            | Self::Square { sell_value, .. }
            | Self::Seance { sell_value, .. }
            | Self::Riff { sell_value, .. }
            | Self::Vampire { sell_value, .. }
            | Self::Shortcut { sell_value, .. }
            | Self::Hologram { sell_value, .. }
            | Self::Vagabond { sell_value, .. }
            | Self::Baron { sell_value, .. }
            | Self::Cloud { sell_value, .. }
            | Self::Rocket { sell_value, .. }
            | Self::Obelisk { sell_value, .. }
            | Self::Midas { sell_value, .. }
            | Self::Luchador { sell_value, .. }
            | Self::Photograph { sell_value, .. }
            | Self::Gift { sell_value, .. }
            | Self::Turtle { sell_value, .. }
            | Self::Erosion { sell_value, .. }
            | Self::Parking { sell_value, .. }
            | Self::Rebate { sell_value, .. }
            | Self::Moon { sell_value, .. }
            | Self::Hallucination { sell_value, .. }
            | Self::Fortune { sell_value, .. }
            | Self::Juggler { sell_value, .. }
            | Self::Drunkard { sell_value, .. }
            | Self::Stone { sell_value, .. }
            | Self::Golden { sell_value, .. }
            | Self::Lucky { sell_value, .. }
            | Self::Baseball { sell_value, .. }
            | Self::Bull { sell_value, .. }
            | Self::Cola { sell_value, .. }
            | Self::Trading { sell_value, .. }
            | Self::Flash { sell_value, .. }
            | Self::Popcorn { sell_value, .. }
            | Self::Trousers { sell_value, .. }
            | Self::Ancient { sell_value, .. }
            | Self::Ramen { sell_value, .. }
            | Self::Walkie { sell_value, .. }
            | Self::Seltzer { sell_value, .. }
            | Self::Castle { sell_value, .. }
            | Self::Smiley { sell_value, .. }
            | Self::Campfire { sell_value, .. }
            | Self::Ticket { sell_value, .. }
            | Self::Bones { sell_value, .. }
            | Self::Acrobat { sell_value, .. }
            | Self::Sock { sell_value, .. }
            | Self::Swashbuckler { sell_value, .. }
            | Self::Troubadour { sell_value, .. }
            | Self::Certificate { sell_value, .. }
            | Self::Smeared { sell_value, .. }
            | Self::Throwback { sell_value, .. }
            | Self::Chad { sell_value, .. }
            | Self::Gem { sell_value, .. }
            | Self::Bloodstone { sell_value, .. }
            | Self::Arrowhead { sell_value, .. }
            | Self::Onyx { sell_value, .. }
            | Self::Glass { sell_value, .. }
            | Self::Showman { sell_value, .. }
            | Self::Flower { sell_value, .. }
            | Self::Blueprint { sell_value, .. }
            | Self::Wee { sell_value, .. }
            | Self::Andy { sell_value, .. }
            | Self::Oops { sell_value, .. }
            | Self::Idol { sell_value, .. }
            | Self::Double { sell_value, .. }
            | Self::Matador { sell_value, .. }
            | Self::Road { sell_value, .. }
            | Self::Duo { sell_value, .. }
            | Self::Trio { sell_value, .. }
            | Self::Family { sell_value, .. }
            | Self::Order { sell_value, .. }
            | Self::Tribe { sell_value, .. }
            | Self::Stuntman { sell_value, .. }
            | Self::Invisible { sell_value, .. }
            | Self::Brainstorm { sell_value, .. }
            | Self::Satellite { sell_value, .. }
            | Self::Shoot { sell_value, .. }
            | Self::License { sell_value, .. }
            | Self::Cartomancer { sell_value, .. }
            | Self::Astronomer { sell_value, .. }
            | Self::Burnt { sell_value, .. }
            | Self::Bootstraps { sell_value, .. }
            | Self::Canio { sell_value, .. }
            | Self::Triboulet { sell_value, .. }
            | Self::Yorick { sell_value, .. }
            | Self::Chicot { sell_value, .. }
            | Self::Perkeo { sell_value, .. } => *sell_value,
        }
    }

    pub fn increase_sell_value(&mut self) {
        match self {
            Self::Joker { mut sell_value, .. }
            | Self::Greedy { mut sell_value, .. }
            | Self::Lusty { mut sell_value, .. }
            | Self::Wrathful { mut sell_value, .. }
            | Self::Gluttonous { mut sell_value, .. }
            | Self::Jolly { mut sell_value, .. }
            | Self::Zany { mut sell_value, .. }
            | Self::Mad { mut sell_value, .. }
            | Self::Crazy { mut sell_value, .. }
            | Self::Droll { mut sell_value, .. }
            | Self::Sly { mut sell_value, .. }
            | Self::Wily { mut sell_value, .. }
            | Self::Clever { mut sell_value, .. }
            | Self::Devious { mut sell_value, .. }
            | Self::Crafty { mut sell_value, .. }
            | Self::Half { mut sell_value, .. }
            | Self::Stencil { mut sell_value, .. }
            | Self::Fingers { mut sell_value, .. }
            | Self::Mime { mut sell_value, .. }
            | Self::Credit { mut sell_value, .. }
            | Self::Dagger { mut sell_value, .. }
            | Self::Banner { mut sell_value, .. }
            | Self::Mystic { mut sell_value, .. }
            | Self::Marble { mut sell_value, .. }
            | Self::Loyalty { mut sell_value, .. }
            | Self::Ball { mut sell_value, .. }
            | Self::Misprint { mut sell_value, .. }
            | Self::Dusk { mut sell_value, .. }
            | Self::Fist { mut sell_value, .. }
            | Self::Chaos { mut sell_value, .. }
            | Self::Fibonacci { mut sell_value, .. }
            | Self::Steel { mut sell_value, .. }
            | Self::Scary { mut sell_value, .. }
            | Self::Abstract { mut sell_value, .. }
            | Self::Gratification { mut sell_value, .. }
            | Self::Hack { mut sell_value, .. }
            | Self::Pareidolia { mut sell_value, .. }
            | Self::Michel { mut sell_value, .. }
            | Self::Steven { mut sell_value, .. }
            | Self::Todd { mut sell_value, .. }
            | Self::Scholar { mut sell_value, .. }
            | Self::Business { mut sell_value, .. }
            | Self::Supernova { mut sell_value, .. }
            | Self::Bus { mut sell_value, .. }
            | Self::Space { mut sell_value, .. }
            | Self::Egg { mut sell_value, .. }
            | Self::Burglar { mut sell_value, .. }
            | Self::Blackboard { mut sell_value, .. }
            | Self::Runner { mut sell_value, .. }
            | Self::Cream { mut sell_value, .. }
            | Self::DNA { mut sell_value, .. }
            | Self::Splash { mut sell_value, .. }
            | Self::Blue { mut sell_value, .. }
            | Self::Sixth { mut sell_value, .. }
            | Self::Constellation { mut sell_value, .. }
            | Self::Hiker { mut sell_value, .. }
            | Self::Faceless { mut sell_value, .. }
            | Self::Green { mut sell_value, .. }
            | Self::Superposition { mut sell_value, .. }
            | Self::List { mut sell_value, .. }
            | Self::Cavendish { mut sell_value, .. }
            | Self::Sharp { mut sell_value, .. }
            | Self::Red { mut sell_value, .. }
            | Self::Madness { mut sell_value, .. }
            | Self::Square { mut sell_value, .. }
            | Self::Seance { mut sell_value, .. }
            | Self::Riff { mut sell_value, .. }
            | Self::Vampire { mut sell_value, .. }
            | Self::Shortcut { mut sell_value, .. }
            | Self::Hologram { mut sell_value, .. }
            | Self::Vagabond { mut sell_value, .. }
            | Self::Baron { mut sell_value, .. }
            | Self::Cloud { mut sell_value, .. }
            | Self::Rocket { mut sell_value, .. }
            | Self::Obelisk { mut sell_value, .. }
            | Self::Midas { mut sell_value, .. }
            | Self::Luchador { mut sell_value, .. }
            | Self::Photograph { mut sell_value, .. }
            | Self::Gift { mut sell_value, .. }
            | Self::Turtle { mut sell_value, .. }
            | Self::Erosion { mut sell_value, .. }
            | Self::Parking { mut sell_value, .. }
            | Self::Rebate { mut sell_value, .. }
            | Self::Moon { mut sell_value, .. }
            | Self::Hallucination { mut sell_value, .. }
            | Self::Fortune { mut sell_value, .. }
            | Self::Juggler { mut sell_value, .. }
            | Self::Drunkard { mut sell_value, .. }
            | Self::Stone { mut sell_value, .. }
            | Self::Golden { mut sell_value, .. }
            | Self::Lucky { mut sell_value, .. }
            | Self::Baseball { mut sell_value, .. }
            | Self::Bull { mut sell_value, .. }
            | Self::Cola { mut sell_value, .. }
            | Self::Trading { mut sell_value, .. }
            | Self::Flash { mut sell_value, .. }
            | Self::Popcorn { mut sell_value, .. }
            | Self::Trousers { mut sell_value, .. }
            | Self::Ancient { mut sell_value, .. }
            | Self::Ramen { mut sell_value, .. }
            | Self::Walkie { mut sell_value, .. }
            | Self::Seltzer { mut sell_value, .. }
            | Self::Castle { mut sell_value, .. }
            | Self::Smiley { mut sell_value, .. }
            | Self::Campfire { mut sell_value, .. }
            | Self::Ticket { mut sell_value, .. }
            | Self::Bones { mut sell_value, .. }
            | Self::Acrobat { mut sell_value, .. }
            | Self::Sock { mut sell_value, .. }
            | Self::Swashbuckler { mut sell_value, .. }
            | Self::Troubadour { mut sell_value, .. }
            | Self::Certificate { mut sell_value, .. }
            | Self::Smeared { mut sell_value, .. }
            | Self::Throwback { mut sell_value, .. }
            | Self::Chad { mut sell_value, .. }
            | Self::Gem { mut sell_value, .. }
            | Self::Bloodstone { mut sell_value, .. }
            | Self::Arrowhead { mut sell_value, .. }
            | Self::Onyx { mut sell_value, .. }
            | Self::Glass { mut sell_value, .. }
            | Self::Showman { mut sell_value, .. }
            | Self::Flower { mut sell_value, .. }
            | Self::Blueprint { mut sell_value, .. }
            | Self::Wee { mut sell_value, .. }
            | Self::Andy { mut sell_value, .. }
            | Self::Oops { mut sell_value, .. }
            | Self::Idol { mut sell_value, .. }
            | Self::Double { mut sell_value, .. }
            | Self::Matador { mut sell_value, .. }
            | Self::Road { mut sell_value, .. }
            | Self::Duo { mut sell_value, .. }
            | Self::Trio { mut sell_value, .. }
            | Self::Family { mut sell_value, .. }
            | Self::Order { mut sell_value, .. }
            | Self::Tribe { mut sell_value, .. }
            | Self::Stuntman { mut sell_value, .. }
            | Self::Invisible { mut sell_value, .. }
            | Self::Brainstorm { mut sell_value, .. }
            | Self::Satellite { mut sell_value, .. }
            | Self::Shoot { mut sell_value, .. }
            | Self::License { mut sell_value, .. }
            | Self::Cartomancer { mut sell_value, .. }
            | Self::Astronomer { mut sell_value, .. }
            | Self::Burnt { mut sell_value, .. }
            | Self::Bootstraps { mut sell_value, .. }
            | Self::Canio { mut sell_value, .. }
            | Self::Triboulet { mut sell_value, .. }
            | Self::Yorick { mut sell_value, .. }
            | Self::Chicot { mut sell_value, .. }
            | Self::Perkeo { mut sell_value, .. } => sell_value += 1,
        }
    }

    pub fn name(&self) -> &'static str {
        match self {
            Self::Joker { .. } => "Joker",
            Self::Greedy { .. } => "Greedy Joker",
            Self::Lusty { .. } => "Lusty Joker",
            Self::Wrathful { .. } => "Wrathful Joker",
            Self::Gluttonous { .. } => "Gluttonous Joker",
            Self::Jolly { .. } => "Jolly Joker",
            Self::Zany { .. } => "Zany Joker",
            Self::Mad { .. } => "Mad Joker",
            Self::Crazy { .. } => "Crazy Joker",
            Self::Droll { .. } => "Droll Joker",
            Self::Sly { .. } => "Sly Joker",
            Self::Wily { .. } => "Wily Joker",
            Self::Clever { .. } => "Clever Joker",
            Self::Devious { .. } => "Devious Joker",
            Self::Crafty { .. } => "Crafty Joker",
            Self::Half { .. } => "Half Joker",
            Self::Stencil { .. } => "Joker Stencil",
            Self::Fingers { .. } => "Four Fingers",
            Self::Mime { .. } => "Mime",
            Self::Credit { .. } => "Credit Card",
            Self::Dagger { .. } => "Ceremonial Dagger",
            Self::Banner { .. } => "Banner",
            Self::Mystic { .. } => "Mystic Summit",
            Self::Marble { .. } => "Marble Joker",
            Self::Loyalty { .. } => "Loyalty Card",
            Self::Ball { .. } => "8 Ball",
            Self::Misprint { .. } => "Misprint",
            Self::Dusk { .. } => "Dusk",
            Self::Fist { .. } => "Raised Fist",
            Self::Chaos { .. } => "Chaos the Clown",
            Self::Fibonacci { .. } => "Fibonacci",
            Self::Steel { .. } => "Steel Joker",
            Self::Scary { .. } => "Scary Face",
            Self::Abstract { .. } => "Abstract Joker",
            Self::Gratification { .. } => "Delayed Gratification",
            Self::Hack { .. } => "Hack",
            Self::Pareidolia { .. } => "Pareidolia",
            Self::Michel { .. } => "Gros Michel",
            Self::Steven { .. } => "Even Steven",
            Self::Todd { .. } => "Odd Todd",
            Self::Scholar { .. } => "Scholar",
            Self::Business { .. } => "Business Card",
            Self::Supernova { .. } => "Supernova",
            Self::Bus { .. } => "Ride the Bus",
            Self::Space { .. } => "Space Joker",
            Self::Egg { .. } => "Egg",
            Self::Burglar { .. } => "Burglar",
            Self::Blackboard { .. } => "Blackboard",
            Self::Runner { .. } => "Runner",
            Self::Cream { .. } => "Ice Cream",
            Self::DNA { .. } => "DNA",
            Self::Splash { .. } => "Splash",
            Self::Blue { .. } => "Blue Joker",
            Self::Sixth { .. } => "Sixth Sense",
            Self::Constellation { .. } => "Constellation",
            Self::Hiker { .. } => "Hiker",
            Self::Faceless { .. } => "Faceless Joker",
            Self::Green { .. } => "Green Joker",
            Self::Superposition { .. } => "Superposition",
            Self::List { .. } => "To Do List",
            Self::Cavendish { .. } => "Cavendish",
            Self::Sharp { .. } => "Card Sharp",
            Self::Red { .. } => "Red Card",
            Self::Madness { .. } => "Madness",
            Self::Square { .. } => "Square Joker",
            Self::Seance { .. } => "Seance",
            Self::Riff { .. } => "Riff-Raff",
            Self::Vampire { .. } => "Vampire",
            Self::Shortcut { .. } => "Shortcut",
            Self::Hologram { .. } => "Hologram",
            Self::Vagabond { .. } => "Vagabond",
            Self::Baron { .. } => "Baron",
            Self::Cloud { .. } => "Cloud 9",
            Self::Rocket { .. } => "Rocket",
            Self::Obelisk { .. } => "Obelisk",
            Self::Midas { .. } => "Midas Mask",
            Self::Luchador { .. } => "Luchador",
            Self::Photograph { .. } => "Photograph",
            Self::Gift { .. } => "Gift Card",
            Self::Turtle { .. } => "Turtle Bean",
            Self::Erosion { .. } => "Erosion",
            Self::Parking { .. } => "Reserved Parking",
            Self::Rebate { .. } => "Mail-In Rebate",
            Self::Moon { .. } => "To the Moon",
            Self::Hallucination { .. } => "Hallucination",
            Self::Fortune { .. } => "Fortune Teller",
            Self::Juggler { .. } => "Juggler",
            Self::Drunkard { .. } => "Drunkard",
            Self::Stone { .. } => "Stone Joker",
            Self::Golden { .. } => "Golden Joker",
            Self::Lucky { .. } => "Lucky Cat",
            Self::Baseball { .. } => "Baseball Card",
            Self::Bull { .. } => "Bull",
            Self::Cola { .. } => "Diet Cola",
            Self::Trading { .. } => "Trading Card",
            Self::Flash { .. } => "Flash Card",
            Self::Popcorn { .. } => "Popcorn",
            Self::Trousers { .. } => "Spare Trousers",
            Self::Ancient { .. } => "Ancient Joker",
            Self::Ramen { .. } => "Ramen",
            Self::Walkie { .. } => "Walkie Talkie",
            Self::Seltzer { .. } => "Seltzer",
            Self::Castle { .. } => "Castle",
            Self::Smiley { .. } => "Smiley Face",
            Self::Campfire { .. } => "Campfire",
            Self::Ticket { .. } => "Golden Ticket",
            Self::Bones { .. } => "Mr Bones",
            Self::Acrobat { .. } => "Acrobat",
            Self::Sock { .. } => "Sock and Buskin",
            Self::Swashbuckler { .. } => "Swashbuckler",
            Self::Troubadour { .. } => "Troubadour",
            Self::Certificate { .. } => "Certificate",
            Self::Smeared { .. } => "Smeared Joker",
            Self::Throwback { .. } => "Throwback",
            Self::Chad { .. } => "Hanging Chad",
            Self::Gem { .. } => "Rough Gem",
            Self::Bloodstone { .. } => "Bloodstone",
            Self::Arrowhead { .. } => "Arrowhead",
            Self::Onyx { .. } => "Onyx Agate",
            Self::Glass { .. } => "Glass Joker",
            Self::Showman { .. } => "Showman",
            Self::Flower { .. } => "Flower Pot",
            Self::Blueprint { .. } => "Blueprint",
            Self::Wee { .. } => "Wee Joker",
            Self::Andy { .. } => "Merry Andy",
            Self::Oops { .. } => "Oops! All 6s",
            Self::Idol { .. } => "The Idol",
            Self::Double { .. } => "Seeing Double",
            Self::Matador { .. } => "Matador",
            Self::Road { .. } => "Hit the Road",
            Self::Duo { .. } => "The Duo",
            Self::Trio { .. } => "The Trio",
            Self::Family { .. } => "The Family",
            Self::Order { .. } => "The Order",
            Self::Tribe { .. } => "The Tribe",
            Self::Stuntman { .. } => "Stuntman",
            Self::Invisible { .. } => "Invisible Joker",
            Self::Brainstorm { .. } => "Brainstorm",
            Self::Satellite { .. } => "Satellite",
            Self::Shoot { .. } => "Shoot the Moon",
            Self::License { .. } => "Driver's License",
            Self::Cartomancer { .. } => "Cartomancer",
            Self::Astronomer { .. } => "Astronomer",
            Self::Burnt { .. } => "Burnt Joker",
            Self::Bootstraps { .. } => "Bootstraps",
            Self::Canio { .. } => "Canio",
            Self::Triboulet { .. } => "Triboulet",
            Self::Yorick { .. } => "Yorick",
            Self::Chicot { .. } => "Chicot",
            Self::Perkeo { .. } => "Perkeo",
        }
    }
}
