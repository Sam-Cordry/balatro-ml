use super::Joker;

use crate::model::{
    cards::{Card, Enhancement},
    tarots::Tarot,
    traits::Generatable,
    State,
};
use rand::Rng;

impl Joker {
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
            let new = Box::new(Tarot::gen_single(state, false));
            state.consumables.push(new);
        }
    }

    pub fn on_pack_skip(&mut self, state: &State) {
        if let Self::Red { mult, .. } = self {
            *mult += 3;
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
        if let Self::Glass { xmult, .. } = self
            && card.enhancement == Some(Enhancement::Glass)
        {
            *xmult += 3;
        }
    }
}
