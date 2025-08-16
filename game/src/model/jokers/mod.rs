use std::fmt::Display;

use crate::model::{
    cards::{Rank, Suit},
    HandType, JokerEdition,
};

pub mod cost;
pub mod from_db;
pub mod name;
pub mod on_blind_select;
pub mod on_buy;
pub mod on_discard;
pub mod on_held;
pub mod on_independent;
pub mod on_misc;
pub mod on_played;
pub mod on_round_end;
pub mod on_scored;
pub mod on_sell;
pub mod sell_value;

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
