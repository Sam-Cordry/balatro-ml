use rand::{distr::StandardUniform, prelude::IteratorRandom, Rng};
use std::{cmp::max, collections::HashSet, fmt::Display};

use crate::model::{
    cards::{Card, Enhancement, Rank, Suit},
    db::{JokerRow, JokerType},
    spectrals::Spectral,
    tarots::Tarot,
    HandType, JokerEdition, ScoreModification, State,
};

use super::cards::CardEdition;

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Joker {
    #[allow(clippy::enum_variant_names)]
    Joker {
        sell_value: usize,
        edition: JokerEdition,
        index: usize,
    },
    Greedy {
        sell_value: usize,
        edition: JokerEdition,
        index: usize,
    },
    Lusty {
        sell_value: usize,
        edition: JokerEdition,
        index: usize,
    },
    Wrathful {
        sell_value: usize,
        edition: JokerEdition,
        index: usize,
    },
    Gluttonous {
        sell_value: usize,
        edition: JokerEdition,
        index: usize,
    },
    Jolly {
        sell_value: usize,
        edition: JokerEdition,
        index: usize,
    },
    Zany {
        sell_value: usize,
        edition: JokerEdition,
        index: usize,
    },
    Mad {
        sell_value: usize,
        edition: JokerEdition,
        index: usize,
    },
    Crazy {
        sell_value: usize,
        edition: JokerEdition,
        index: usize,
    },
    Droll {
        sell_value: usize,
        edition: JokerEdition,
        index: usize,
    },
    Sly {
        sell_value: usize,
        edition: JokerEdition,
        index: usize,
    },
    Wily {
        sell_value: usize,
        edition: JokerEdition,
        index: usize,
    },
    Clever {
        sell_value: usize,
        edition: JokerEdition,
        index: usize,
    },
    Devious {
        sell_value: usize,
        edition: JokerEdition,
        index: usize,
    },
    Crafty {
        sell_value: usize,
        edition: JokerEdition,
        index: usize,
    },
    Half {
        sell_value: usize,
        edition: JokerEdition,
        index: usize,
    },
    Stencil {
        sell_value: usize,
        edition: JokerEdition,
        index: usize,
    },
    Fingers {
        sell_value: usize,
        edition: JokerEdition,
        index: usize,
    },
    Mime {
        sell_value: usize,
        edition: JokerEdition,
        index: usize,
    },
    Credit {
        sell_value: usize,
        edition: JokerEdition,
        index: usize,
    },
    Dagger {
        sell_value: usize,
        edition: JokerEdition,
        index: usize,
        mult: usize,
    },
    Banner {
        sell_value: usize,
        edition: JokerEdition,
        index: usize,
    },
    Mystic {
        sell_value: usize,
        edition: JokerEdition,
        index: usize,
    },
    Marble {
        sell_value: usize,
        edition: JokerEdition,
        index: usize,
    },
    Loyalty {
        sell_value: usize,
        edition: JokerEdition,
        index: usize,
        hands: u8,
    },
    Ball {
        sell_value: usize,
        edition: JokerEdition,
        index: usize,
    },
    Misprint {
        sell_value: usize,
        edition: JokerEdition,
        index: usize,
    },
    Dusk {
        sell_value: usize,
        edition: JokerEdition,
        index: usize,
    },
    Fist {
        sell_value: usize,
        edition: JokerEdition,
        index: usize,
        min: Option<Rank>,
    },
    Chaos {
        sell_value: usize,
        edition: JokerEdition,
        index: usize,
    },
    Fibonacci {
        sell_value: usize,
        edition: JokerEdition,
        index: usize,
    },
    Steel {
        sell_value: usize,
        edition: JokerEdition,
        index: usize,
    },
    Scary {
        sell_value: usize,
        edition: JokerEdition,
        index: usize,
    },
    Abstract {
        sell_value: usize,
        edition: JokerEdition,
        index: usize,
    },
    Gratification {
        sell_value: usize,
        edition: JokerEdition,
        index: usize,
    },
    Hack {
        sell_value: usize,
        edition: JokerEdition,
        index: usize,
    },
    Pareidolia {
        sell_value: usize,
        edition: JokerEdition,
        index: usize,
    },
    Michel {
        sell_value: usize,
        edition: JokerEdition,
        index: usize,
    },
    Steven {
        sell_value: usize,
        edition: JokerEdition,
        index: usize,
    },
    Todd {
        sell_value: usize,
        edition: JokerEdition,
        index: usize,
    },
    Scholar {
        sell_value: usize,
        edition: JokerEdition,
        index: usize,
    },
    Business {
        sell_value: usize,
        edition: JokerEdition,
        index: usize,
    },
    Supernova {
        sell_value: usize,
        edition: JokerEdition,
        index: usize,
    },
    Bus {
        sell_value: usize,
        edition: JokerEdition,
        index: usize,
        mult: usize,
    },
    Space {
        sell_value: usize,
        edition: JokerEdition,
        index: usize,
    },
    Egg {
        sell_value: usize,
        edition: JokerEdition,
        index: usize,
    },
    Burglar {
        sell_value: usize,
        edition: JokerEdition,
        index: usize,
    },
    Blackboard {
        sell_value: usize,
        edition: JokerEdition,
        index: usize,
    },
    Runner {
        sell_value: usize,
        edition: JokerEdition,
        index: usize,
        chips: usize,
    },
    Cream {
        sell_value: usize,
        edition: JokerEdition,
        index: usize,
        chips: usize,
    },
    Dna {
        sell_value: usize,
        edition: JokerEdition,
        index: usize,
    },
    Splash {
        sell_value: usize,
        edition: JokerEdition,
        index: usize,
    },
    Blue {
        sell_value: usize,
        edition: JokerEdition,
        index: usize,
    },
    Sixth {
        sell_value: usize,
        edition: JokerEdition,
        index: usize,
    },
    Constellation {
        sell_value: usize,
        edition: JokerEdition,
        index: usize,
        mult: usize,
    },
    Hiker {
        sell_value: usize,
        edition: JokerEdition,
        index: usize,
    },
    Faceless {
        sell_value: usize,
        edition: JokerEdition,
        index: usize,
    },
    Green {
        sell_value: usize,
        edition: JokerEdition,
        index: usize,
        mult: usize,
    },
    Superposition {
        sell_value: usize,
        edition: JokerEdition,
        index: usize,
    },
    List {
        sell_value: usize,
        edition: JokerEdition,
        index: usize,
        hand_type: HandType,
    },
    Cavendish {
        sell_value: usize,
        edition: JokerEdition,
        index: usize,
    },
    Sharp {
        sell_value: usize,
        edition: JokerEdition,
        index: usize,
    },
    Red {
        sell_value: usize,
        edition: JokerEdition,
        index: usize,
        mult: usize,
    },
    Madness {
        sell_value: usize,
        edition: JokerEdition,
        index: usize,
        xmult: usize,
    },
    Square {
        sell_value: usize,
        edition: JokerEdition,
        index: usize,
        chips: usize,
    },
    Seance {
        sell_value: usize,
        edition: JokerEdition,
        index: usize,
    },
    Riff {
        sell_value: usize,
        edition: JokerEdition,
        index: usize,
    },
    Vampire {
        sell_value: usize,
        edition: JokerEdition,
        index: usize,
        xmult: usize,
    },
    Shortcut {
        sell_value: usize,
        edition: JokerEdition,
        index: usize,
    },
    Hologram {
        sell_value: usize,
        edition: JokerEdition,
        index: usize,
        xmult: usize,
    },
    Vagabond {
        sell_value: usize,
        edition: JokerEdition,
        index: usize,
    },
    Baron {
        sell_value: usize,
        edition: JokerEdition,
        index: usize,
    },
    Cloud {
        sell_value: usize,
        edition: JokerEdition,
        index: usize,
    },
    Rocket {
        sell_value: usize,
        edition: JokerEdition,
        index: usize,
        money: usize,
    },
    Obelisk {
        sell_value: usize,
        edition: JokerEdition,
        index: usize,
        xmult: usize,
    },
    Midas {
        sell_value: usize,
        edition: JokerEdition,
        index: usize,
    },
    Luchador {
        sell_value: usize,
        edition: JokerEdition,
        index: usize,
    },
    Photograph {
        sell_value: usize,
        edition: JokerEdition,
        index: usize,
        used: bool,
    },
    Gift {
        sell_value: usize,
        edition: JokerEdition,
        index: usize,
    },
    Turtle {
        sell_value: usize,
        edition: JokerEdition,
        index: usize,
        hand_size: u8,
    },
    Erosion {
        sell_value: usize,
        edition: JokerEdition,
        index: usize,
    },
    Parking {
        sell_value: usize,
        edition: JokerEdition,
        index: usize,
    },
    Rebate {
        sell_value: usize,
        edition: JokerEdition,
        index: usize,
        rank: Rank,
    },
    Moon {
        sell_value: usize,
        edition: JokerEdition,
        index: usize,
    },
    Hallucination {
        sell_value: usize,
        edition: JokerEdition,
        index: usize,
    },
    Fortune {
        sell_value: usize,
        edition: JokerEdition,
        index: usize,
    },
    Juggler {
        sell_value: usize,
        edition: JokerEdition,
        index: usize,
    },
    Drunkard {
        sell_value: usize,
        edition: JokerEdition,
        index: usize,
    },
    Stone {
        sell_value: usize,
        edition: JokerEdition,
        index: usize,
    },
    Golden {
        sell_value: usize,
        edition: JokerEdition,
        index: usize,
    },
    Lucky {
        sell_value: usize,
        edition: JokerEdition,
        index: usize,
        xmult: usize,
    },
    Baseball {
        sell_value: usize,
        edition: JokerEdition,
        index: usize,
    },
    Bull {
        sell_value: usize,
        edition: JokerEdition,
        index: usize,
    },
    Cola {
        sell_value: usize,
        edition: JokerEdition,
        index: usize,
    },
    Trading {
        sell_value: usize,
        edition: JokerEdition,
        index: usize,
    },
    Flash {
        sell_value: usize,
        edition: JokerEdition,
        index: usize,
        mult: usize,
    },
    Popcorn {
        sell_value: usize,
        edition: JokerEdition,
        index: usize,
        mult: usize,
    },
    Trousers {
        sell_value: usize,
        edition: JokerEdition,
        index: usize,
        mult: usize,
    },
    Ancient {
        sell_value: usize,
        edition: JokerEdition,
        index: usize,
        suit: Suit,
    },
    Ramen {
        sell_value: usize,
        edition: JokerEdition,
        index: usize,
        xmult: usize,
    },
    Walkie {
        sell_value: usize,
        edition: JokerEdition,
        index: usize,
    },
    Seltzer {
        sell_value: usize,
        edition: JokerEdition,
        index: usize,
    },
    Castle {
        sell_value: usize,
        edition: JokerEdition,
        index: usize,
        suit: Suit,
        chips: usize,
    },
    Smiley {
        sell_value: usize,
        edition: JokerEdition,
        index: usize,
    },
    Campfire {
        sell_value: usize,
        edition: JokerEdition,
        index: usize,
        xmult: usize,
    },
    Ticket {
        sell_value: usize,
        edition: JokerEdition,
        index: usize,
    },
    Bones {
        sell_value: usize,
        edition: JokerEdition,
        index: usize,
    },
    Acrobat {
        sell_value: usize,
        edition: JokerEdition,
        index: usize,
    },
    Sock {
        sell_value: usize,
        edition: JokerEdition,
        index: usize,
    },
    Swashbuckler {
        sell_value: usize,
        edition: JokerEdition,
        index: usize,
    },
    Troubadour {
        sell_value: usize,
        edition: JokerEdition,
        index: usize,
    },
    Certificate {
        sell_value: usize,
        edition: JokerEdition,
        index: usize,
    },
    Smeared {
        sell_value: usize,
        edition: JokerEdition,
        index: usize,
    },
    Throwback {
        sell_value: usize,
        edition: JokerEdition,
        index: usize,
        xmult: usize,
    },
    Chad {
        sell_value: usize,
        edition: JokerEdition,
        index: usize,
    },
    Gem {
        sell_value: usize,
        edition: JokerEdition,
        index: usize,
    },
    Bloodstone {
        sell_value: usize,
        edition: JokerEdition,
        index: usize,
    },
    Arrowhead {
        sell_value: usize,
        edition: JokerEdition,
        index: usize,
    },
    Onyx {
        sell_value: usize,
        edition: JokerEdition,
        index: usize,
    },
    Glass {
        sell_value: usize,
        edition: JokerEdition,
        index: usize,
        xmult: usize,
    },
    Showman {
        sell_value: usize,
        edition: JokerEdition,
        index: usize,
    },
    Flower {
        sell_value: usize,
        edition: JokerEdition,
        index: usize,
    },
    Blueprint {
        sell_value: usize,
        edition: JokerEdition,
        index: usize,
    },
    Wee {
        sell_value: usize,
        edition: JokerEdition,
        index: usize,
        chips: usize,
    },
    Andy {
        sell_value: usize,
        edition: JokerEdition,
        index: usize,
    },
    Oops {
        sell_value: usize,
        edition: JokerEdition,
        index: usize,
    },
    Idol {
        sell_value: usize,
        edition: JokerEdition,
        index: usize,
        rank: Rank,
        suit: Suit,
    },
    Double {
        sell_value: usize,
        edition: JokerEdition,
        index: usize,
    },
    Matador {
        sell_value: usize,
        edition: JokerEdition,
        index: usize,
    },
    Road {
        sell_value: usize,
        edition: JokerEdition,
        index: usize,
        xmult: usize,
    },
    Duo {
        sell_value: usize,
        edition: JokerEdition,
        index: usize,
    },
    Trio {
        sell_value: usize,
        edition: JokerEdition,
        index: usize,
    },
    Family {
        sell_value: usize,
        edition: JokerEdition,
        index: usize,
    },
    Order {
        sell_value: usize,
        edition: JokerEdition,
        index: usize,
    },
    Tribe {
        sell_value: usize,
        edition: JokerEdition,
        index: usize,
    },
    Stuntman {
        sell_value: usize,
        edition: JokerEdition,
        index: usize,
    },
    Invisible {
        sell_value: usize,
        edition: JokerEdition,
        index: usize,
        rounds: usize,
    },
    Brainstorm {
        sell_value: usize,
        edition: JokerEdition,
        index: usize,
    },
    Satellite {
        sell_value: usize,
        edition: JokerEdition,
        index: usize,
    },
    Shoot {
        sell_value: usize,
        edition: JokerEdition,
        index: usize,
    },
    License {
        sell_value: usize,
        edition: JokerEdition,
        index: usize,
    },
    Cartomancer {
        sell_value: usize,
        edition: JokerEdition,
        index: usize,
    },
    Astronomer {
        sell_value: usize,
        edition: JokerEdition,
        index: usize,
    },
    Burnt {
        sell_value: usize,
        edition: JokerEdition,
        index: usize,
    },
    Bootstraps {
        sell_value: usize,
        edition: JokerEdition,
        index: usize,
    },
    Canio {
        sell_value: usize,
        edition: JokerEdition,
        index: usize,
        xmult: usize,
    },
    Triboulet {
        sell_value: usize,
        edition: JokerEdition,
        index: usize,
    },
    Yorick {
        sell_value: usize,
        edition: JokerEdition,
        index: usize,
        discards: usize,
        xmult: usize,
    },
    Chicot {
        sell_value: usize,
        edition: JokerEdition,
        index: usize,
    },
    Perkeo {
        sell_value: usize,
        edition: JokerEdition,
        index: usize,
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
                index: value.index as usize,
            },
            JokerType::Greedy => Self::Greedy {
                sell_value: value.sell_value as usize,
                edition: value.edition,
                index: value.index as usize,
            },
            JokerType::Lusty => Self::Lusty {
                sell_value: value.sell_value as usize,
                edition: value.edition,
                index: value.index as usize,
            },
            JokerType::Wrathful => Self::Wrathful {
                sell_value: value.sell_value as usize,
                edition: value.edition,
                index: value.index as usize,
            },
            JokerType::Gluttonous => Self::Gluttonous {
                sell_value: value.sell_value as usize,
                edition: value.edition,
                index: value.index as usize,
            },
            JokerType::Jolly => Self::Jolly {
                sell_value: value.sell_value as usize,
                edition: value.edition,
                index: value.index as usize,
            },
            JokerType::Zany => Self::Zany {
                sell_value: value.sell_value as usize,
                edition: value.edition,
                index: value.index as usize,
            },
            JokerType::Mad => Self::Mad {
                sell_value: value.sell_value as usize,
                edition: value.edition,
                index: value.index as usize,
            },
            JokerType::Crazy => Self::Crazy {
                sell_value: value.sell_value as usize,
                edition: value.edition,
                index: value.index as usize,
            },
            JokerType::Droll => Self::Droll {
                sell_value: value.sell_value as usize,
                edition: value.edition,
                index: value.index as usize,
            },
            JokerType::Sly => Self::Sly {
                sell_value: value.sell_value as usize,
                edition: value.edition,
                index: value.index as usize,
            },
            JokerType::Wily => Self::Wily {
                sell_value: value.sell_value as usize,
                edition: value.edition,
                index: value.index as usize,
            },
            JokerType::Clever => Self::Clever {
                sell_value: value.sell_value as usize,
                edition: value.edition,
                index: value.index as usize,
            },
            JokerType::Devious => Self::Devious {
                sell_value: value.sell_value as usize,
                edition: value.edition,
                index: value.index as usize,
            },
            JokerType::Crafty => Self::Crafty {
                sell_value: value.sell_value as usize,
                edition: value.edition,
                index: value.index as usize,
            },
            JokerType::Half => Self::Half {
                sell_value: value.sell_value as usize,
                edition: value.edition,
                index: value.index as usize,
            },
            JokerType::Stencil => Self::Stencil {
                sell_value: value.sell_value as usize,
                edition: value.edition,
                index: value.index as usize,
            },
            JokerType::Fingers => Self::Fingers {
                sell_value: value.sell_value as usize,
                edition: value.edition,
                index: value.index as usize,
            },
            JokerType::Mime => Self::Mime {
                sell_value: value.sell_value as usize,
                edition: value.edition,
                index: value.index as usize,
            },
            JokerType::Credit => Self::Credit {
                sell_value: value.sell_value as usize,
                edition: value.edition,
                index: value.index as usize,
            },
            JokerType::Dagger => Self::Dagger {
                sell_value: value.sell_value as usize,
                edition: value.edition,
                index: value.index as usize,
                mult: value.mult.unwrap() as usize,
            },
            JokerType::Banner => Self::Banner {
                sell_value: value.sell_value as usize,
                edition: value.edition,
                index: value.index as usize,
            },
            JokerType::Mystic => Self::Mystic {
                sell_value: value.sell_value as usize,
                edition: value.edition,
                index: value.index as usize,
            },
            JokerType::Marble => Self::Marble {
                sell_value: value.sell_value as usize,
                edition: value.edition,
                index: value.index as usize,
            },
            JokerType::Loyalty => Self::Loyalty {
                sell_value: value.sell_value as usize,
                edition: value.edition,
                index: value.index as usize,
                hands: value.hands.unwrap() as u8,
            },
            JokerType::Ball => Self::Ball {
                sell_value: value.sell_value as usize,
                edition: value.edition,
                index: value.index as usize,
            },
            JokerType::Misprint => Self::Misprint {
                sell_value: value.sell_value as usize,
                edition: value.edition,
                index: value.index as usize,
            },
            JokerType::Dusk => Self::Dusk {
                sell_value: value.sell_value as usize,
                edition: value.edition,
                index: value.index as usize,
            },
            JokerType::Fist => Self::Fist {
                sell_value: value.sell_value as usize,
                edition: value.edition,
                index: value.index as usize,
                min: None,
            },
            JokerType::Chaos => Self::Chaos {
                sell_value: value.sell_value as usize,
                edition: value.edition,
                index: value.index as usize,
            },
            JokerType::Fibonacci => Self::Fibonacci {
                sell_value: value.sell_value as usize,
                edition: value.edition,
                index: value.index as usize,
            },
            JokerType::Steel => Self::Steel {
                sell_value: value.sell_value as usize,
                edition: value.edition,
                index: value.index as usize,
            },
            JokerType::Scary => Self::Scary {
                sell_value: value.sell_value as usize,
                edition: value.edition,
                index: value.index as usize,
            },
            JokerType::Abstract => Self::Abstract {
                sell_value: value.sell_value as usize,
                edition: value.edition,
                index: value.index as usize,
            },
            JokerType::Gratification => Self::Gratification {
                sell_value: value.sell_value as usize,
                edition: value.edition,
                index: value.index as usize,
            },
            JokerType::Hack => Self::Hack {
                sell_value: value.sell_value as usize,
                edition: value.edition,
                index: value.index as usize,
            },
            JokerType::Pareidolia => Self::Pareidolia {
                sell_value: value.sell_value as usize,
                edition: value.edition,
                index: value.index as usize,
            },
            JokerType::Michel => Self::Michel {
                sell_value: value.sell_value as usize,
                edition: value.edition,
                index: value.index as usize,
            },
            JokerType::Steven => Self::Steven {
                sell_value: value.sell_value as usize,
                edition: value.edition,
                index: value.index as usize,
            },
            JokerType::Todd => Self::Todd {
                sell_value: value.sell_value as usize,
                edition: value.edition,
                index: value.index as usize,
            },
            JokerType::Scholar => Self::Scholar {
                sell_value: value.sell_value as usize,
                edition: value.edition,
                index: value.index as usize,
            },
            JokerType::Business => Self::Business {
                sell_value: value.sell_value as usize,
                edition: value.edition,
                index: value.index as usize,
            },
            JokerType::Supernova => Self::Supernova {
                sell_value: value.sell_value as usize,
                edition: value.edition,
                index: value.index as usize,
            },
            JokerType::Bus => Self::Bus {
                sell_value: value.sell_value as usize,
                edition: value.edition,
                index: value.index as usize,
                mult: value.mult.unwrap() as usize,
            },
            JokerType::Space => Self::Space {
                sell_value: value.sell_value as usize,
                edition: value.edition,
                index: value.index as usize,
            },
            JokerType::Egg => Self::Egg {
                sell_value: value.sell_value as usize,
                edition: value.edition,
                index: value.index as usize,
            },
            JokerType::Burglar => Self::Burglar {
                sell_value: value.sell_value as usize,
                edition: value.edition,
                index: value.index as usize,
            },
            JokerType::Blackboard => Self::Blackboard {
                sell_value: value.sell_value as usize,
                edition: value.edition,
                index: value.index as usize,
            },
            JokerType::Runner => Self::Runner {
                sell_value: value.sell_value as usize,
                edition: value.edition,
                index: value.index as usize,
                chips: value.chips.unwrap() as usize,
            },
            JokerType::Cream => Self::Cream {
                sell_value: value.sell_value as usize,
                edition: value.edition,
                index: value.index as usize,
                chips: value.chips.unwrap() as usize,
            },
            JokerType::Dna => Self::Dna {
                sell_value: value.sell_value as usize,
                edition: value.edition,
                index: value.index as usize,
            },
            JokerType::Splash => Self::Splash {
                sell_value: value.sell_value as usize,
                edition: value.edition,
                index: value.index as usize,
            },
            JokerType::Blue => Self::Blue {
                sell_value: value.sell_value as usize,
                edition: value.edition,
                index: value.index as usize,
            },
            JokerType::Sixth => Self::Sixth {
                sell_value: value.sell_value as usize,
                edition: value.edition,
                index: value.index as usize,
            },
            JokerType::Constellation => Self::Constellation {
                sell_value: value.sell_value as usize,
                edition: value.edition,
                index: value.index as usize,
                mult: value.xmult.unwrap() as usize,
            },
            JokerType::Hiker => Self::Hiker {
                sell_value: value.sell_value as usize,
                edition: value.edition,
                index: value.index as usize,
            },
            JokerType::Faceless => Self::Faceless {
                sell_value: value.sell_value as usize,
                edition: value.edition,
                index: value.index as usize,
            },
            JokerType::Green => Self::Green {
                sell_value: value.sell_value as usize,
                edition: value.edition,
                index: value.index as usize,
                mult: value.mult.unwrap() as usize,
            },
            JokerType::Superposition => Self::Superposition {
                sell_value: value.sell_value as usize,
                edition: value.edition,
                index: value.index as usize,
            },
            JokerType::List => Self::List {
                sell_value: value.sell_value as usize,
                edition: value.edition,
                index: value.index as usize,
                hand_type: value.hand_type.unwrap(),
            },
            JokerType::Cavendish => Self::Cavendish {
                sell_value: value.sell_value as usize,
                edition: value.edition,
                index: value.index as usize,
            },
            JokerType::Sharp => Self::Sharp {
                sell_value: value.sell_value as usize,
                edition: value.edition,
                index: value.index as usize,
            },
            JokerType::Red => Self::Red {
                sell_value: value.sell_value as usize,
                edition: value.edition,
                index: value.index as usize,
                mult: value.mult.unwrap() as usize,
            },
            JokerType::Madness => Self::Madness {
                sell_value: value.sell_value as usize,
                edition: value.edition,
                index: value.index as usize,
                xmult: value.xmult.unwrap() as usize,
            },
            JokerType::Square => Self::Square {
                sell_value: value.sell_value as usize,
                edition: value.edition,
                index: value.index as usize,
                chips: value.chips.unwrap() as usize,
            },
            JokerType::Seance => Self::Seance {
                sell_value: value.sell_value as usize,
                edition: value.edition,
                index: value.index as usize,
            },
            JokerType::Riff => Self::Riff {
                sell_value: value.sell_value as usize,
                edition: value.edition,
                index: value.index as usize,
            },
            JokerType::Vampire => Self::Vampire {
                sell_value: value.sell_value as usize,
                edition: value.edition,
                index: value.index as usize,
                xmult: value.xmult.unwrap() as usize,
            },
            JokerType::Shortcut => Self::Shortcut {
                sell_value: value.sell_value as usize,
                edition: value.edition,
                index: value.index as usize,
            },
            JokerType::Hologram => Self::Hologram {
                sell_value: value.sell_value as usize,
                edition: value.edition,
                index: value.index as usize,
                xmult: value.xmult.unwrap() as usize,
            },
            JokerType::Vagabond => Self::Vagabond {
                sell_value: value.sell_value as usize,
                edition: value.edition,
                index: value.index as usize,
            },
            JokerType::Baron => Self::Baron {
                sell_value: value.sell_value as usize,
                edition: value.edition,
                index: value.index as usize,
            },
            JokerType::Cloud => Self::Cloud {
                sell_value: value.sell_value as usize,
                edition: value.edition,
                index: value.index as usize,
            },
            JokerType::Rocket => Self::Rocket {
                sell_value: value.sell_value as usize,
                edition: value.edition,
                index: value.index as usize,
                money: value.money.unwrap() as usize,
            },
            JokerType::Obelisk => Self::Obelisk {
                sell_value: value.sell_value as usize,
                edition: value.edition,
                index: value.index as usize,
                xmult: value.xmult.unwrap() as usize,
            },
            JokerType::Midas => Self::Midas {
                sell_value: value.sell_value as usize,
                edition: value.edition,
                index: value.index as usize,
            },
            JokerType::Luchador => Self::Luchador {
                sell_value: value.sell_value as usize,
                edition: value.edition,
                index: value.index as usize,
            },
            JokerType::Photograph => Self::Photograph {
                sell_value: value.sell_value as usize,
                edition: value.edition,
                index: value.index as usize,
                used: false,
            },
            JokerType::Gift => Self::Gift {
                sell_value: value.sell_value as usize,
                edition: value.edition,
                index: value.index as usize,
            },
            JokerType::Turtle => Self::Turtle {
                sell_value: value.sell_value as usize,
                edition: value.edition,
                index: value.index as usize,
                hand_size: value.hand_size.unwrap() as u8,
            },
            JokerType::Erosion => Self::Erosion {
                sell_value: value.sell_value as usize,
                edition: value.edition,
                index: value.index as usize,
            },
            JokerType::Parking => Self::Parking {
                sell_value: value.sell_value as usize,
                edition: value.edition,
                index: value.index as usize,
            },
            JokerType::Rebate => Self::Rebate {
                sell_value: value.sell_value as usize,
                edition: value.edition,
                index: value.index as usize,
                rank: value.rank.unwrap(),
            },
            JokerType::Moon => Self::Moon {
                sell_value: value.sell_value as usize,
                edition: value.edition,
                index: value.index as usize,
            },
            JokerType::Hallucination => Self::Hallucination {
                sell_value: value.sell_value as usize,
                edition: value.edition,
                index: value.index as usize,
            },
            JokerType::Fortune => Self::Fortune {
                sell_value: value.sell_value as usize,
                edition: value.edition,
                index: value.index as usize,
            },
            JokerType::Juggler => Self::Juggler {
                sell_value: value.sell_value as usize,
                edition: value.edition,
                index: value.index as usize,
            },
            JokerType::Drunkard => Self::Drunkard {
                sell_value: value.sell_value as usize,
                edition: value.edition,
                index: value.index as usize,
            },
            JokerType::Stone => Self::Stone {
                sell_value: value.sell_value as usize,
                edition: value.edition,
                index: value.index as usize,
            },
            JokerType::Golden => Self::Golden {
                sell_value: value.sell_value as usize,
                edition: value.edition,
                index: value.index as usize,
            },
            JokerType::Lucky => Self::Lucky {
                sell_value: value.sell_value as usize,
                edition: value.edition,
                index: value.index as usize,
                xmult: value.xmult.unwrap() as usize,
            },
            JokerType::Baseball => Self::Baseball {
                sell_value: value.sell_value as usize,
                edition: value.edition,
                index: value.index as usize,
            },
            JokerType::Bull => Self::Bull {
                sell_value: value.sell_value as usize,
                edition: value.edition,
                index: value.index as usize,
            },
            JokerType::Cola => Self::Cola {
                sell_value: value.sell_value as usize,
                edition: value.edition,
                index: value.index as usize,
            },
            JokerType::Trading => Self::Trading {
                sell_value: value.sell_value as usize,
                edition: value.edition,
                index: value.index as usize,
            },
            JokerType::Flash => Self::Flash {
                sell_value: value.sell_value as usize,
                edition: value.edition,
                index: value.index as usize,
                mult: value.mult.unwrap() as usize,
            },
            JokerType::Popcorn => Self::Popcorn {
                sell_value: value.sell_value as usize,
                edition: value.edition,
                index: value.index as usize,
                mult: value.mult.unwrap() as usize,
            },
            JokerType::Trousers => Self::Trousers {
                sell_value: value.sell_value as usize,
                edition: value.edition,
                index: value.index as usize,
                mult: value.mult.unwrap() as usize,
            },
            JokerType::Ancient => Self::Ancient {
                sell_value: value.sell_value as usize,
                edition: value.edition,
                index: value.index as usize,
                suit: value.suit.unwrap(),
            },
            JokerType::Ramen => Self::Ramen {
                sell_value: value.sell_value as usize,
                edition: value.edition,
                index: value.index as usize,
                xmult: value.xmult.unwrap() as usize,
            },
            JokerType::Walkie => Self::Walkie {
                sell_value: value.sell_value as usize,
                edition: value.edition,
                index: value.index as usize,
            },
            JokerType::Seltzer => Self::Seltzer {
                sell_value: value.sell_value as usize,
                edition: value.edition,
                index: value.index as usize,
            },
            JokerType::Castle => Self::Castle {
                sell_value: value.sell_value as usize,
                edition: value.edition,
                index: value.index as usize,
                chips: value.chips.unwrap() as usize,
                suit: value.suit.unwrap(),
            },
            JokerType::Smiley => Self::Smiley {
                sell_value: value.sell_value as usize,
                edition: value.edition,
                index: value.index as usize,
            },
            JokerType::Campfire => Self::Campfire {
                sell_value: value.sell_value as usize,
                edition: value.edition,
                index: value.index as usize,
                xmult: value.xmult.unwrap() as usize,
            },
            JokerType::Ticket => Self::Ticket {
                sell_value: value.sell_value as usize,
                edition: value.edition,
                index: value.index as usize,
            },
            JokerType::Bones => Self::Bones {
                sell_value: value.sell_value as usize,
                edition: value.edition,
                index: value.index as usize,
            },
            JokerType::Acrobat => Self::Acrobat {
                sell_value: value.sell_value as usize,
                edition: value.edition,
                index: value.index as usize,
            },
            JokerType::Sock => Self::Sock {
                sell_value: value.sell_value as usize,
                edition: value.edition,
                index: value.index as usize,
            },
            JokerType::Swashbuckler => Self::Swashbuckler {
                sell_value: value.sell_value as usize,
                edition: value.edition,
                index: value.index as usize,
            },
            JokerType::Troubadour => Self::Troubadour {
                sell_value: value.sell_value as usize,
                edition: value.edition,
                index: value.index as usize,
            },
            JokerType::Certificate => Self::Certificate {
                sell_value: value.sell_value as usize,
                edition: value.edition,
                index: value.index as usize,
            },
            JokerType::Smeared => Self::Smeared {
                sell_value: value.sell_value as usize,
                edition: value.edition,
                index: value.index as usize,
            },
            JokerType::Throwback => Self::Throwback {
                sell_value: value.sell_value as usize,
                edition: value.edition,
                index: value.index as usize,
                xmult: value.xmult.unwrap() as usize,
            },
            JokerType::Chad => Self::Chad {
                sell_value: value.sell_value as usize,
                edition: value.edition,
                index: value.index as usize,
            },
            JokerType::Gem => Self::Gem {
                sell_value: value.sell_value as usize,
                edition: value.edition,
                index: value.index as usize,
            },
            JokerType::Bloodstone => Self::Bloodstone {
                sell_value: value.sell_value as usize,
                edition: value.edition,
                index: value.index as usize,
            },
            JokerType::Arrowhead => Self::Arrowhead {
                sell_value: value.sell_value as usize,
                edition: value.edition,
                index: value.index as usize,
            },
            JokerType::Onyx => Self::Onyx {
                sell_value: value.sell_value as usize,
                edition: value.edition,
                index: value.index as usize,
            },
            JokerType::Glass => Self::Glass {
                sell_value: value.sell_value as usize,
                edition: value.edition,
                index: value.index as usize,
                xmult: value.xmult.unwrap() as usize,
            },
            JokerType::Showman => Self::Showman {
                sell_value: value.sell_value as usize,
                edition: value.edition,
                index: value.index as usize,
            },
            JokerType::Flower => Self::Flower {
                sell_value: value.sell_value as usize,
                edition: value.edition,
                index: value.index as usize,
            },
            JokerType::Blueprint => Self::Blueprint {
                sell_value: value.sell_value as usize,
                edition: value.edition,
                index: value.index as usize,
            },
            JokerType::Wee => Self::Wee {
                sell_value: value.sell_value as usize,
                edition: value.edition,
                index: value.index as usize,
                chips: value.chips.unwrap() as usize,
            },
            JokerType::Andy => Self::Andy {
                sell_value: value.sell_value as usize,
                edition: value.edition,
                index: value.index as usize,
            },
            JokerType::Oops => Self::Oops {
                sell_value: value.sell_value as usize,
                edition: value.edition,
                index: value.index as usize,
            },
            JokerType::Idol => Self::Idol {
                sell_value: value.sell_value as usize,
                edition: value.edition,
                index: value.index as usize,
                rank: value.rank.unwrap(),
                suit: value.suit.unwrap(),
            },
            JokerType::Double => Self::Double {
                sell_value: value.sell_value as usize,
                edition: value.edition,
                index: value.index as usize,
            },
            JokerType::Matador => Self::Matador {
                sell_value: value.sell_value as usize,
                edition: value.edition,
                index: value.index as usize,
            },
            JokerType::Road => Self::Road {
                sell_value: value.sell_value as usize,
                edition: value.edition,
                index: value.index as usize,
                xmult: value.xmult.unwrap() as usize,
            },
            JokerType::Duo => Self::Duo {
                sell_value: value.sell_value as usize,
                edition: value.edition,
                index: value.index as usize,
            },
            JokerType::Trio => Self::Trio {
                sell_value: value.sell_value as usize,
                edition: value.edition,
                index: value.index as usize,
            },
            JokerType::Family => Self::Family {
                sell_value: value.sell_value as usize,
                edition: value.edition,
                index: value.index as usize,
            },
            JokerType::Order => Self::Order {
                sell_value: value.sell_value as usize,
                edition: value.edition,
                index: value.index as usize,
            },
            JokerType::Tribe => Self::Tribe {
                sell_value: value.sell_value as usize,
                edition: value.edition,
                index: value.index as usize,
            },
            JokerType::Stuntman => Self::Stuntman {
                sell_value: value.sell_value as usize,
                edition: value.edition,
                index: value.index as usize,
            },
            JokerType::Invisible => Self::Invisible {
                sell_value: value.sell_value as usize,
                edition: value.edition,
                index: value.index as usize,
                rounds: value.rounds.unwrap() as usize,
            },
            JokerType::Brainstorm => Self::Brainstorm {
                sell_value: value.sell_value as usize,
                edition: value.edition,
                index: value.index as usize,
            },
            JokerType::Satellite => Self::Satellite {
                sell_value: value.sell_value as usize,
                edition: value.edition,
                index: value.index as usize,
            },
            JokerType::Shoot => Self::Shoot {
                sell_value: value.sell_value as usize,
                edition: value.edition,
                index: value.index as usize,
            },
            JokerType::License => Self::License {
                sell_value: value.sell_value as usize,
                edition: value.edition,
                index: value.index as usize,
            },
            JokerType::Cartomancer => Self::Cartomancer {
                sell_value: value.sell_value as usize,
                edition: value.edition,
                index: value.index as usize,
            },
            JokerType::Astronomer => Self::Astronomer {
                sell_value: value.sell_value as usize,
                edition: value.edition,
                index: value.index as usize,
            },
            JokerType::Burnt => Self::Burnt {
                sell_value: value.sell_value as usize,
                edition: value.edition,
                index: value.index as usize,
            },
            JokerType::Bootstraps => Self::Bootstraps {
                sell_value: value.sell_value as usize,
                edition: value.edition,
                index: value.index as usize,
            },
            JokerType::Canio => Self::Canio {
                sell_value: value.sell_value as usize,
                edition: value.edition,
                index: value.index as usize,
                xmult: value.xmult.unwrap() as usize,
            },
            JokerType::Triboulet => Self::Triboulet {
                sell_value: value.sell_value as usize,
                edition: value.edition,
                index: value.index as usize,
            },
            JokerType::Yorick => Self::Yorick {
                sell_value: value.sell_value as usize,
                edition: value.edition,
                index: value.index as usize,
                xmult: value.xmult.unwrap() as usize,
                discards: value.discards.unwrap() as usize,
            },
            JokerType::Chicot => Self::Chicot {
                sell_value: value.sell_value as usize,
                edition: value.edition,
                index: value.index as usize,
            },
            JokerType::Perkeo => Self::Perkeo {
                sell_value: value.sell_value as usize,
                edition: value.edition,
                index: value.index as usize,
            },
        }
    }
}

impl Joker {
    pub fn on_blind_select(&mut self, state: &mut State) {
        match self {
            Self::Dagger { mult, index, .. } => {
                if *index + 1 < state.jokers.len() {
                    *mult += state.jokers.remove(*index + 1).get_sell_value();
                }
            }
            Self::Marble { .. } => {
                let mut card = Card {
                    suit: state.rng.sample(StandardUniform),
                    rank: state.rng.sample(StandardUniform),
                    edition: CardEdition::Base,
                    enhancement: Some(Enhancement::Stone),
                    seal: None,
                    chips: 0,
                };
                card.chips = card.rank.get_value() as usize;
                state.deck.push(card);
                state.remaining_deck.push(card);
            }
            Self::Burglar { .. } => {
                state.hands_remaining += 3;
                state.discards_remaining = 0;
            }
            Self::Madness { xmult, .. } => todo!("blinds"),
            Self::Riff { .. } => todo!("randomness"),
            Self::Certificate { .. } => {
                let mut card = Card {
                    suit: state.rng.sample(StandardUniform),
                    rank: state.rng.sample(StandardUniform),
                    edition: CardEdition::Base,
                    enhancement: None,
                    seal: Some(state.rng.sample(StandardUniform)),
                    chips: 0,
                };
                card.chips = card.rank.get_value() as usize;
                state.deck.push(card);
                state.hand.push(card);
            }
            Self::Cartomancer { .. } => {
                if state.consumables.len() < state.consumable_slots {
                    state
                        .consumables
                        .push(Box::from(Tarot::sample(&mut state.rng)));
                }
            }
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
            Self::Fist { min, .. } => {
                *min = Some(state.hand.iter().map(|c| c.rank).min().unwrap());
            }
            Self::Bus { mult, .. } => {
                if scoring_cards
                    .iter()
                    .filter(|c| c.is_face_card(&state.jokers))
                    .count()
                    == 0
                {
                    *mult += 1;
                } else {
                    *mult = 0;
                }
            }
            Self::Space { .. } => {
                if state.rng.random_range(0..4) == 0 {
                    state.scoring.level_hand(hand_type_played, 1);
                }
            }
            Self::Runner { chips, .. } => {
                if *hand_type_played == HandType::Straight
                    || *hand_type_played == HandType::StraightFlush
                {
                    *chips += 15;
                }
            }
            Self::Dna { .. } => {
                if state.hands_remaining == state.hands - 1 && played_cards.len() == 1 {
                    state.deck.push(*played_cards.first().unwrap());
                    state.hand.push(*played_cards.first().unwrap());
                }
            }
            // Self::Splash { .. } => {
            //     *scoring_cards = *played_cards;
            // }
            Self::Sixth { .. } => {
                if played_cards.len() == 1 && played_cards.first().unwrap().rank == Rank::Six {
                    state.deck.remove(
                        state
                            .deck
                            .iter()
                            .position(|c| c == played_cards.first().unwrap())
                            .unwrap(),
                    );
                }
            }
            Self::Green { mult, .. } => *mult += 1,
            Self::List { hand_type, .. } => {
                if *hand_type == *hand_type_played {
                    state.money += 4;
                }
            }
            Self::Square { chips, .. } => {
                if played_cards.len() == 4 {
                    *chips += 4;
                }
            }
            Self::Vampire { xmult, .. } => {
                scoring_cards
                    .iter_mut()
                    .filter(|c| c.enhancement.is_some())
                    .for_each(|c| {
                        c.enhancement = None;
                        *xmult += 1;
                    });
            }
            Self::Obelisk { xmult, .. } => {
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
                    *xmult = 5;
                } else {
                    *xmult += 1;
                }
            }
            Self::Midas { .. } => {
                played_cards
                    .iter_mut()
                    .filter(|c| c.is_face_card(&state.jokers))
                    .for_each(|c| c.enhancement = Some(Enhancement::Gold));
            }
            Self::Photograph { used, .. } => *used = false,
            Self::Trousers { mult, .. } => {
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
                    *mult += 2;
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
                if card.rank == Rank::Eight
                    && state.consumables.len() < state.consumable_slots
                    && state.rng.random_range(0..4) == 0
                {
                    state
                        .consumables
                        .push(Box::from(Tarot::sample(&mut state.rng)));
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
                if card.is_face_card(&state.jokers) && state.rng.random_range(0..2) == 0 {
                    state.money += 2;
                }
            }
            Self::Hiker { .. } => {
                card.chips += 5;
            }
            Self::Photograph { used, .. } => {
                if !*used && card.is_face_card(&state.jokers) {
                    *used = true;
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
            Self::Bloodstone { .. } => {
                if card.is_suit(Suit::Heart, &state.jokers) && state.rng.random_range(0..2) == 0 {
                    modification.xmult += 1.5;
                }
            }
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
            Self::Wee { chips, .. } => {
                if card.rank == Rank::Two {
                    *chips += 8;
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
            Self::Fist { min, .. } => {
                if let Some(m) = *min {
                    if m == card.rank {
                        modification.mult += (card.rank.get_value() * 2) as isize;
                    }
                    *min = None;
                }
            }
            Self::Baron { .. } => {
                if card.rank == Rank::King {
                    modification.xmult += 1.5;
                }
            }
            Self::Parking { .. } => {
                if card.is_face_card(&state.jokers) && state.rng.random_range(0..2) == 0 {
                    state.money += 1;
                }
            }
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
            Self::Loyalty { hands, .. } => {
                if *hands == 0 {
                    modification.xmult += 4.0;
                    *hands = 6;
                }
                *hands -= 1;
            }
            Self::Misprint { .. } => {
                modification.mult += state.rng.random_range(0..24) as isize;
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
            Self::Cream { chips, .. } => {
                modification.chips += *chips as isize;
                *chips -= 5;
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
                    state
                        .consumables
                        .push(Box::from(Tarot::sample(&mut state.rng)));
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
                    state
                        .consumables
                        .push(Box::from(Spectral::sample(&mut state.rng)));
                }
            }
            Self::Vampire { xmult, .. } => modification.xmult += (*xmult / 10) as f64,
            Self::Hologram { xmult, .. } => modification.xmult += (*xmult / 4) as f64,
            Self::Vagabond { .. } => {
                if state.money < 5 && state.consumables.len() < state.consumable_slots {
                    state
                        .consumables
                        .push(Box::from(Tarot::sample(&mut state.rng)));
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
            Self::Green { mult, .. } => {
                if *mult > 0 {
                    *mult -= 1;
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
                            .position(|c| c == played_cards.first().unwrap())
                            .unwrap(),
                    );
                    state.money += 3;
                }
            }
            Self::Ramen { xmult, .. } => {
                *xmult -= played_cards.len();
                if *xmult <= 100 {
                    state
                        .jokers
                        .remove(state.jokers.iter().position(|j| j == self).unwrap());
                }
            }
            Self::Castle { suit, chips, .. } => {
                played_cards
                    .iter()
                    .filter(|c| c.is_suit(*suit, &state.jokers))
                    .for_each(|_| *chips += 3);
            }
            Self::Road { xmult, .. } => *xmult += 1,
            Self::Yorick {
                discards, xmult, ..
            } => {
                if played_cards.len() >= *discards {
                    *discards += 23 - played_cards.len();
                    *xmult += 1;
                }
                *discards -= played_cards.len();
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
            Self::Michel { .. } => {
                if state.rng.random_range(0..6) == 0 {
                    state
                        .jokers
                        .remove(state.jokers.iter().position(|j| *j == *self).unwrap());
                }
            }
            Self::Egg { sell_value, .. } => *sell_value += 3,
            Self::List { hand_type, .. } => {
                *hand_type = state.rng.sample(StandardUniform);
            }
            Self::Cavendish { .. } => {
                if state.rng.random_range(0..1000) == 0 {
                    state
                        .jokers
                        .remove(state.jokers.iter().position(|j| *j == *self).unwrap());
                }
            }
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
            Self::Turtle { hand_size, .. } => {
                *hand_size -= 1;
                if *hand_size == 0 {
                    state
                        .jokers
                        .remove(state.jokers.iter().position(|j| j == self).unwrap());
                }
            }
            Self::Rebate { rank, .. } => {
                *rank = state.rng.sample(StandardUniform);
            }
            Self::Golden { .. } => state.money += 4,
            Self::Popcorn { mult, .. } => {
                *mult -= 4;
                if *mult == 0 {
                    state
                        .jokers
                        .remove(state.jokers.iter().position(|j| j == self).unwrap());
                }
            }
            Self::Ancient { suit, .. } => {
                *suit = state.rng.sample(StandardUniform);
            }
            Self::Castle { suit, .. } => {
                *suit = state.rng.sample(StandardUniform);
            }
            Self::Campfire { xmult, .. } => todo!("blinds"),
            Self::Idol { rank, suit, .. } => {
                (*rank, *suit) = state
                    .deck
                    .iter()
                    .map(|c| (c.rank, c.suit))
                    .choose(&mut state.rng)
                    .unwrap();
            }
            Self::Road { xmult, .. } => *xmult = 2,
            Self::Invisible { rounds, .. } => *rounds += 1,
            Self::Satellite { .. } => state.money += state.planets_used.len(),
            _ => {}
        }
    }

    pub fn on_shop_close(&mut self, state: &mut State) {}

    pub fn on_shop_reroll(&mut self, state: &mut State) {
        if let Self::Flash { mult, .. } = self {
            *mult += 2;
        }
    }

    pub fn on_pack_open(&mut self, state: &mut State) {
        if let Self::Hallucination { .. } = self
            && state.consumables.len() < state.consumable_slots
            && state.rng.random_range(0..2) == 0
        {
            state
                .consumables
                .push(Box::from(Tarot::sample(&mut state.rng)));
        }
    }

    pub fn on_pack_skip(&mut self, state: &State) {
        if let Self::Red { mult, .. } = self {
            *mult += 3;
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
        if let Self::Constellation { mult, .. } = self {
            *mult += 1;
        }
    }

    pub fn on_card_add(&mut self) {
        if let Self::Hologram { xmult, .. } = self {
            *xmult += 1;
        }
    }

    pub fn on_card_sell(&mut self) {
        if let Self::Campfire { xmult, .. } = self {
            *xmult += 1;
        }
    }

    pub fn on_blind_skip(&mut self) {
        if let Self::Throwback { xmult, .. } = self {
            *xmult += 1;
        }
    }

    pub fn on_card_destroy(&mut self, card: &Card) {
        if let Self::Glass { xmult, .. } = self {
            if card.enhancement == Some(Enhancement::Glass) {
                *xmult += 3;
            }
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
            Self::Dna { .. } => 8,
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
            | Self::Dna { sell_value, .. }
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
            | Self::Dna { sell_value, .. }
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
            | Self::Perkeo { sell_value, .. } => *sell_value += 1,
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
            Self::Dna { .. } => "DNA",
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
