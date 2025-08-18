use super::Joker;

use crate::model::{
    cards::{Card, CardEdition, Enhancement},
    tarots::Tarot,
    traits::Generatable,
    State,
};
use rand::{distr::StandardUniform, Rng};

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
                    let new = Box::new(Tarot::gen_single(state, false));
                    state.consumables.push(new);
                }
            }
            _ => {}
        }
    }
}
