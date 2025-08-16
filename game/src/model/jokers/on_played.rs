use super::Joker;

use crate::model::{
    cards::{Card, Enhancement, Rank},
    HandType, State,
};
use rand::Rng;
use std::collections::HashSet;

impl Joker {
    pub fn on_played(
        &mut self,
        state: &mut State,
        played_cards: &mut [Card],
        scoring_cards: &mut [Card],
        hand_type_played: &HandType,
    ) {
        match self {
            Self::Fist { min, .. } => {
                *min = Some(state.hand.iter().map(|c| c.rank).min().unwrap());
            }
            Self::Bus { mult, .. } => {
                if scoring_cards
                    .iter()
                    .filter(|c| c.is_face_card(&state.jokers))
                    .count()
                    == 0
                {
                    *mult += 1;
                } else {
                    *mult = 0;
                }
            }
            Self::Space { .. } => {
                if state.rng.random_range(0..4) == 0 {
                    state.scoring.level_hand(hand_type_played, 1);
                }
            }
            Self::Runner { chips, .. } => {
                if *hand_type_played == HandType::Straight
                    || *hand_type_played == HandType::StraightFlush
                {
                    *chips += 15;
                }
            }
            Self::Dna { .. } => {
                if state.hands_remaining == state.hands - 1 && played_cards.len() == 1 {
                    state.deck.push(*played_cards.first().unwrap());
                    state.hand.push(*played_cards.first().unwrap());
                }
            }
            // Self::Splash { .. } => {
            //     *scoring_cards = *played_cards;
            // }
            Self::Sixth { .. } => {
                if played_cards.len() == 1 && played_cards.first().unwrap().rank == Rank::Six {
                    state.deck.remove(
                        state
                            .deck
                            .iter()
                            .position(|c| c == played_cards.first().unwrap())
                            .unwrap(),
                    );
                }
            }
            Self::Green { mult, .. } => *mult += 1,
            Self::List { hand_type, .. } => {
                if *hand_type == *hand_type_played {
                    state.money += 4;
                }
            }
            Self::Square { chips, .. } => {
                if played_cards.len() == 4 {
                    *chips += 4;
                }
            }
            Self::Vampire { xmult, .. } => {
                scoring_cards
                    .iter_mut()
                    .filter(|c| c.enhancement.is_some())
                    .for_each(|c| {
                        c.enhancement = None;
                        *xmult += 1;
                    });
            }
            Self::Obelisk { xmult, .. } => {
                if state
                    .scoring
                    .scoring_times
                    .iter()
                    .filter(|e| {
                        e.1 == state
                            .scoring
                            .scoring_times
                            .iter()
                            .map(|e| e.1)
                            .max_by(|x, y| x.cmp(y))
                            .unwrap()
                    })
                    .map(|(k, _)| k)
                    .any(|k| *k == *hand_type_played)
                {
                    *xmult = 5;
                } else {
                    *xmult += 1;
                }
            }
            Self::Midas { .. } => {
                played_cards
                    .iter_mut()
                    .filter(|c| c.is_face_card(&state.jokers))
                    .for_each(|c| c.enhancement = Some(Enhancement::Gold));
            }
            Self::Photograph { used, .. } => *used = false,
            Self::Trousers { mult, .. } => {
                let s: HashSet<Rank> = HashSet::from_iter(played_cards.iter().map(|c| c.rank));
                let mut result: u8 = 0;
                if s.len() < played_cards.len() - 1 {
                    for v in s {
                        if played_cards
                            .iter()
                            .filter(|c| c.rank == v)
                            .collect::<Vec<&Card>>()
                            .len()
                            >= 2
                        {
                            result += 1;
                        }
                    }
                }

                if result == 2 {
                    *mult += 2;
                }
            }
            _ => {}
        }
    }
}
