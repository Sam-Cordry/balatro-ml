use std::fmt::Display;

use crate::model::{
    cards::{Rank, Suit},
    HandType, JokerEdition,
};

pub mod cost;
pub mod edition;
pub mod from_db;
pub mod from_type;
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
pub mod random;
pub mod sell_value;

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Joker {
    #[allow(clippy::enum_variant_names)]
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
        edition: JokerEdition,
        chips: usize,
    },
    Dna {
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
        xmult: usize,
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
        edition: JokerEdition,
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
        hands: usize,
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
