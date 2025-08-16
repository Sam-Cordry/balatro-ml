use super::Joker;

use crate::model::{
    cards::{Card, Rank},
    ScoreModification, State,
};
use rand::Rng;

impl Joker {
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
}
