use super::Joker;

use crate::model::db::{JokerRow, JokerType};

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
            JokerType::Crafty => Self::Crafty {
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
                hands: value.hands.unwrap() as u8,
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
                min: None,
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
                mult: value.mult.unwrap() as usize,
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
                chips: value.chips.unwrap() as usize,
            },
            JokerType::Cream => Self::Cream {
                sell_value: value.sell_value as usize,
                edition: value.edition,
                chips: value.chips.unwrap() as usize,
            },
            JokerType::Dna => Self::Dna {
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
                hand_type: value.hand_type.unwrap(),
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
                used: false,
            },
            JokerType::Gift => Self::Gift {
                sell_value: value.sell_value as usize,
                edition: value.edition,
            },
            JokerType::Turtle => Self::Turtle {
                sell_value: value.sell_value as usize,
                edition: value.edition,
                hand_size: value.hand_size.unwrap() as u8,
            },
            JokerType::Erosion => Self::Erosion {
                sell_value: value.sell_value as usize,
                edition: value.edition,
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
                suit: value.suit.unwrap(),
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
                hands: value.hands.unwrap() as usize,
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
                rounds: value.rounds.unwrap() as usize,
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
                discards: value.discards.unwrap() as usize,
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
