use super::Joker;

use crate::model::{cards::Card, State};

impl Joker {
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
}
