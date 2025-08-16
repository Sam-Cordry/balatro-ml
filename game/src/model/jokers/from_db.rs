use super::Joker;

use crate::model::db::{JokerRow, JokerType};

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
