use super::Joker;

use crate::model::{
    cards::{Card, Enhancement, Rank, Suit},
    spectrals::Spectral,
    tarots::Tarot,
    HandType, ScoreModification, State,
};
use rand::Rng;
use std::{cmp::max, collections::HashSet};

impl Joker {
    pub fn on_independent(
        &mut self,
        state: &mut State,
        played_cards: &mut [Card],
        scoring_cards: &mut [Card],
        hand_type_played: &HandType,
    ) -> ScoreModification {
        let mut modification = ScoreModification::default();
        match self {
            Self::Joker { .. } => modification.mult += 4,
            Self::Jolly { .. } => {
                if HashSet::<Rank>::from_iter(played_cards.iter().cloned().map(|c| c.rank)).len()
                    < played_cards.len()
                {
                    modification.mult += 8;
                }
            }
            Self::Zany { .. } => {
                let s: HashSet<Rank> = HashSet::from_iter(played_cards.iter().map(|c| c.rank));
                let mut result: bool = false;
                if s.len() < played_cards.len() - 1 {
                    for v in s {
                        result |= played_cards
                            .iter()
                            .filter(|c| c.rank == v)
                            .collect::<Vec<&Card>>()
                            .len()
                            >= 3;
                    }
                }

                if result {
                    modification.mult += 12;
                }
            }
            Self::Mad { .. } => {
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
                    modification.mult += 10;
                }
            }
            Self::Crazy { .. } => {
                if *hand_type_played == HandType::Straight
                    || *hand_type_played == HandType::StraightFlush
                {
                    modification.mult += 12;
                }
            }
            Self::Droll { .. } => {
                let s: HashSet<Suit> = HashSet::from_iter(played_cards.iter().map(|c| c.suit));
                if s.len() == 1 {
                    modification.mult += 10;
                }
            }
            Self::Sly { .. } => {
                if HashSet::<Rank>::from_iter(played_cards.iter().cloned().map(|c| c.rank)).len()
                    < played_cards.len()
                {
                    modification.chips += 50;
                }
            }
            Self::Wily { .. } => {
                let s: HashSet<Rank> = HashSet::from_iter(played_cards.iter().map(|c| c.rank));
                let mut result: bool = false;
                if s.len() < played_cards.len() - 1 {
                    for v in s {
                        result |= played_cards
                            .iter()
                            .filter(|c| c.rank == v)
                            .collect::<Vec<&Card>>()
                            .len()
                            >= 3;
                    }
                }

                if result {
                    modification.chips += 100;
                }
            }
            Self::Clever { .. } => {
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
                    modification.chips += 80;
                }
            }
            Self::Devious { .. } => {
                if *hand_type_played == HandType::Straight
                    || *hand_type_played == HandType::StraightFlush
                {
                    modification.chips += 100;
                }
            }
            Self::Crafty { .. } => {
                let s: HashSet<Suit> = HashSet::from_iter(played_cards.iter().map(|c| c.suit));
                if s.len() == 1 {
                    modification.chips += 80;
                }
            }
            Self::Half { .. } => {
                if played_cards.len() <= 3 {
                    modification.mult += 20;
                }
            }
            Self::Stencil { .. } => {
                modification.xmult += (state.jokers.len() - state.joker_slots + 1) as f64;
            }
            Self::Dagger { mult, .. } => modification.mult += *mult as isize,
            Self::Banner { .. } => {
                modification.chips += (state.discards_remaining * 30) as isize;
            }
            Self::Mystic { .. } => {
                if state.discards_remaining == 0 {
                    modification.mult += 15;
                }
            }
            Self::Loyalty { hands, .. } => {
                if *hands == 0 {
                    modification.xmult += 4.0;
                    *hands = 6;
                }
                *hands -= 1;
            }
            Self::Misprint { .. } => {
                modification.mult += state.rng.random_range(0..24) as isize;
            }
            Self::Steel { .. } => {
                modification.xmult += 0.2
                    * state
                        .deck
                        .iter()
                        .filter(|c| c.enhancement == Some(Enhancement::Steel))
                        .count() as f64;
            }
            Self::Abstract { .. } => {
                modification.mult += 3 * state.jokers.len() as isize;
            }
            Self::Michel { .. } => modification.mult += 15,
            Self::Supernova { .. } => {
                modification.mult +=
                    *state.scoring.scoring_times.get(hand_type_played).unwrap() as isize;
            }
            Self::Bus { mult, .. } => modification.mult += *mult as isize,
            Self::Blackboard { .. } => {
                if state
                    .hand
                    .iter()
                    .filter(|c| {
                        !(c.is_suit(Suit::Spade, &state.jokers)
                            || c.is_suit(Suit::Club, &state.jokers))
                    })
                    .count()
                    == 0
                {
                    modification.xmult += 3.0;
                }
            }
            Self::Runner { chips, .. } => {
                modification.chips += *chips as isize;
            }
            Self::Cream { chips, .. } => {
                modification.chips += *chips as isize;
                *chips -= 5;
            }
            Self::Blue { .. } => modification.chips += 2 * state.remaining_deck.len() as isize,
            Self::Constellation { mult, .. } => modification.xmult += (*mult / 10) as f64,
            Self::Green { mult, .. } => modification.mult += *mult as isize,
            Self::Superposition { .. } => {
                if (*hand_type_played == HandType::Straight
                    || *hand_type_played == HandType::StraightFlush)
                    && played_cards.iter().filter(|c| c.rank == Rank::Ace).count() > 0
                    && state.consumables.len() < state.consumable_slots
                {
                    state
                        .consumables
                        .push(Box::from(Tarot::sample(&mut state.rng)));
                }
            }
            Self::Cavendish { .. } => modification.xmult += 3.0,
            Self::Sharp { .. } => {
                if state
                    .scoring
                    .scoring_times
                    .iter()
                    .filter(|e| *e.1 != 0)
                    .map(|e| e.0)
                    .any(|t| *t == *hand_type_played)
                {
                    modification.xmult += 3.0;
                }
            }
            Self::Red { mult, .. } => modification.mult += *mult as isize,
            Self::Madness { xmult, .. } => modification.xmult += (*xmult / 4) as f64,
            Self::Square { chips, .. } => modification.chips += *chips as isize,
            Self::Seance { .. } => {
                if *hand_type_played == HandType::StraightFlush
                    && state.consumables.len() < state.consumable_slots
                {
                    state
                        .consumables
                        .push(Box::from(Spectral::sample(&mut state.rng)));
                }
            }
            Self::Vampire { xmult, .. } => modification.xmult += (*xmult / 10) as f64,
            Self::Hologram { xmult, .. } => modification.xmult += (*xmult / 4) as f64,
            Self::Vagabond { .. } => {
                if state.money < 5 && state.consumables.len() < state.consumable_slots {
                    state
                        .consumables
                        .push(Box::from(Tarot::sample(&mut state.rng)));
                }
            }
            Self::Obelisk { xmult, .. } => modification.xmult += (*xmult / 4) as f64,
            Self::Erosion { .. } => modification.mult += max(52 - state.deck.len(), 0) as isize,
            Self::Fortune { .. } => modification.mult += state.tarots_used as isize,
            Self::Stone { .. } => {
                modification.chips += (state
                    .deck
                    .iter()
                    .filter(|c| c.enhancement == Some(Enhancement::Stone))
                    .count()
                    * 25) as isize;
            }
            Self::Lucky { xmult, .. } => modification.xmult += (*xmult / 4) as f64,
            Self::Bull { .. } => modification.chips += 2 * state.money as isize,
            Self::Flash { mult, .. } => modification.mult += *mult as isize,
            Self::Popcorn { mult, .. } => modification.mult += *mult as isize,
            Self::Trousers { mult, .. } => modification.mult += *mult as isize,
            Self::Ramen { xmult, .. } => modification.xmult += (*xmult / 100) as f64,
            Self::Castle { chips, .. } => modification.chips += *chips as isize,
            Self::Campfire { xmult, .. } => modification.xmult += (*xmult / 4) as f64,
            Self::Acrobat { .. } => {
                if state.hands_remaining == state.hands - 1 {
                    modification.xmult += 3.0;
                }
            }
            Self::Swashbuckler { .. } => {
                modification.mult += (state
                    .jokers
                    .iter()
                    .map(|j| j.get_sell_value())
                    .sum::<usize>()
                    - self.get_sell_value()) as isize
            }
            Self::Throwback { xmult, .. } => modification.xmult += (*xmult / 4) as f64,
            Self::Glass { xmult, .. } => modification.xmult += (*xmult / 4) as f64,
            Self::Flower { .. } => {
                if scoring_cards
                    .iter()
                    .any(|c| c.is_suit(Suit::Spade, &state.jokers))
                    && scoring_cards
                        .iter()
                        .any(|c| c.is_suit(Suit::Heart, &state.jokers))
                    && scoring_cards
                        .iter()
                        .any(|c| c.is_suit(Suit::Club, &state.jokers))
                    && scoring_cards
                        .iter()
                        .any(|c| c.is_suit(Suit::Diamond, &state.jokers))
                {
                    modification.xmult += 3.0;
                }
            }
            Self::Wee { chips, .. } => modification.chips += *chips as isize,
            Self::Double { .. } => {
                if scoring_cards
                    .iter()
                    .any(|c| c.is_suit(Suit::Club, &state.jokers))
                    && scoring_cards
                        .iter()
                        .any(|c| !c.is_suit(Suit::Club, &state.jokers))
                {
                    modification.xmult += 2.0;
                }
            }
            Self::Road { xmult, .. } => modification.xmult += (*xmult / 2) as f64,
            Self::Duo { .. } => {
                if HashSet::<Rank>::from_iter(scoring_cards.iter().map(|c| c.rank)).len()
                    < scoring_cards.len()
                {
                    modification.xmult += 2.0;
                }
            }
            Self::Trio { .. } => {
                let s: HashSet<Rank> = HashSet::from_iter(scoring_cards.iter().map(|c| c.rank));
                if s.len() < scoring_cards.len() - 1
                    && scoring_cards
                        .iter()
                        .filter(|c| c.rank == *s.iter().max().unwrap())
                        .count()
                        >= 3
                {
                    modification.xmult += 3.0;
                }
            }
            Self::Family { .. } => {
                let s: HashSet<Rank> = HashSet::from_iter(scoring_cards.iter().map(|c| c.rank));
                if s.len() < scoring_cards.len() - 2 {
                    modification.xmult += 4.0;
                }
            }
            Self::Order { .. } => {
                if *hand_type_played == HandType::Straight
                    || *hand_type_played == HandType::StraightFlush
                {
                    modification.xmult += 3.0;
                }
            }
            Self::Tribe { .. } => {
                if *hand_type_played == HandType::Flush
                    || *hand_type_played == HandType::FlushHouse
                    || *hand_type_played == HandType::FlushFive
                {
                    modification.xmult += 2.0;
                }
            }
            Self::Stuntman { .. } => modification.chips += 100,
            Self::License { .. } => {
                if state
                    .deck
                    .iter()
                    .filter(|c| c.enhancement.is_some())
                    .count()
                    >= 16
                {
                    modification.xmult += 3.0;
                }
            }
            Self::Bootstraps { .. } => modification.mult += ((state.money / 5) * 2) as isize,
            Self::Canio { xmult, .. } => modification.xmult += *xmult as f64,
            Self::Yorick { xmult, .. } => modification.xmult += *xmult as f64,
            _ => {}
        }
        modification
    }
}
