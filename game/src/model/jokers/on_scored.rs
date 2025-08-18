use super::Joker;

use crate::model::{
    cards::{Card, Enhancement, Rank, Suit},
    tarots::Tarot,
    traits::Generatable,
    ScoreModification, State,
};
use rand::Rng;

impl Joker {
    pub fn on_scored(&mut self, state: &mut State, card: &mut Card) -> ScoreModification {
        let mut modification = ScoreModification::default();
        match self {
            Self::Greedy { .. } => {
                if card.is_suit(Suit::Diamond, &state.jokers) {
                    modification.mult += 3;
                }
            }
            Self::Lusty { .. } => {
                if card.is_suit(Suit::Heart, &state.jokers) {
                    modification.mult += 3;
                }
            }
            Self::Wrathful { .. } => {
                if card.is_suit(Suit::Spade, &state.jokers) {
                    modification.mult += 3;
                }
            }
            Self::Gluttonous { .. } => {
                if card.is_suit(Suit::Club, &state.jokers) {
                    modification.mult += 3;
                }
            }
            Self::Ball { .. } => {
                if card.rank == Rank::Eight
                    && state.consumables.len() < state.consumable_slots
                    && state.rng.random_range(0..4) == 0
                {
                    let new = Box::new(Tarot::gen_single(state, false));
                    state.consumables.push(new);
                }
            }
            Self::Dusk { .. } => {
                if state.hands_remaining == 0 {
                    modification.triggers += 1;
                }
            }
            Self::Fibonacci { .. } => {
                if card.rank == Rank::Ace
                    || card.rank == Rank::Two
                    || card.rank == Rank::Three
                    || card.rank == Rank::Five
                    || card.rank == Rank::Eight
                {
                    modification.mult += 9;
                }
            }
            Self::Scary { .. } => {
                modification.chips += 30;
            }
            Self::Hack { .. } => {
                if card.rank == Rank::Two
                    || card.rank == Rank::Three
                    || card.rank == Rank::Four
                    || card.rank == Rank::Five
                {
                    modification.triggers += 1;
                }
            }
            Self::Steven { .. } => {
                if card.rank == Rank::Two
                    || card.rank == Rank::Four
                    || card.rank == Rank::Six
                    || card.rank == Rank::Eight
                    || card.rank == Rank::Ten
                {
                    modification.mult += 4;
                }
            }
            Self::Todd { .. } => {
                if card.rank == Rank::Ace
                    || card.rank == Rank::Three
                    || card.rank == Rank::Five
                    || card.rank == Rank::Seven
                    || card.rank == Rank::Nine
                {
                    modification.chips += 31;
                }
            }
            Self::Scholar { .. } => {
                if card.rank == Rank::Ace {
                    modification.chips += 20;
                    modification.mult += 4;
                }
            }
            Self::Business { .. } => {
                if card.is_face_card(&state.jokers) && state.rng.random_range(0..2) == 0 {
                    state.money += 2;
                }
            }
            Self::Hiker { .. } => {
                card.chips += 5;
            }
            Self::Photograph { used, .. } => {
                if !*used && card.is_face_card(&state.jokers) {
                    *used = true;
                    modification.xmult += 2.0;
                }
            }
            Self::Ancient { suit, .. } => {
                if card.is_suit(*suit, &state.jokers) {
                    modification.xmult += 1.5;
                }
            }
            Self::Walkie { .. } => {
                if card.rank == Rank::Ten || card.rank == Rank::Four {
                    modification.chips += 10;
                    modification.mult += 4;
                }
            }
            Self::Seltzer { .. } => modification.triggers += 1,
            Self::Smiley { .. } => {
                if card.is_face_card(&state.jokers) {
                    modification.mult += 5;
                }
            }
            Self::Ticket { .. } => {
                if card.enhancement == Some(Enhancement::Gold) {
                    state.money += 4;
                }
            }
            Self::Sock { .. } => {
                if card.is_face_card(&state.jokers) {
                    modification.triggers += 1;
                }
            }
            Self::Chad { .. } => {
                modification.triggers += 2;
            }
            Self::Gem { .. } => {
                if card.is_suit(Suit::Diamond, &state.jokers) {
                    state.money += 1;
                }
            }
            Self::Bloodstone { .. } => {
                if card.is_suit(Suit::Heart, &state.jokers) && state.rng.random_range(0..2) == 0 {
                    modification.xmult += 1.5;
                }
            }
            Self::Arrowhead { .. } => {
                if card.is_suit(Suit::Spade, &state.jokers) {
                    modification.chips += 50;
                }
            }
            Self::Onyx { .. } => {
                if card.is_suit(Suit::Club, &state.jokers) {
                    modification.mult += 7;
                }
            }
            Self::Wee { chips, .. } => {
                if card.rank == Rank::Two {
                    *chips += 8;
                }
            }
            Self::Idol { rank, suit, .. } => {
                if card.is_suit(*suit, &state.jokers) && card.rank == *rank {
                    modification.xmult += 2.0;
                }
            }
            Self::Triboulet { .. } => {
                if card.rank == Rank::King || card.rank == Rank::Queen {
                    modification.xmult += 2.0;
                }
            }
            _ => {}
        }
        modification
    }
}
