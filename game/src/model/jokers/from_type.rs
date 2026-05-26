use super::Joker;

use crate::model::{db::JokerType, JokerEdition, State, Voucher};
use rand::{distr::StandardUniform, Rng};

impl Joker {
    pub fn create(joker: JokerType, edition: JokerEdition, state: &mut State) -> Self {
        let mut result = match joker {
            JokerType::Joker => Self::Joker {
                sell_value: 1,
                edition,
            },
            JokerType::Greedy => Self::Greedy {
                sell_value: 2,
                edition,
            },
            JokerType::Lusty => Self::Lusty {
                sell_value: 2,
                edition,
            },
            JokerType::Wrathful => Self::Wrathful {
                sell_value: 2,
                edition,
            },
            JokerType::Gluttonous => Self::Gluttonous {
                sell_value: 2,
                edition,
            },
            JokerType::Jolly => Self::Jolly {
                sell_value: 1,
                edition,
            },
            JokerType::Zany => Self::Zany {
                sell_value: 2,
                edition,
            },
            JokerType::Mad => Self::Mad {
                sell_value: 2,
                edition,
            },
            JokerType::Crazy => Self::Crazy {
                sell_value: 2,
                edition,
            },
            JokerType::Droll => Self::Droll {
                sell_value: 2,
                edition,
            },
            JokerType::Sly => Self::Sly {
                sell_value: 1,
                edition,
            },
            JokerType::Wily => Self::Wily {
                sell_value: 2,
                edition,
            },
            JokerType::Clever => Self::Clever {
                sell_value: 2,
                edition,
            },
            JokerType::Devious => Self::Devious {
                sell_value: 2,
                edition,
            },
            JokerType::Crafty => Self::Crafty {
                sell_value: 2,
                edition,
            },
            JokerType::Half => Self::Half {
                sell_value: 2,
                edition,
            },
            JokerType::Stencil => Self::Stencil {
                sell_value: 4,
                edition,
            },
            JokerType::Fingers => Self::Fingers {
                sell_value: 3,
                edition,
            },
            JokerType::Mime => Self::Mime {
                sell_value: 2,
                edition,
            },
            JokerType::Credit => Self::Credit {
                sell_value: 1,
                edition,
            },
            JokerType::Dagger => Self::Dagger {
                sell_value: 3,
                edition,
                mult: 0,
            },
            JokerType::Banner => Self::Banner {
                sell_value: 2,
                edition,
            },
            JokerType::Mystic => Self::Mystic {
                sell_value: 2,
                edition,
            },
            JokerType::Marble => Self::Marble {
                sell_value: 3,
                edition,
            },
            JokerType::Loyalty => Self::Loyalty {
                sell_value: 2,
                edition,
                hands: 2,
            },
            JokerType::Ball => Self::Ball {
                sell_value: 2,
                edition,
            },
            JokerType::Misprint => Self::Misprint {
                sell_value: 2,
                edition,
            },
            JokerType::Dusk => Self::Dusk {
                sell_value: 2,
                edition,
            },
            JokerType::Fist => Self::Fist {
                sell_value: 2,
                edition,
                min: None,
            },
            JokerType::Chaos => Self::Chaos {
                sell_value: 2,
                edition,
            },
            JokerType::Fibonacci => Self::Fibonacci {
                sell_value: 4,
                edition,
            },
            JokerType::Steel => Self::Steel {
                sell_value: 3,
                edition,
            },
            JokerType::Scary => Self::Scary {
                sell_value: 2,
                edition,
            },
            JokerType::Abstract => Self::Abstract {
                sell_value: 2,
                edition,
            },
            JokerType::Gratification => Self::Gratification {
                sell_value: 2,
                edition,
            },
            JokerType::Hack => Self::Hack {
                sell_value: 3,
                edition,
            },
            JokerType::Pareidolia => Self::Pareidolia {
                sell_value: 2,
                edition,
            },
            JokerType::Michel => Self::Michel {
                sell_value: 2,
                edition,
            },
            JokerType::Steven => Self::Steven {
                sell_value: 2,
                edition,
            },
            JokerType::Todd => Self::Todd {
                sell_value: 2,
                edition,
            },
            JokerType::Scholar => Self::Scholar {
                sell_value: 2,
                edition,
            },
            JokerType::Business => Self::Business {
                sell_value: 2,
                edition,
            },
            JokerType::Supernova => Self::Supernova {
                sell_value: 2,
                edition,
            },
            JokerType::Bus => Self::Bus {
                sell_value: 3,
                edition,
                mult: 0,
            },
            JokerType::Space => Self::Space {
                sell_value: 2,
                edition,
            },
            JokerType::Egg => Self::Egg {
                sell_value: 2,
                edition,
            },
            JokerType::Burglar => Self::Burglar {
                sell_value: 3,
                edition,
            },
            JokerType::Blackboard => Self::Blackboard {
                sell_value: 3,
                edition,
            },
            JokerType::Runner => Self::Runner {
                sell_value: 2,
                edition,
                chips: 0,
            },
            JokerType::Cream => Self::Cream {
                sell_value: 2,
                edition,
                chips: 0,
            },
            JokerType::Dna => Self::Dna {
                sell_value: 4,
                edition,
            },
            JokerType::Splash => Self::Splash {
                sell_value: 1,
                edition,
            },
            JokerType::Blue => Self::Blue {
                sell_value: 2,
                edition,
            },
            JokerType::Sixth => Self::Sixth {
                sell_value: 3,
                edition,
            },
            JokerType::Constellation => Self::Constellation {
                sell_value: 3,
                edition,
                xmult: 1,
            },
            JokerType::Hiker => Self::Hiker {
                sell_value: 2,
                edition,
            },
            JokerType::Faceless => Self::Faceless {
                sell_value: 2,
                edition,
            },
            JokerType::Green => Self::Green {
                sell_value: 2,
                edition,
                mult: 0,
            },
            JokerType::Superposition => Self::Superposition {
                sell_value: 2,
                edition,
            },
            JokerType::List => Self::List {
                sell_value: 2,
                edition,
                hand_type: state.rng.sample(StandardUniform),
            },
            JokerType::Cavendish => Self::Cavendish {
                sell_value: 2,
                edition,
            },
            JokerType::Sharp => Self::Sharp {
                sell_value: 3,
                edition,
            },
            JokerType::Red => Self::Red {
                sell_value: 2,
                edition,
                mult: 0,
            },
            JokerType::Madness => Self::Madness {
                sell_value: 3,
                edition,
                xmult: 1,
            },
            JokerType::Square => Self::Square {
                sell_value: 2,
                edition,
                chips: 0,
            },
            JokerType::Seance => Self::Seance {
                sell_value: 3,
                edition,
            },
            JokerType::Riff => Self::Riff {
                sell_value: 3,
                edition,
            },
            JokerType::Vampire => Self::Vampire {
                sell_value: 3,
                edition,
                xmult: 1,
            },
            JokerType::Shortcut => Self::Shortcut {
                sell_value: 3,
                edition,
            },
            JokerType::Hologram => Self::Hologram {
                sell_value: 3,
                edition,
                xmult: 1,
            },
            JokerType::Vagabond => Self::Vagabond {
                sell_value: 4,
                edition,
            },
            JokerType::Baron => Self::Baron {
                sell_value: 4,
                edition,
            },
            JokerType::Cloud => Self::Cloud {
                sell_value: 3,
                edition,
            },
            JokerType::Rocket => Self::Rocket {
                sell_value: 3,
                edition,
                money: 1,
            },
            JokerType::Obelisk => Self::Obelisk {
                sell_value: 4,
                edition,
                xmult: 1,
            },
            JokerType::Midas => Self::Midas {
                sell_value: 3,
                edition,
            },
            JokerType::Luchador => Self::Luchador {
                sell_value: 2,
                edition,
            },
            JokerType::Photograph => Self::Photograph {
                sell_value: 2,
                edition,
                used: false,
            },
            JokerType::Gift => Self::Gift {
                sell_value: 3,
                edition,
            },
            JokerType::Turtle => Self::Turtle {
                sell_value: 3,
                edition,
                hand_size: 2,
            },
            JokerType::Erosion => Self::Erosion {
                sell_value: 3,
                edition,
            },
            JokerType::Parking => Self::Parking {
                sell_value: 3,
                edition,
            },
            JokerType::Rebate => Self::Rebate {
                sell_value: 3,
                edition,
                rank: state.rng.sample(StandardUniform),
            },
            JokerType::Moon => Self::Moon {
                sell_value: 2,
                edition,
            },
            JokerType::Hallucination => Self::Hallucination {
                sell_value: 2,
                edition,
            },
            JokerType::Fortune => Self::Fortune {
                sell_value: 3,
                edition,
            },
            JokerType::Juggler => Self::Juggler {
                sell_value: 2,
                edition,
            },
            JokerType::Drunkard => Self::Drunkard {
                sell_value: 2,
                edition,
            },
            JokerType::Stone => Self::Stone {
                sell_value: 3,
                edition,
            },
            JokerType::Golden => Self::Golden {
                sell_value: 3,
                edition,
            },
            JokerType::Lucky => Self::Lucky {
                sell_value: 3,
                edition,
                xmult: 1,
            },
            JokerType::Baseball => Self::Baseball {
                sell_value: 4,
                edition,
            },
            JokerType::Bull => Self::Bull {
                sell_value: 3,
                edition,
            },
            JokerType::Cola => Self::Cola {
                sell_value: 3,
                edition,
            },
            JokerType::Trading => Self::Trading {
                sell_value: 3,
                edition,
            },
            JokerType::Flash => Self::Flash {
                sell_value: 2,
                edition,
                mult: 0,
            },
            JokerType::Popcorn => Self::Popcorn {
                sell_value: 2,
                edition,
                mult: 20,
            },
            JokerType::Trousers => Self::Trousers {
                sell_value: 3,
                edition,
                mult: 0,
            },
            JokerType::Ancient => Self::Ancient {
                sell_value: 4,
                edition,
                suit: state.rng.sample(StandardUniform),
            },
            JokerType::Ramen => Self::Ramen {
                sell_value: 3,
                edition,
                xmult: 200,
            },
            JokerType::Walkie => Self::Walkie {
                sell_value: 2,
                edition,
            },
            JokerType::Seltzer => Self::Seltzer {
                sell_value: 3,
                edition,
                hands: 10,
            },
            JokerType::Castle => Self::Castle {
                sell_value: 3,
                edition,
                suit: state.rng.sample(StandardUniform),
                chips: 0,
            },
            JokerType::Smiley => Self::Smiley {
                sell_value: 2,
                edition,
            },
            JokerType::Campfire => Self::Campfire {
                sell_value: 4,
                edition,
                xmult: 1,
            },
            JokerType::Ticket => Self::Ticket {
                sell_value: 2,
                edition,
            },
            JokerType::Bones => Self::Bones {
                sell_value: 2,
                edition,
            },
            JokerType::Acrobat => Self::Acrobat {
                sell_value: 3,
                edition,
            },
            JokerType::Sock => Self::Sock {
                sell_value: 3,
                edition,
            },
            JokerType::Swashbuckler => Self::Swashbuckler {
                sell_value: 4,
                edition,
            },
            JokerType::Troubadour => Self::Troubadour {
                sell_value: 3,
                edition,
            },
            JokerType::Certificate => Self::Certificate {
                sell_value: 3,
                edition,
            },
            JokerType::Smeared => Self::Smeared {
                sell_value: 3,
                edition,
            },
            JokerType::Throwback => Self::Throwback {
                sell_value: 3,
                edition,
                xmult: 1,
            },
            JokerType::Chad => Self::Chad {
                sell_value: 2,
                edition,
            },
            JokerType::Gem => Self::Gem {
                sell_value: 3,
                edition,
            },
            JokerType::Bloodstone => Self::Bloodstone {
                sell_value: 3,
                edition,
            },
            JokerType::Arrowhead => Self::Arrowhead {
                sell_value: 3,
                edition,
            },
            JokerType::Onyx => Self::Onyx {
                sell_value: 3,
                edition,
            },
            JokerType::Glass => Self::Glass {
                sell_value: 3,
                edition,
                xmult: 1,
            },
            JokerType::Showman => Self::Showman {
                sell_value: 2,
                edition,
            },
            JokerType::Flower => Self::Flower {
                sell_value: 3,
                edition,
            },
            JokerType::Blueprint => Self::Blueprint {
                sell_value: 5,
                edition,
            },
            JokerType::Wee => Self::Wee {
                sell_value: 4,
                edition,
                chips: 0,
            },
            JokerType::Andy => Self::Andy {
                sell_value: 3,
                edition,
            },
            JokerType::Oops => Self::Oops {
                sell_value: 2,
                edition,
            },
            JokerType::Idol => Self::Idol {
                sell_value: 3,
                edition,
                rank: state.rng.sample(StandardUniform),
                suit: state.rng.sample(StandardUniform),
            },
            JokerType::Double => Self::Double {
                sell_value: 3,
                edition,
            },
            JokerType::Matador => Self::Matador {
                sell_value: 3,
                edition,
            },
            JokerType::Road => Self::Road {
                sell_value: 4,
                edition,
                xmult: 1,
            },
            JokerType::Duo => Self::Duo {
                sell_value: 4,
                edition,
            },
            JokerType::Trio => Self::Trio {
                sell_value: 4,
                edition,
            },
            JokerType::Family => Self::Family {
                sell_value: 4,
                edition,
            },
            JokerType::Order => Self::Order {
                sell_value: 4,
                edition,
            },
            JokerType::Tribe => Self::Tribe {
                sell_value: 4,
                edition,
            },
            JokerType::Stuntman => Self::Stuntman {
                sell_value: 3,
                edition,
            },
            JokerType::Invisible => Self::Invisible {
                sell_value: 4,
                edition,
                rounds: 0,
            },
            JokerType::Brainstorm => Self::Brainstorm {
                sell_value: 5,
                edition,
            },
            JokerType::Satellite => Self::Satellite {
                sell_value: 3,
                edition,
            },
            JokerType::Shoot => Self::Shoot {
                sell_value: 2,
                edition,
            },
            JokerType::License => Self::License {
                sell_value: 3,
                edition,
            },
            JokerType::Cartomancer => Self::Cartomancer {
                sell_value: 3,
                edition,
            },
            JokerType::Astronomer => Self::Astronomer {
                sell_value: 3,
                edition,
            },
            JokerType::Burnt => Self::Burnt {
                sell_value: 4,
                edition,
            },
            JokerType::Bootstraps => Self::Bootstraps {
                sell_value: 3,
                edition,
            },
            JokerType::Canio => Self::Canio {
                sell_value: 10,
                edition,
                xmult: 1,
            },
            JokerType::Triboulet => Self::Triboulet {
                sell_value: 10,
                edition,
            },
            JokerType::Yorick => Self::Yorick {
                sell_value: 10,
                edition,
                discards: 23,
                xmult: 1,
            },
            JokerType::Chicot => Self::Chicot {
                sell_value: 10,
                edition,
            },
            JokerType::Perkeo => Self::Perkeo {
                sell_value: 10,
                edition,
            },
        };

        if state.redeemed_vouchers.contains(&Voucher::Liquidation) {
            result.set_sell_value(result.get_sell_value() / 2);
        } else if state.redeemed_vouchers.contains(&Voucher::ClearanceSale) {
            result.set_sell_value(result.get_sell_value() * 3 / 4);
        }

        result
    }
}
