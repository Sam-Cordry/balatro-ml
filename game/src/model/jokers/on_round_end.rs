use super::Joker;

use crate::model::{cards::Rank, State};
use rand::{distr::StandardUniform, prelude::IteratorRandom, Rng};

impl Joker {
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
}
