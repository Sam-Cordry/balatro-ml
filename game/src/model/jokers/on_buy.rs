use super::Joker;

use crate::model::State;

impl Joker {
    pub fn on_buy(&self, state: &mut State) {
        match self {
            Self::Juggler { .. } => state.hand_size += 1,
            Self::Drunkard { .. } => {
                state.discards += 1;
                state.discards_remaining += 1;
            }
            Self::Troubadour { .. } => {
                state.hand_size += 2;
                state.hands -= 1;
                state.hands_remaining -= 1;
            }
            Self::Andy { .. } => {
                state.discards += 3;
                state.discards_remaining += 3;
                state.hand_size -= 1;
            }
            Self::Stuntman { .. } => state.hand_size -= 2,
            _ => {}
        }
    }
}
