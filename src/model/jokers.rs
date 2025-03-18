use std::{cmp::max, collections::HashSet, fmt::Display};

use crate::{
    model::{
        cards::{Card, Enhancement, Rank, Suit},
        Edition, HandType, State,
    },
    traits::Scoreable,
};

#[derive(Debug, PartialEq, Eq)]
pub enum Joker {
    Joker {
        cost: usize,
        sell_rank: usize,
        edition: Edition,
    },
    Greedy {
        cost: usize,
        sell_rank: usize,
        edition: Edition,
    },
    Lusty {
        cost: usize,
        sell_rank: usize,
        edition: Edition,
    },
    Wrathful {
        cost: usize,
        sell_rank: usize,
        edition: Edition,
    },
    Gluttonous {
        cost: usize,
        sell_rank: usize,
        edition: Edition,
    },
    Jolly {
        cost: usize,
        sell_rank: usize,
        edition: Edition,
    },
    Zany {
        cost: usize,
        sell_rank: usize,
        edition: Edition,
    },
    Mad {
        cost: usize,
        sell_rank: usize,
        edition: Edition,
    },
    Crazy {
        cost: usize,
        sell_rank: usize,
        edition: Edition,
    },
    Droll {
        cost: usize,
        sell_rank: usize,
        edition: Edition,
    },
    Sly {
        cost: usize,
        sell_rank: usize,
        edition: Edition,
    },
    Wily {
        cost: usize,
        sell_rank: usize,
        edition: Edition,
    },
    Clever {
        cost: usize,
        sell_rank: usize,
        edition: Edition,
    },
    Devious {
        cost: usize,
        sell_rank: usize,
        edition: Edition,
    },
    Crafty {
        cost: usize,
        sell_rank: usize,
        edition: Edition,
    },
    Half {
        cost: usize,
        sell_rank: usize,
        edition: Edition,
    },
    Stencil {
        cost: usize,
        sell_rank: usize,
        edition: Edition,
    },
    Fingers {
        cost: usize,
        sell_rank: usize,
        edition: Edition,
    },
    Mime {
        cost: usize,
        sell_rank: usize,
        edition: Edition,
    },
    Credit {
        cost: usize,
        sell_rank: usize,
        edition: Edition,
    },
    Dagger {
        cost: usize,
        sell_rank: usize,
        edition: Edition,
        mult: usize,
    },
    Banner {
        cost: usize,
        sell_rank: usize,
        edition: Edition,
    },
    Mystic {
        cost: usize,
        sell_rank: usize,
        edition: Edition,
    },
    Marble {
        cost: usize,
        sell_rank: usize,
        edition: Edition,
    },
    Loyalty {
        cost: usize,
        sell_rank: usize,
        edition: Edition,
        hands: u8,
    },
    Ball {
        cost: usize,
        sell_rank: usize,
        edition: Edition,
    },
    Misprint {
        cost: usize,
        sell_rank: usize,
        edition: Edition,
    },
    Dusk {
        cost: usize,
        sell_rank: usize,
        edition: Edition,
    },
    Fist {
        cost: usize,
        sell_rank: usize,
        edition: Edition,
        min: Option<Rank>,
    },
    Chaos {
        cost: usize,
        sell_rank: usize,
        edition: Edition,
    },
    Fibonacci {
        cost: usize,
        sell_rank: usize,
        edition: Edition,
    },
    Steel {
        cost: usize,
        sell_rank: usize,
        edition: Edition,
    },
    Scary {
        cost: usize,
        sell_rank: usize,
        edition: Edition,
    },
    Abstract {
        cost: usize,
        sell_rank: usize,
        edition: Edition,
    },
    Gratification {
        cost: usize,
        sell_rank: usize,
        edition: Edition,
    },
    Hack {
        cost: usize,
        sell_rank: usize,
        edition: Edition,
    },
    Pareidolia {
        cost: usize,
        sell_rank: usize,
        edition: Edition,
    },
    Michel {
        cost: usize,
        sell_rank: usize,
        edition: Edition,
    },
    Steven {
        cost: usize,
        sell_rank: usize,
        edition: Edition,
    },
    Todd {
        cost: usize,
        sell_rank: usize,
        edition: Edition,
    },
    Scholar {
        cost: usize,
        sell_rank: usize,
        edition: Edition,
    },
    Business {
        cost: usize,
        sell_rank: usize,
        edition: Edition,
    },
    Supernova {
        cost: usize,
        sell_rank: usize,
        edition: Edition,
    },
    Bus {
        cost: usize,
        sell_rank: usize,
        edition: Edition,
        mult: usize,
    },
    Space {
        cost: usize,
        sell_rank: usize,
        edition: Edition,
    },
    Egg {
        cost: usize,
        sell_rank: usize,
        edition: Edition,
    },
    Burglar {
        cost: usize,
        sell_rank: usize,
        edition: Edition,
    },
    Blackboard {
        cost: usize,
        sell_rank: usize,
        edition: Edition,
    },
    Runner {
        cost: usize,
        sell_rank: usize,
        edition: Edition,
        chips: usize,
    },
    Cream {
        cost: usize,
        sell_rank: usize,
        edition: usize,
        chips: usize,
    },
    DNA {
        cost: usize,
        sell_rank: usize,
        edition: Edition,
    },
    Splash {
        cost: usize,
        sell_rank: usize,
        edition: Edition,
    },
    Blue {
        cost: usize,
        sell_rank: usize,
        edition: Edition,
    },
    Sixth {
        cost: usize,
        sell_rank: usize,
        edition: Edition,
    },
    Constellation {
        cost: usize,
        sell_rank: usize,
        edition: Edition,
        mult: usize,
    },
    Hiker {
        cost: usize,
        sell_rank: usize,
        edition: Edition,
    },
    Faceless {
        cost: usize,
        sell_rank: usize,
        edition: Edition,
    },
    Green {
        cost: usize,
        sell_rank: usize,
        edition: Edition,
        mult: usize,
    },
    Superposition {
        cost: usize,
        sell_rank: usize,
        edition: Edition,
    },
    List {
        cost: usize,
        sell_rank: usize,
        edition: Edition,
        hand_type: HandType,
    },
    Cavendish {
        cost: usize,
        sell_rank: usize,
        edition: Edition,
    },
    Sharp {
        cost: usize,
        sell_rank: usize,
        edition: Edition,
    },
    Red {
        cost: usize,
        sell_rank: usize,
        edition: Edition,
        mult: usize,
    },
    Madness {
        cost: usize,
        sell_rank: usize,
        edition: Edition,
        mult: usize,
    },
    Square {
        cost: usize,
        sell_rank: usize,
        edition: Edition,
        chips: usize,
    },
    Seance {
        cost: usize,
        sell_rank: usize,
        edition: Edition,
    },
    Riff {
        cost: usize,
        sell_rank: usize,
        edition: Edition,
    },
    Vampire {
        cost: usize,
        sell_rank: usize,
        edition: Edition,
        mult: usize,
    },
    Shortcut {
        cost: usize,
        sell_rank: usize,
        edition: Edition,
    },
    Hologram {
        cost: usize,
        sell_rank: usize,
        edition: Edition,
        mult: usize,
    },
    Vagabond {
        cost: usize,
        sell_rank: usize,
        ediiton: Edition,
    },
    Baron {
        cost: usize,
        sell_rank: usize,
        edition: Edition,
    },
    Cloud {
        cost: usize,
        sell_rank: usize,
        edition: Edition,
    },
    Rocket {
        cost: usize,
        sell_rank: usize,
        edition: Edition,
        money: usize,
    },
    Obelisk {
        cost: usize,
        sell_rank: usize,
        edition: Edition,
        mult: usize,
    },
    Midas {
        cost: usize,
        sell_rank: usize,
        edition: Edition,
    },
    Luchador {
        cost: usize,
        sell_rank: usize,
        edition: Edition,
    },
    Photograph {
        cost: usize,
        sell_rank: usize,
        edition: Edition,
        used: bool,
    },
    Gift {
        cost: usize,
        sell_rank: usize,
        edition: Edition,
    },
    Turtle {
        cost: usize,
        sell_rank: usize,
        edition: Edition,
        hand_size: u8,
    },
    Erosion {
        cost: usize,
        sell_rank: usize,
        edition: Edition,
    },
    Parking {
        cost: usize,
        sell_rank: usize,
        edition: Edition,
    },
    Rebate {
        cost: usize,
        sell_rank: usize,
        edition: Edition,
        rank: Rank,
    },
    Moon {
        cost: usize,
        sell_rank: usize,
        edition: Edition,
    },
    Hallucination {
        cost: usize,
        sell_rank: usize,
        edition: Edition,
    },
    Fortune {
        cost: usize,
        sell_rank: usize,
        edition: Edition,
    },
    Juggler {
        cost: usize,
        sell_rank: usize,
        edition: Edition,
    },
    Drunkard {
        cost: usize,
        sell_rank: usize,
        edition: Edition,
    },
    Stone {
        cost: usize,
        sell_rank: usize,
        edition: Edition,
    },
    Golden {
        cost: usize,
        sell_rank: usize,
        edition: Edition,
    },
    Lucky {
        cost: usize,
        sell_rank: usize,
        edition: Edition,
        mult: usize,
    },
    Baseball {
        cost: usize,
        sell_rank: usize,
        edition: Edition,
    },
    Bull {
        cost: usize,
        sell_rank: usize,
        edition: Edition,
    },
    Cola {
        cost: usize,
        sell_rank: usize,
        edition: Edition,
    },
    Trading {
        cost: usize,
        sell_rank: usize,
        edition: Edition,
    },
    Flash {
        cost: usize,
        sell_rank: usize,
        edition: Edition,
        mult: usize,
    },
    Popcorn {
        cost: usize,
        sell_rank: usize,
        edition: Edition,
        mult: usize,
    },
    Trousers {
        cost: usize,
        sell_rank: usize,
        edition: Edition,
        mult: usize,
    },
    Ancient {
        cost: usize,
        sell_rank: usize,
        edition: Edition,
        suit: Suit,
    },
    Ramen {
        cost: usize,
        sell_rank: usize,
        edition: Edition,
        mult: usize,
    },
    Walkie {
        cost: usize,
        sell_rank: usize,
        edition: Edition,
    },
    Seltzer {
        cost: usize,
        sell_rank: usize,
        edition: Edition,
    },
    Castle {
        cost: usize,
        sell_rank: usize,
        edition: Edition,
        suit: Suit,
        chips: usize,
    },
    Smiley {
        cost: usize,
        sell_rank: usize,
        edition: Edition,
    },
    Campfire {
        cost: usize,
        sell_rank: usize,
        edition: Edition,
        mult: usize,
    },
    Ticket {
        cost: usize,
        sell_rank: usize,
        edition: Edition,
    },
    Bones {
        cost: usize,
        sell_rank: usize,
        edition: Edition,
    },
    Acrobat {
        cost: usize,
        sell_rank: usize,
        edition: Edition,
    },
    Sock {
        cost: usize,
        sell_rank: usize,
        edition: Edition,
    },
    Swashbuckler {
        cost: usize,
        sell_rank: usize,
        edition: Edition,
    },
    Troubadour {
        cost: usize,
        sell_rank: usize,
        edition: Edition,
    },
    Certificate {
        cost: usize,
        sell_rank: usize,
        edition: Edition,
    },
    Smeared {
        cost: usize,
        sell_rank: usize,
        edition: Edition,
    },
    Throwback {
        cost: usize,
        sell_rank: usize,
        edition: Edition,
        mult: usize,
    },
    Chad {
        cost: usize,
        sell_rank: usize,
        edition: Edition,
    },
    Gem {
        cost: usize,
        sell_rank: usize,
        edition: Edition,
    },
    Bloodstone {
        cost: usize,
        sell_rank: usize,
        edition: Edition,
    },
    Arrowhead {
        cost: usize,
        sell_rank: usize,
        edition: Edition,
    },
    Onyx {
        cost: usize,
        sell_rank: usize,
        edition: Edition,
    },
    Glass {
        cost: usize,
        sell_rank: usize,
        edition: Edition,
        mult: usize,
    },
    Showman {
        cost: usize,
        sell_rank: usize,
        edition: Edition,
    },
    Flower {
        cost: usize,
        sell_rank: usize,
        edition: Edition,
    },
    Blueprint {
        cost: usize,
        sell_rank: usize,
        edition: Edition,
    },
    Wee {
        cost: usize,
        sell_rank: usize,
        edition: Edition,
        chips: usize,
    },
    Andy {
        cost: usize,
        sell_rank: usize,
        edition: Edition,
    },
    Oops {
        cost: usize,
        sell_rank: usize,
        edition: Edition,
    },
    Idol {
        cost: usize,
        sell_rank: usize,
        edition: Edition,
        rank: Rank,
        suit: Suit,
    },
    Double {
        cost: usize,
        sell_rank: usize,
        edition: Edition,
    },
    Matador {
        cost: usize,
        sell_rank: usize,
        edition: Edition,
    },
    Road {
        cost: usize,
        sell_rank: usize,
        edition: Edition,
        mult: usize,
    },
    Duo {
        cost: usize,
        sell_rank: usize,
        edition: Edition,
    },
    Trio {
        cost: usize,
        sell_rank: usize,
        edition: Edition,
    },
    Family {
        cost: usize,
        sell_rank: usize,
        edition: Edition,
    },
    Order {
        cost: usize,
        sell_rank: usize,
        edition: Edition,
    },
    Tribe {
        cost: usize,
        sell_rank: usize,
        edition: Edition,
    },
    Stuntman {
        cost: usize,
        sell_rank: usize,
        edition: Edition,
    },
    Invisible {
        cost: usize,
        sell_rank: usize,
        edition: Edition,
        rounds: usize,
    },
    Brainstorm {
        cost: usize,
        sell_rank: usize,
        edition: Edition,
    },
    Satellite {
        cost: usize,
        sell_rank: usize,
        edition: Edition,
    },
    Shoot {
        cost: usize,
        sell_rank: usize,
        edition: Edition,
    },
    License {
        cost: usize,
        sell_rank: usize,
        edition: Edition,
    },
    Cartomancer {
        cost: usize,
        sell_rank: usize,
        edition: Edition,
    },
    Astronomer {
        cost: usize,
        sell_rank: usize,
        edition: Edition,
    },
    Burnt {
        cost: usize,
        sell_rank: usize,
        edition: Edition,
    },
    Bootstraps {
        cost: usize,
        sell_rank: usize,
        edition: Edition,
    },
    Canio {
        cost: usize,
        sell_rank: usize,
        edition: Edition,
        mult: usize,
    },
    Triboulet {
        cost: usize,
        sell_rank: usize,
        edition: Edition,
    },
    Yorick {
        cost: usize,
        sell_rank: usize,
        edition: Edition,
        discards: usize,
        mult: usize,
    },
    Chicot {
        cost: usize,
        sell_rank: usize,
        edition: Edition,
    },
    Perkeo {
        cost: usize,
        sell_rank: usize,
        edition: Edition,
    },
}

impl Display for Joker {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name())
    }
}

impl Joker {
    pub fn on_blind_select(&mut self, state: &mut State) {
        match self {
            Self::Dagger { mut mult, .. } => {
                if state.jokers.iter().position(|j| j == self).unwrap() + 1 < state.jokers.len() {
                    mult += state
                        .jokers
                        .remove(state.jokers.iter().position(|j| j == self).unwrap() + 1)
                        .get_sell_rank();
                }
            }
            Self::Marble { .. } => {
                todo!("randomness needed")
            }
            Self::Burglar { .. } => {
                state.hands_remaining += 3;
                state.discards_remaining = 0;
            }
            Self::Madness { mut mult, .. } => todo!("blinds"),
            Self::Riff { .. } => todo!("randomness"),
            Self::Certificate { .. } => todo!("randomness"),
            Self::Cartomancer { .. } => todo!("randomness"),
            _ => {}
        }
    }

    pub fn on_hand_played(&mut self, state: &mut State) {
        match self {
            Self::Fist { mut min, .. } => {
                min = Some(state.hand.iter().map(|c| c.rank).min().unwrap());
            }
            Self::Bus { mut mult, .. } => {
                if state
                    .scoring_cards
                    .iter()
                    .filter(|c| c.is_face_card(&state.jokers))
                    .count()
                    == 0
                {
                    mult += 1;
                } else {
                    mult = 0;
                }
            }
            Self::Space { .. } => todo!("randomness"),
            Self::Runner { mut chips, .. } => {
                if state.hand_type_played == HandType::Straight
                    || state.hand_type_played == HandType::StraightFlush
                {
                    chips += 15;
                }
            }
            Self::DNA { .. } => {
                if state.hands_remaining == state.hands - 1 && state.played_hand.len() == 1 {
                    state.deck.push(state.played_hand.get(0).unwrap().clone());
                    state.hand.push(state.played_hand.get(0).unwrap().clone());
                }
            }
            Self::Splash { .. } => {
                state.scoring_cards = state.played_hand.clone();
            }
            Self::Sixth { .. } => {
                if state.played_hand.len() == 1
                    && state.played_hand.get(0).unwrap().rank == Rank::Six
                {
                    state.deck.remove(
                        state
                            .deck
                            .iter()
                            .position(|c| c == state.played_hand.get(0).unwrap())
                            .unwrap(),
                    );
                }
            }
            Self::Green { mut mult, .. } => mult += 1,
            Self::List { hand_type, .. } => {
                if *hand_type == state.hand_type_played {
                    state.money += 4;
                }
            }
            Self::Square { mut chips, .. } => {
                if state.played_hand.len() == 4 {
                    chips += 4;
                }
            }
            Self::Vampire { mut mult, .. } => {
                state
                    .scoring_cards
                    .iter_mut()
                    .filter(|c| c.enhancement.is_some())
                    .for_each(|c| {
                        c.enhancement = None;
                        mult += 1;
                    });
            }
            Self::Obelisk { mut mult, .. } => {
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
                    .any(|k| *k == state.hand_type_played)
                {
                    mult = 5;
                } else {
                    mult += 1;
                }
            }
            Self::Midas { .. } => {
                state
                    .played_hand
                    .iter_mut()
                    .filter(|c| c.is_face_card(&state.jokers))
                    .for_each(|c| c.enhancement = Some(Enhancement::Gold));
            }
            Self::Photograph { mut used, .. } => used = false,
            Self::Trousers { mut mult, .. } => {
                let s: HashSet<Rank> =
                    HashSet::from_iter(state.selected_cards.iter().map(|c| c.rank));
                let mut result: u8 = 0;
                if s.len() < state.selected_cards.len() - 1 {
                    for v in s {
                        if state
                            .selected_cards
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
                    mult += 2;
                }
            }
            _ => {}
        }
    }

    pub fn on_card_score(&mut self, state: &mut State, card: &mut Card) {
        match self {
            Self::Greedy { .. } => {
                if card.suit == Suit::Diamond {
                    state.current_score.update(None, Some(3.0), None);
                }
            }
            Self::Lusty { .. } => {
                if card.suit == Suit::Heart {
                    state.current_score.update(None, Some(3.0), None);
                }
            }
            Self::Wrathful { .. } => {
                if card.suit == Suit::Spade {
                    state.current_score.update(None, Some(3.0), None);
                }
            }
            Self::Gluttonous { .. } => {
                if card.suit == Suit::Club {
                    state.current_score.update(None, Some(3.0), None);
                }
            }
            Self::Ball { .. } => {
                if card.rank == Rank::Eight {
                    todo!("randomness")
                }
            }
            Self::Dusk { .. } => {
                card.on_score(state);
            }
            Self::Fibonacci { .. } => {
                if card.rank == Rank::Ace
                    || card.rank == Rank::Two
                    || card.rank == Rank::Three
                    || card.rank == Rank::Five
                    || card.rank == Rank::Eight
                {
                    state.current_score.update(None, Some(8.0), None);
                }
            }
            Self::Scary { .. } => {
                state.current_score.update(Some(30), None, None);
            }
            Self::Hack { .. } => {
                if card.rank == Rank::Two
                    || card.rank == Rank::Three
                    || card.rank == Rank::Four
                    || card.rank == Rank::Five
                {
                    card.on_score(state);
                }
            }
            Self::Steven { .. } => {
                if card.rank == Rank::Two
                    || card.rank == Rank::Four
                    || card.rank == Rank::Six
                    || card.rank == Rank::Eight
                    || card.rank == Rank::Ten
                {
                    state.current_score.update(None, Some(4.0), None);
                }
            }
            Self::Todd { .. } => {
                if card.rank == Rank::Ace
                    || card.rank == Rank::Three
                    || card.rank == Rank::Five
                    || card.rank == Rank::Seven
                    || card.rank == Rank::Nine
                {
                    state.current_score.update(Some(31), None, None);
                }
            }
            Self::Scholar { .. } => {
                if card.rank == Rank::Ace {
                    state.current_score.update(Some(20), Some(4.0), None);
                }
            }
            Self::Business { .. } => {
                if card.is_face_card(&state.jokers) {
                    todo!("randomness");
                }
            }
            Self::Hiker { .. } => {
                card.chips += 5;
            }
            Self::Photograph { mut used, .. } => {
                if !used && card.is_face_card(&state.jokers) {
                    used = true;
                    state.current_score.update(None, None, Some(2.0));
                }
            }
            Self::Ancient { suit, .. } => {
                if card.suit == *suit {
                    state.current_score.update(None, None, Some(1.5));
                }
            }
            Self::Walkie { .. } => {
                if card.rank == Rank::Ten || card.rank == Rank::Four {
                    state.current_score.update(Some(10), Some(4.0), None);
                }
            }
            Self::Seltzer { .. } => card.on_score(state),
            Self::Smiley { .. } => {
                if card.is_face_card(&state.jokers) {
                    state.current_score.update(None, Some(5.0), None);
                }
            }
            Self::Ticket { .. } => state
                .scoring_cards
                .iter()
                .filter(|c| c.enhancement == Some(Enhancement::Gold))
                .for_each(|c| state.money += 4),
            Self::Sock { .. } => {
                if card.is_face_card(&state.jokers) {
                    card.on_score(state);
                }
            }
            Self::Chad { .. } => {
                card.on_score(state);
                card.on_score(state);
            }
            Self::Gem { .. } => {
                if card.is_suit(Suit::Diamond, state) {
                    state.money += 1;
                }
            }
            Self::Bloodstone { .. } => todo!("randomness"),
            Self::Arrowhead { .. } => {
                if card.is_suit(Suit::Spade, state) {
                    state.current_score.update(Some(50), None, None);
                }
            }
            Self::Onyx { .. } => {
                if card.is_suit(Suit::Club, state) {
                    state.current_score.update(None, Some(7.0), None);
                }
            }
            Self::Wee { mut chips, .. } => {
                if card.rank == Rank::Two {
                    chips += 8;
                }
            }
            Self::Idol { rank, suit, .. } => {
                if card.is_suit(*suit, state) && card.rank == *rank {
                    state.current_score.update(None, None, Some(2.0));
                }
            }
            Self::Triboulet { .. } => {
                if card.rank == Rank::King || card.rank == Rank::Queen {
                    state.current_score.update(None, None, Some(2.0));
                }
            }
            _ => {}
        }
    }

    pub fn on_card_in_hand_score(&mut self, state: &mut State, card: &Card) {
        match self {
            Self::Mime { .. } => card.on_score(state),
            Self::Fist { mut min, .. } => {
                if let Some(m) = min {
                    if m == card.rank {
                        state.current_score.update(
                            None,
                            Some((card.rank.get_rank() * 2) as f64),
                            None,
                        );
                    }
                    min = None;
                }
            }
            Self::Baron { .. } => {
                if card.rank == Rank::King {
                    state.current_score.update(None, None, Some(1.5));
                }
            }
            Self::Parking { .. } => todo!("randomness"),
            Self::Shoot { .. } => {
                if card.rank == Rank::Queen {
                    state.current_score.update(None, Some(13.0), None);
                }
            }
            _ => {}
        }
    }

    pub fn on_independent(&mut self, state: &mut State) {
        match self {
            Self::Joker { .. } => state.current_score.update(None, Some(4.0), None),
            Self::Jolly { .. } => {
                if HashSet::<Rank>::from_iter(state.selected_cards.iter().cloned().map(|c| c.rank))
                    .len()
                    < state.selected_cards.len()
                {
                    state.current_score.update(None, Some(8.0), None);
                }
            }
            Self::Zany { .. } => {
                let s: HashSet<Rank> =
                    HashSet::from_iter(state.selected_cards.iter().map(|c| c.rank));
                let mut result: bool = false;
                if s.len() < state.selected_cards.len() - 1 {
                    for v in s {
                        result |= state
                            .selected_cards
                            .iter()
                            .filter(|c| c.rank == v)
                            .collect::<Vec<&Card>>()
                            .len()
                            >= 3;
                    }
                }

                if result {
                    state.current_score.update(None, Some(12.0), None);
                }
            }
            Self::Mad { .. } => {
                let s: HashSet<Rank> =
                    HashSet::from_iter(state.selected_cards.iter().map(|c| c.rank));
                let mut result: u8 = 0;
                if s.len() < state.selected_cards.len() - 1 {
                    for v in s {
                        if state
                            .selected_cards
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
                    state.current_score.update(None, Some(10.0), None);
                }
            }
            Self::Crazy { .. } => {
                if state.hand_type_played == HandType::Straight
                    || state.hand_type_played == HandType::StraightFlush
                {
                    state.current_score.update(None, Some(12.0), None);
                }
            }
            Self::Droll { .. } => {
                let s: HashSet<Suit> =
                    HashSet::from_iter(state.selected_cards.iter().map(|c| c.suit));
                if s.len() == 1 {
                    state.current_score.update(None, Some(10.0), None);
                }
            }
            Self::Sly { .. } => {
                if HashSet::<Rank>::from_iter(state.selected_cards.iter().cloned().map(|c| c.rank))
                    .len()
                    < state.selected_cards.len()
                {
                    state.current_score.update(Some(50), None, None);
                }
            }
            Self::Wily { .. } => {
                let s: HashSet<Rank> =
                    HashSet::from_iter(state.selected_cards.iter().map(|c| c.rank));
                let mut result: bool = false;
                if s.len() < state.selected_cards.len() - 1 {
                    for v in s {
                        result |= state
                            .selected_cards
                            .iter()
                            .filter(|c| c.rank == v)
                            .collect::<Vec<&Card>>()
                            .len()
                            >= 3;
                    }
                }

                if result {
                    state.current_score.update(Some(100), None, None);
                }
            }
            Self::Clever { .. } => {
                let s: HashSet<Rank> =
                    HashSet::from_iter(state.selected_cards.iter().map(|c| c.rank));
                let mut result: u8 = 0;
                if s.len() < state.selected_cards.len() - 1 {
                    for v in s {
                        if state
                            .selected_cards
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
                    state.current_score.update(Some(80), None, None);
                }
            }
            Self::Devious { .. } => {
                if state.hand_type_played == HandType::Straight
                    || state.hand_type_played == HandType::StraightFlush
                {
                    state.current_score.update(Some(100), None, None);
                }
            }
            Self::Crafty { .. } => {
                let s: HashSet<Suit> =
                    HashSet::from_iter(state.selected_cards.iter().map(|c| c.suit));
                if s.len() == 1 {
                    state.current_score.update(Some(80), None, None);
                }
            }
            Self::Half { .. } => {
                if state.selected_cards.len() <= 3 {
                    state.current_score.update(None, Some(20.0), None);
                }
            }
            Self::Stencil { .. } => state.current_score.update(
                None,
                None,
                Some((state.jokers.len() - state.joker_slots + 1) as f64),
            ),
            Self::Dagger { mult, .. } => state.current_score.update(None, Some(*mult as f64), None),
            Self::Banner { .. } => {
                state
                    .current_score
                    .update(Some((state.discards_remaining * 30) as u64), None, None)
            }
            Self::Mystic { .. } => {
                if state.discards_remaining == 0 {
                    state.current_score.update(None, Some(15.0), None);
                }
            }
            Self::Loyalty { mut hands, .. } => {
                if hands == 0 {
                    state.current_score.update(None, None, Some(4.0));
                    hands = 6;
                }
                hands -= 1;
            }
            Self::Misprint { .. } => {
                todo!("randomness")
            }
            Self::Steel { .. } => state.current_score.update(
                None,
                None,
                Some(
                    0.2 * state
                        .deck
                        .iter()
                        .filter(|c| c.enhancement == Some(Enhancement::Steel))
                        .count() as f64,
                ),
            ),
            Self::Abstract { .. } => {
                state
                    .current_score
                    .update(None, Some(3.0 * state.jokers.len() as f64), None)
            }
            Self::Michel { .. } => state.current_score.update(None, Some(15.0), None),
            Self::Supernova { .. } => state.current_score.update(
                None,
                Some(
                    *state
                        .scoring
                        .scoring_times
                        .get(&state.hand_type_played)
                        .unwrap() as f64,
                ),
                None,
            ),
            Self::Bus { mult, .. } => state.current_score.update(None, Some(*mult as f64), None),
            Self::Blackboard { .. } => {
                if state
                    .hand
                    .iter()
                    .filter(|c| !(c.is_suit(Suit::Spade, state) || c.is_suit(Suit::Club, state)))
                    .count()
                    == 0
                {
                    state.current_score.update(None, None, Some(3.0));
                }
            }
            Self::Runner { chips, .. } => {
                state.current_score.update(Some(*chips as u64), None, None);
            }
            Self::Cream { mut chips, .. } => {
                state.current_score.update(Some(chips as u64), None, None);
                chips -= 5;
            }
            Self::Blue { .. } => state.current_score.update(
                Some((2 * state.remaining_deck.len()) as u64),
                None,
                None,
            ),
            Self::Constellation { mult, .. } => {
                state
                    .current_score
                    .update(None, None, Some((*mult / 10) as f64));
            }
            Self::Green { mult, .. } => state.current_score.update(None, Some(*mult as f64), None),
            Self::Superposition { .. } => {
                if (state.hand_type_played == HandType::Straight
                    || state.hand_type_played == HandType::StraightFlush)
                    && state
                        .played_hand
                        .iter()
                        .filter(|c| c.rank == Rank::Ace)
                        .count()
                        > 0
                    && state.consumables.len() < state.consumable_slots
                {
                    todo!("randomness")
                }
            }
            Self::Cavendish { .. } => state.current_score.update(None, None, Some(3.0)),
            Self::Sharp { .. } => {
                if state
                    .poker_hands_played
                    .iter()
                    .any(|t| *t == state.hand_type_played)
                {
                    state.current_score.update(None, None, Some(3.0));
                }
            }
            Self::Red { mult, .. } => state.current_score.update(None, Some(*mult as f64), None),
            Self::Madness { mult, .. } => {
                state.current_score.update(None, None, Some(*mult as f64));
            }
            Self::Square { chips, .. } => {
                state.current_score.update(Some(*chips as u64), None, None);
            }
            Self::Seance { .. } => {
                if state.hand_type_played == HandType::StraightFlush
                    && state.consumables.len() < state.consumable_slots
                {
                    todo!("randomness")
                }
            }
            Self::Vampire { mult, .. } => {
                state.current_score.update(None, None, Some(*mult as f64));
            }
            Self::Hologram { mult, .. } => {
                state
                    .current_score
                    .update(None, None, Some((*mult / 4) as f64));
            }
            Self::Vagabond { .. } => {
                if state.money < 5 {
                    todo!("randomness")
                }
            }
            Self::Obelisk { mult, .. } => {
                state.current_score.update(None, None, Some(*mult as f64));
            }
            Self::Erosion { .. } => {
                state
                    .current_score
                    .update(None, Some(max(52 - state.deck.len(), 0) as f64), None);
            }
            Self::Fortune { .. } => {
                state
                    .current_score
                    .update(None, Some(state.tarots_used as f64), None)
            }
            Self::Stone { .. } => state.current_score.update(
                Some(
                    (state
                        .deck
                        .iter()
                        .filter(|c| c.enhancement == Some(Enhancement::Stone))
                        .count()
                        * 25) as u64,
                ),
                None,
                None,
            ),
            Self::Lucky { mult, .. } => {
                state
                    .current_score
                    .update(None, None, Some((*mult / 4) as f64))
            }
            Self::Bull { .. } => {
                state
                    .current_score
                    .update(Some((2 * state.money) as u64), None, None)
            }
            Self::Flash { mult, .. } => state.current_score.update(None, Some(*mult as f64), None),
            Self::Popcorn { mult, .. } => {
                state.current_score.update(None, Some(*mult as f64), None);
            }
            Self::Trousers { mult, .. } => {
                state.current_score.update(None, Some(*mult as f64), None);
            }
            Self::Ramen { mult, .. } => {
                state
                    .current_score
                    .update(None, None, Some((*mult / 100) as f64));
            }
            Self::Castle { chips, .. } => {
                state.current_score.update(Some(*chips as u64), None, None);
            }
            Self::Campfire { mult, .. } => {
                state
                    .current_score
                    .update(None, None, Some((*mult / 4) as f64))
            }
            Self::Acrobat { .. } => {
                if state.hands_remaining == state.hands - 1 {
                    state.current_score.update(None, None, Some(3.0));
                }
            }
            Self::Swashbuckler { .. } => state.current_score.update(
                None,
                Some(
                    (state
                        .jokers
                        .iter()
                        .map(|j| j.get_sell_rank())
                        .sum::<usize>()
                        - self.get_sell_rank()) as f64,
                ),
                None,
            ),
            Self::Throwback { mult, .. } => {
                state
                    .current_score
                    .update(None, None, Some((*mult / 4) as f64))
            }
            Self::Glass { mult, .. } => {
                state
                    .current_score
                    .update(None, None, Some((*mult / 4) as f64))
            }
            Self::Flower { .. } => {
                if state
                    .scoring_cards
                    .iter()
                    .any(|c| c.is_suit(Suit::Spade, state))
                    && state
                        .scoring_cards
                        .iter()
                        .any(|c| c.is_suit(Suit::Heart, state))
                    && state
                        .scoring_cards
                        .iter()
                        .any(|c| c.is_suit(Suit::Club, state))
                    && state
                        .scoring_cards
                        .iter()
                        .any(|c| c.is_suit(Suit::Diamond, state))
                {
                    state.current_score.update(None, None, Some(3.0));
                }
            }
            Self::Wee { chips, .. } => state.current_score.update(Some(*chips as u64), None, None),
            Self::Double { .. } => {
                if state
                    .scoring_cards
                    .iter()
                    .any(|c| c.is_suit(Suit::Club, state))
                    && state
                        .scoring_cards
                        .iter()
                        .any(|c| !c.is_suit(Suit::Club, state))
                {
                    state.current_score.update(None, None, Some(2.0));
                }
            }
            Self::Road { mult, .. } => {
                state
                    .current_score
                    .update(None, None, Some((*mult / 2) as f64))
            }
            Self::Duo { .. } => {
                if HashSet::<Rank>::from_iter(state.scoring_cards.iter().map(|c| c.rank)).len()
                    < state.scoring_cards.len()
                {
                    state.current_score.update(None, None, Some(2.0));
                }
            }
            Self::Trio { .. } => {
                let s: HashSet<Rank> =
                    HashSet::from_iter(state.scoring_cards.iter().map(|c| c.rank));
                if s.len() < state.scoring_cards.len() - 1
                    && state
                        .scoring_cards
                        .iter()
                        .filter(|c| c.rank == *s.iter().max().unwrap())
                        .count()
                        >= 3
                {
                    state.current_score.update(None, None, Some(3.0));
                }
            }
            Self::Family { .. } => {
                let s: HashSet<Rank> =
                    HashSet::from_iter(state.scoring_cards.iter().map(|c| c.rank));
                if s.len() < state.scoring_cards.len() - 2 {
                    state.current_score.update(None, None, Some(4.0));
                }
            }
            Self::Order { .. } => {
                if state.hand_type_played == HandType::Straight
                    || state.hand_type_played == HandType::StraightFlush
                {
                    state.current_score.update(None, None, Some(3.0));
                }
            }
            Self::Tribe { .. } => {
                if state.hand_type_played == HandType::Flush
                    || state.hand_type_played == HandType::FlushHouse
                    || state.hand_type_played == HandType::FlushFive
                {
                    state.current_score.update(None, None, Some(2.0));
                }
            }
            Self::Stuntman { .. } => state.current_score.update(Some(100), None, None),
            Self::License { .. } => {
                if state
                    .deck
                    .iter()
                    .filter(|c| c.enhancement.is_some())
                    .count()
                    >= 16
                {
                    state.current_score.update(None, None, Some(3.0));
                }
            }
            Self::Bootstraps { .. } => {
                state
                    .current_score
                    .update(None, Some(((state.money / 5) * 2) as f64), None);
            }
            Self::Canio { mult, .. } => state.current_score.update(None, None, Some(*mult as f64)),
            Self::Yorick { mult, .. } => state.current_score.update(None, None, Some(*mult as f64)),
            _ => {}
        }
    }

    pub fn on_discard(&mut self, state: &mut State) {
        match self {
            Self::Faceless { .. } => {
                if state
                    .played_hand
                    .iter()
                    .filter(|c| c.is_face_card(&state.jokers))
                    .count()
                    >= 3
                {
                    state.money += 5;
                }
            }
            Self::Green { mut mult, .. } => {
                if mult > 0 {
                    mult -= 1;
                }
            }
            Self::Rebate { rank, .. } => state
                .selected_cards
                .iter()
                .filter(|c| c.rank == *rank)
                .for_each(|_| state.money += 5),
            Self::Trading { .. } => {
                if state.discards_remaining == state.discards - 1 && state.selected_cards.len() == 1
                {
                    state.deck.remove(
                        state
                            .deck
                            .iter()
                            .position(|c| c == state.selected_cards.get(0).unwrap())
                            .unwrap(),
                    );
                    state.money += 3;
                }
            }
            Self::Ramen { mut mult, .. } => {
                mult -= state.selected_cards.len();
                if mult <= 100 {
                    state
                        .jokers
                        .remove(state.jokers.iter().position(|j| j == self).unwrap());
                }
            }
            Self::Castle {
                suit, mut chips, ..
            } => {
                state
                    .selected_cards
                    .iter()
                    .filter(|c| c.is_suit(*suit, state))
                    .for_each(|_| chips += 3);
            }
            Self::Road { mut mult, .. } => mult += 1,
            Self::Yorick {
                mut discards,
                mut mult,
                ..
            } => {
                discards -= state.selected_cards.len();
                if discards <= 0 {
                    discards += 23;
                    mult += 1;
                }
            }
            _ => {}
        }
    }

    pub fn on_round_end(&mut self, state: &mut State) {
        match self {
            Self::Gratification { .. } => {
                if state.discards_remaining == state.discards {
                    state.money += 2 * state.discards;
                }
            }
            Self::Michel { .. } => todo!("randomness"),
            Self::Egg { mut sell_rank, .. } => sell_rank += 3,
            Self::List { hand_type, .. } => todo!("randomness"),
            Self::Cavendish { .. } => todo!("randomness"),
            Self::Cloud { .. } => {
                state.money += state.deck.iter().filter(|c| c.rank == Rank::Nine).count();
            }
            Self::Rocket { money, .. } => {
                state.money += *money;
                todo!("blinds")
            }
            Self::Gift { .. } => {
                state.jokers.iter_mut().for_each(|j| j.increase_sell_rank());
            }
            Self::Turtle { mut hand_size, .. } => {
                hand_size -= 1;
                if hand_size == 0 {
                    state
                        .jokers
                        .remove(state.jokers.iter().position(|j| j == self).unwrap());
                }
            }
            Self::Rebate { mut rank, .. } => todo!("randomness"),
            Self::Golden { .. } => state.money += 4,
            Self::Popcorn { mut mult, .. } => {
                mult -= 4;
                if mult == 0 {
                    state
                        .jokers
                        .remove(state.jokers.iter().position(|j| j == self).unwrap());
                }
            }
            Self::Ancient { mut suit, .. } => todo!("randomness"),
            Self::Castle { mut suit, .. } => todo!("randomness"),
            Self::Campfire { mut mult, .. } => todo!("blinds"),
            Self::Idol {
                mut rank, mut suit, ..
            } => todo!("randomness"),
            Self::Road { mut mult, .. } => mult = 2,
            Self::Invisible { mut rounds, .. } => rounds += 1,
            Self::Satellite { .. } => state.money += state.unique_planets_used,
            _ => {}
        }
    }

    pub fn on_shop_close(&mut self, state: &mut State) {}

    pub fn on_shop_reroll(&mut self, state: &mut State) {
        match self {
            Self::Flash { mut mult, .. } => mult += 2,
            _ => {}
        }
    }

    pub fn on_pack_open(&mut self, state: &mut State) {
        match self {
            Self::Hallucination { .. } => todo!("randomness"),
            _ => {}
        }
    }

    pub fn on_pack_skip(&mut self, state: &State) {
        match self {
            Self::Red { mut mult, .. } => mult += 3,
            _ => {}
        }
    }

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

    pub fn on_sell(self, state: &mut State) {
        match self {
            Self::Luchador { .. } => todo!("blinds"),
            Self::Juggler { .. } => state.hand_size -= 1,
            Self::Drunkard { .. } => {
                state.discards -= 1;
                state.discards_remaining -= 1;
            }
            Self::Cola { .. } => todo!("tags"),
            Self::Troubadour { .. } => {
                state.hand_size -= 2;
                state.hands += 1;
                state.hands_remaining += 1;
            }
            Self::Andy { .. } => {
                state.discards -= 3;
                state.discards_remaining -= 3;
                state.hand_size -= 1;
            }
            Self::Stuntman { .. } => state.hand_size += 2,
            _ => {}
        }
    }

    pub fn on_planet_use(&mut self) {
        match self {
            Self::Constellation { mut mult, .. } => {
                mult += 1;
            }
            _ => {}
        }
    }

    pub fn on_card_add(&mut self) {
        match self {
            Self::Hologram { mut mult, .. } => mult += 1,
            _ => {}
        }
    }

    pub fn on_card_sell(&mut self) {
        match self {
            Self::Campfire { mut mult, .. } => mult += 1,
            _ => {}
        }
    }

    pub fn on_blind_skip(&mut self) {
        match self {
            Self::Throwback { mut mult, .. } => mult += 1,
            _ => {}
        }
    }

    pub fn on_card_destroy(&mut self, card: &Card) {
        match self {
            Self::Glass { mut mult, .. } => {
                if card.enhancement == Some(Enhancement::Glass) {
                    mult += 3;
                }
            }
            _ => {}
        }
    }

    pub fn get_cost(&self) -> usize {
        match self {
            Self::Joker { cost, .. }
            | Self::Greedy { cost, .. }
            | Self::Lusty { cost, .. }
            | Self::Wrathful { cost, .. }
            | Self::Gluttonous { cost, .. }
            | Self::Jolly { cost, .. }
            | Self::Zany { cost, .. }
            | Self::Mad { cost, .. }
            | Self::Crazy { cost, .. }
            | Self::Droll { cost, .. }
            | Self::Sly { cost, .. }
            | Self::Wily { cost, .. }
            | Self::Clever { cost, .. }
            | Self::Devious { cost, .. }
            | Self::Crafty { cost, .. }
            | Self::Half { cost, .. }
            | Self::Stencil { cost, .. }
            | Self::Fingers { cost, .. }
            | Self::Mime { cost, .. }
            | Self::Credit { cost, .. }
            | Self::Dagger { cost, .. }
            | Self::Banner { cost, .. }
            | Self::Mystic { cost, .. }
            | Self::Marble { cost, .. }
            | Self::Loyalty { cost, .. }
            | Self::Ball { cost, .. }
            | Self::Misprint { cost, .. }
            | Self::Dusk { cost, .. }
            | Self::Fist { cost, .. }
            | Self::Chaos { cost, .. }
            | Self::Fibonacci { cost, .. }
            | Self::Steel { cost, .. }
            | Self::Scary { cost, .. }
            | Self::Abstract { cost, .. }
            | Self::Gratification { cost, .. }
            | Self::Hack { cost, .. }
            | Self::Pareidolia { cost, .. }
            | Self::Michel { cost, .. }
            | Self::Steven { cost, .. }
            | Self::Todd { cost, .. }
            | Self::Scholar { cost, .. }
            | Self::Business { cost, .. }
            | Self::Supernova { cost, .. }
            | Self::Bus { cost, .. }
            | Self::Space { cost, .. }
            | Self::Egg { cost, .. }
            | Self::Burglar { cost, .. }
            | Self::Blackboard { cost, .. }
            | Self::Runner { cost, .. }
            | Self::Cream { cost, .. }
            | Self::DNA { cost, .. }
            | Self::Splash { cost, .. }
            | Self::Blue { cost, .. }
            | Self::Sixth { cost, .. }
            | Self::Constellation { cost, .. }
            | Self::Hiker { cost, .. }
            | Self::Faceless { cost, .. }
            | Self::Green { cost, .. }
            | Self::Superposition { cost, .. }
            | Self::List { cost, .. }
            | Self::Cavendish { cost, .. }
            | Self::Sharp { cost, .. }
            | Self::Red { cost, .. }
            | Self::Madness { cost, .. }
            | Self::Square { cost, .. }
            | Self::Seance { cost, .. }
            | Self::Riff { cost, .. }
            | Self::Vampire { cost, .. }
            | Self::Shortcut { cost, .. }
            | Self::Hologram { cost, .. }
            | Self::Vagabond { cost, .. }
            | Self::Baron { cost, .. }
            | Self::Cloud { cost, .. }
            | Self::Rocket { cost, .. }
            | Self::Obelisk { cost, .. }
            | Self::Midas { cost, .. }
            | Self::Luchador { cost, .. }
            | Self::Photograph { cost, .. }
            | Self::Gift { cost, .. }
            | Self::Turtle { cost, .. }
            | Self::Erosion { cost, .. }
            | Self::Parking { cost, .. }
            | Self::Rebate { cost, .. }
            | Self::Moon { cost, .. }
            | Self::Hallucination { cost, .. }
            | Self::Fortune { cost, .. }
            | Self::Juggler { cost, .. }
            | Self::Drunkard { cost, .. }
            | Self::Stone { cost, .. }
            | Self::Golden { cost, .. }
            | Self::Lucky { cost, .. }
            | Self::Baseball { cost, .. }
            | Self::Bull { cost, .. }
            | Self::Cola { cost, .. }
            | Self::Trading { cost, .. }
            | Self::Flash { cost, .. }
            | Self::Popcorn { cost, .. }
            | Self::Trousers { cost, .. }
            | Self::Ancient { cost, .. }
            | Self::Ramen { cost, .. }
            | Self::Walkie { cost, .. }
            | Self::Seltzer { cost, .. }
            | Self::Castle { cost, .. }
            | Self::Smiley { cost, .. }
            | Self::Campfire { cost, .. }
            | Self::Ticket { cost, .. }
            | Self::Bones { cost, .. }
            | Self::Acrobat { cost, .. }
            | Self::Sock { cost, .. }
            | Self::Swashbuckler { cost, .. }
            | Self::Troubadour { cost, .. }
            | Self::Certificate { cost, .. }
            | Self::Smeared { cost, .. }
            | Self::Throwback { cost, .. }
            | Self::Chad { cost, .. }
            | Self::Gem { cost, .. }
            | Self::Bloodstone { cost, .. }
            | Self::Arrowhead { cost, .. }
            | Self::Onyx { cost, .. }
            | Self::Glass { cost, .. }
            | Self::Showman { cost, .. }
            | Self::Flower { cost, .. }
            | Self::Blueprint { cost, .. }
            | Self::Wee { cost, .. }
            | Self::Andy { cost, .. }
            | Self::Oops { cost, .. }
            | Self::Idol { cost, .. }
            | Self::Double { cost, .. }
            | Self::Matador { cost, .. }
            | Self::Road { cost, .. }
            | Self::Duo { cost, .. }
            | Self::Trio { cost, .. }
            | Self::Family { cost, .. }
            | Self::Order { cost, .. }
            | Self::Tribe { cost, .. }
            | Self::Stuntman { cost, .. }
            | Self::Invisible { cost, .. }
            | Self::Brainstorm { cost, .. }
            | Self::Satellite { cost, .. }
            | Self::Shoot { cost, .. }
            | Self::License { cost, .. }
            | Self::Cartomancer { cost, .. }
            | Self::Astronomer { cost, .. }
            | Self::Burnt { cost, .. }
            | Self::Bootstraps { cost, .. }
            | Self::Canio { cost, .. }
            | Self::Triboulet { cost, .. }
            | Self::Yorick { cost, .. }
            | Self::Chicot { cost, .. }
            | Self::Perkeo { cost, .. } => *cost,
        }
    }

    pub fn get_sell_rank(&self) -> usize {
        match self {
            Self::Joker { sell_rank, .. }
            | Self::Greedy { sell_rank, .. }
            | Self::Lusty { sell_rank, .. }
            | Self::Wrathful { sell_rank, .. }
            | Self::Gluttonous { sell_rank, .. }
            | Self::Jolly { sell_rank, .. }
            | Self::Zany { sell_rank, .. }
            | Self::Mad { sell_rank, .. }
            | Self::Crazy { sell_rank, .. }
            | Self::Droll { sell_rank, .. }
            | Self::Sly { sell_rank, .. }
            | Self::Wily { sell_rank, .. }
            | Self::Clever { sell_rank, .. }
            | Self::Devious { sell_rank, .. }
            | Self::Crafty { sell_rank, .. }
            | Self::Half { sell_rank, .. }
            | Self::Stencil { sell_rank, .. }
            | Self::Fingers { sell_rank, .. }
            | Self::Mime { sell_rank, .. }
            | Self::Credit { sell_rank, .. }
            | Self::Dagger { sell_rank, .. }
            | Self::Banner { sell_rank, .. }
            | Self::Mystic { sell_rank, .. }
            | Self::Marble { sell_rank, .. }
            | Self::Loyalty { sell_rank, .. }
            | Self::Ball { sell_rank, .. }
            | Self::Misprint { sell_rank, .. }
            | Self::Dusk { sell_rank, .. }
            | Self::Fist { sell_rank, .. }
            | Self::Chaos { sell_rank, .. }
            | Self::Fibonacci { sell_rank, .. }
            | Self::Steel { sell_rank, .. }
            | Self::Scary { sell_rank, .. }
            | Self::Abstract { sell_rank, .. }
            | Self::Gratification { sell_rank, .. }
            | Self::Hack { sell_rank, .. }
            | Self::Pareidolia { sell_rank, .. }
            | Self::Michel { sell_rank, .. }
            | Self::Steven { sell_rank, .. }
            | Self::Todd { sell_rank, .. }
            | Self::Scholar { sell_rank, .. }
            | Self::Business { sell_rank, .. }
            | Self::Supernova { sell_rank, .. }
            | Self::Bus { sell_rank, .. }
            | Self::Space { sell_rank, .. }
            | Self::Egg { sell_rank, .. }
            | Self::Burglar { sell_rank, .. }
            | Self::Blackboard { sell_rank, .. }
            | Self::Runner { sell_rank, .. }
            | Self::Cream { sell_rank, .. }
            | Self::DNA { sell_rank, .. }
            | Self::Splash { sell_rank, .. }
            | Self::Blue { sell_rank, .. }
            | Self::Sixth { sell_rank, .. }
            | Self::Constellation { sell_rank, .. }
            | Self::Hiker { sell_rank, .. }
            | Self::Faceless { sell_rank, .. }
            | Self::Green { sell_rank, .. }
            | Self::Superposition { sell_rank, .. }
            | Self::List { sell_rank, .. }
            | Self::Cavendish { sell_rank, .. }
            | Self::Sharp { sell_rank, .. }
            | Self::Red { sell_rank, .. }
            | Self::Madness { sell_rank, .. }
            | Self::Square { sell_rank, .. }
            | Self::Seance { sell_rank, .. }
            | Self::Riff { sell_rank, .. }
            | Self::Vampire { sell_rank, .. }
            | Self::Shortcut { sell_rank, .. }
            | Self::Hologram { sell_rank, .. }
            | Self::Vagabond { sell_rank, .. }
            | Self::Baron { sell_rank, .. }
            | Self::Cloud { sell_rank, .. }
            | Self::Rocket { sell_rank, .. }
            | Self::Obelisk { sell_rank, .. }
            | Self::Midas { sell_rank, .. }
            | Self::Luchador { sell_rank, .. }
            | Self::Photograph { sell_rank, .. }
            | Self::Gift { sell_rank, .. }
            | Self::Turtle { sell_rank, .. }
            | Self::Erosion { sell_rank, .. }
            | Self::Parking { sell_rank, .. }
            | Self::Rebate { sell_rank, .. }
            | Self::Moon { sell_rank, .. }
            | Self::Hallucination { sell_rank, .. }
            | Self::Fortune { sell_rank, .. }
            | Self::Juggler { sell_rank, .. }
            | Self::Drunkard { sell_rank, .. }
            | Self::Stone { sell_rank, .. }
            | Self::Golden { sell_rank, .. }
            | Self::Lucky { sell_rank, .. }
            | Self::Baseball { sell_rank, .. }
            | Self::Bull { sell_rank, .. }
            | Self::Cola { sell_rank, .. }
            | Self::Trading { sell_rank, .. }
            | Self::Flash { sell_rank, .. }
            | Self::Popcorn { sell_rank, .. }
            | Self::Trousers { sell_rank, .. }
            | Self::Ancient { sell_rank, .. }
            | Self::Ramen { sell_rank, .. }
            | Self::Walkie { sell_rank, .. }
            | Self::Seltzer { sell_rank, .. }
            | Self::Castle { sell_rank, .. }
            | Self::Smiley { sell_rank, .. }
            | Self::Campfire { sell_rank, .. }
            | Self::Ticket { sell_rank, .. }
            | Self::Bones { sell_rank, .. }
            | Self::Acrobat { sell_rank, .. }
            | Self::Sock { sell_rank, .. }
            | Self::Swashbuckler { sell_rank, .. }
            | Self::Troubadour { sell_rank, .. }
            | Self::Certificate { sell_rank, .. }
            | Self::Smeared { sell_rank, .. }
            | Self::Throwback { sell_rank, .. }
            | Self::Chad { sell_rank, .. }
            | Self::Gem { sell_rank, .. }
            | Self::Bloodstone { sell_rank, .. }
            | Self::Arrowhead { sell_rank, .. }
            | Self::Onyx { sell_rank, .. }
            | Self::Glass { sell_rank, .. }
            | Self::Showman { sell_rank, .. }
            | Self::Flower { sell_rank, .. }
            | Self::Blueprint { sell_rank, .. }
            | Self::Wee { sell_rank, .. }
            | Self::Andy { sell_rank, .. }
            | Self::Oops { sell_rank, .. }
            | Self::Idol { sell_rank, .. }
            | Self::Double { sell_rank, .. }
            | Self::Matador { sell_rank, .. }
            | Self::Road { sell_rank, .. }
            | Self::Duo { sell_rank, .. }
            | Self::Trio { sell_rank, .. }
            | Self::Family { sell_rank, .. }
            | Self::Order { sell_rank, .. }
            | Self::Tribe { sell_rank, .. }
            | Self::Stuntman { sell_rank, .. }
            | Self::Invisible { sell_rank, .. }
            | Self::Brainstorm { sell_rank, .. }
            | Self::Satellite { sell_rank, .. }
            | Self::Shoot { sell_rank, .. }
            | Self::License { sell_rank, .. }
            | Self::Cartomancer { sell_rank, .. }
            | Self::Astronomer { sell_rank, .. }
            | Self::Burnt { sell_rank, .. }
            | Self::Bootstraps { sell_rank, .. }
            | Self::Canio { sell_rank, .. }
            | Self::Triboulet { sell_rank, .. }
            | Self::Yorick { sell_rank, .. }
            | Self::Chicot { sell_rank, .. }
            | Self::Perkeo { sell_rank, .. } => *sell_rank,
        }
    }

    pub fn increase_sell_rank(&mut self) {
        match self {
            Self::Joker { mut sell_rank, .. }
            | Self::Greedy { mut sell_rank, .. }
            | Self::Lusty { mut sell_rank, .. }
            | Self::Wrathful { mut sell_rank, .. }
            | Self::Gluttonous { mut sell_rank, .. }
            | Self::Jolly { mut sell_rank, .. }
            | Self::Zany { mut sell_rank, .. }
            | Self::Mad { mut sell_rank, .. }
            | Self::Crazy { mut sell_rank, .. }
            | Self::Droll { mut sell_rank, .. }
            | Self::Sly { mut sell_rank, .. }
            | Self::Wily { mut sell_rank, .. }
            | Self::Clever { mut sell_rank, .. }
            | Self::Devious { mut sell_rank, .. }
            | Self::Crafty { mut sell_rank, .. }
            | Self::Half { mut sell_rank, .. }
            | Self::Stencil { mut sell_rank, .. }
            | Self::Fingers { mut sell_rank, .. }
            | Self::Mime { mut sell_rank, .. }
            | Self::Credit { mut sell_rank, .. }
            | Self::Dagger { mut sell_rank, .. }
            | Self::Banner { mut sell_rank, .. }
            | Self::Mystic { mut sell_rank, .. }
            | Self::Marble { mut sell_rank, .. }
            | Self::Loyalty { mut sell_rank, .. }
            | Self::Ball { mut sell_rank, .. }
            | Self::Misprint { mut sell_rank, .. }
            | Self::Dusk { mut sell_rank, .. }
            | Self::Fist { mut sell_rank, .. }
            | Self::Chaos { mut sell_rank, .. }
            | Self::Fibonacci { mut sell_rank, .. }
            | Self::Steel { mut sell_rank, .. }
            | Self::Scary { mut sell_rank, .. }
            | Self::Abstract { mut sell_rank, .. }
            | Self::Gratification { mut sell_rank, .. }
            | Self::Hack { mut sell_rank, .. }
            | Self::Pareidolia { mut sell_rank, .. }
            | Self::Michel { mut sell_rank, .. }
            | Self::Steven { mut sell_rank, .. }
            | Self::Todd { mut sell_rank, .. }
            | Self::Scholar { mut sell_rank, .. }
            | Self::Business { mut sell_rank, .. }
            | Self::Supernova { mut sell_rank, .. }
            | Self::Bus { mut sell_rank, .. }
            | Self::Space { mut sell_rank, .. }
            | Self::Egg { mut sell_rank, .. }
            | Self::Burglar { mut sell_rank, .. }
            | Self::Blackboard { mut sell_rank, .. }
            | Self::Runner { mut sell_rank, .. }
            | Self::Cream { mut sell_rank, .. }
            | Self::DNA { mut sell_rank, .. }
            | Self::Splash { mut sell_rank, .. }
            | Self::Blue { mut sell_rank, .. }
            | Self::Sixth { mut sell_rank, .. }
            | Self::Constellation { mut sell_rank, .. }
            | Self::Hiker { mut sell_rank, .. }
            | Self::Faceless { mut sell_rank, .. }
            | Self::Green { mut sell_rank, .. }
            | Self::Superposition { mut sell_rank, .. }
            | Self::List { mut sell_rank, .. }
            | Self::Cavendish { mut sell_rank, .. }
            | Self::Sharp { mut sell_rank, .. }
            | Self::Red { mut sell_rank, .. }
            | Self::Madness { mut sell_rank, .. }
            | Self::Square { mut sell_rank, .. }
            | Self::Seance { mut sell_rank, .. }
            | Self::Riff { mut sell_rank, .. }
            | Self::Vampire { mut sell_rank, .. }
            | Self::Shortcut { mut sell_rank, .. }
            | Self::Hologram { mut sell_rank, .. }
            | Self::Vagabond { mut sell_rank, .. }
            | Self::Baron { mut sell_rank, .. }
            | Self::Cloud { mut sell_rank, .. }
            | Self::Rocket { mut sell_rank, .. }
            | Self::Obelisk { mut sell_rank, .. }
            | Self::Midas { mut sell_rank, .. }
            | Self::Luchador { mut sell_rank, .. }
            | Self::Photograph { mut sell_rank, .. }
            | Self::Gift { mut sell_rank, .. }
            | Self::Turtle { mut sell_rank, .. }
            | Self::Erosion { mut sell_rank, .. }
            | Self::Parking { mut sell_rank, .. }
            | Self::Rebate { mut sell_rank, .. }
            | Self::Moon { mut sell_rank, .. }
            | Self::Hallucination { mut sell_rank, .. }
            | Self::Fortune { mut sell_rank, .. }
            | Self::Juggler { mut sell_rank, .. }
            | Self::Drunkard { mut sell_rank, .. }
            | Self::Stone { mut sell_rank, .. }
            | Self::Golden { mut sell_rank, .. }
            | Self::Lucky { mut sell_rank, .. }
            | Self::Baseball { mut sell_rank, .. }
            | Self::Bull { mut sell_rank, .. }
            | Self::Cola { mut sell_rank, .. }
            | Self::Trading { mut sell_rank, .. }
            | Self::Flash { mut sell_rank, .. }
            | Self::Popcorn { mut sell_rank, .. }
            | Self::Trousers { mut sell_rank, .. }
            | Self::Ancient { mut sell_rank, .. }
            | Self::Ramen { mut sell_rank, .. }
            | Self::Walkie { mut sell_rank, .. }
            | Self::Seltzer { mut sell_rank, .. }
            | Self::Castle { mut sell_rank, .. }
            | Self::Smiley { mut sell_rank, .. }
            | Self::Campfire { mut sell_rank, .. }
            | Self::Ticket { mut sell_rank, .. }
            | Self::Bones { mut sell_rank, .. }
            | Self::Acrobat { mut sell_rank, .. }
            | Self::Sock { mut sell_rank, .. }
            | Self::Swashbuckler { mut sell_rank, .. }
            | Self::Troubadour { mut sell_rank, .. }
            | Self::Certificate { mut sell_rank, .. }
            | Self::Smeared { mut sell_rank, .. }
            | Self::Throwback { mut sell_rank, .. }
            | Self::Chad { mut sell_rank, .. }
            | Self::Gem { mut sell_rank, .. }
            | Self::Bloodstone { mut sell_rank, .. }
            | Self::Arrowhead { mut sell_rank, .. }
            | Self::Onyx { mut sell_rank, .. }
            | Self::Glass { mut sell_rank, .. }
            | Self::Showman { mut sell_rank, .. }
            | Self::Flower { mut sell_rank, .. }
            | Self::Blueprint { mut sell_rank, .. }
            | Self::Wee { mut sell_rank, .. }
            | Self::Andy { mut sell_rank, .. }
            | Self::Oops { mut sell_rank, .. }
            | Self::Idol { mut sell_rank, .. }
            | Self::Double { mut sell_rank, .. }
            | Self::Matador { mut sell_rank, .. }
            | Self::Road { mut sell_rank, .. }
            | Self::Duo { mut sell_rank, .. }
            | Self::Trio { mut sell_rank, .. }
            | Self::Family { mut sell_rank, .. }
            | Self::Order { mut sell_rank, .. }
            | Self::Tribe { mut sell_rank, .. }
            | Self::Stuntman { mut sell_rank, .. }
            | Self::Invisible { mut sell_rank, .. }
            | Self::Brainstorm { mut sell_rank, .. }
            | Self::Satellite { mut sell_rank, .. }
            | Self::Shoot { mut sell_rank, .. }
            | Self::License { mut sell_rank, .. }
            | Self::Cartomancer { mut sell_rank, .. }
            | Self::Astronomer { mut sell_rank, .. }
            | Self::Burnt { mut sell_rank, .. }
            | Self::Bootstraps { mut sell_rank, .. }
            | Self::Canio { mut sell_rank, .. }
            | Self::Triboulet { mut sell_rank, .. }
            | Self::Yorick { mut sell_rank, .. }
            | Self::Chicot { mut sell_rank, .. }
            | Self::Perkeo { mut sell_rank, .. } => sell_rank += 1,
        }
    }

    pub fn name(&self) -> &'static str {
        match self {
            Self::Joker { .. } => "Joker",
            Self::Greedy { .. } => "Greedy Joker",
            Self::Lusty { .. } => "Lusty Joker",
            Self::Wrathful { .. } => "Wrathful Joker",
            Self::Gluttonous { .. } => "Gluttonous Joker",
            Self::Jolly { .. } => "Jolly Joker",
            Self::Zany { .. } => "Zany Joker",
            Self::Mad { .. } => "Mad Joker",
            Self::Crazy { .. } => "Crazy Joker",
            Self::Droll { .. } => "Droll Joker",
            Self::Sly { .. } => "Sly Joker",
            Self::Wily { .. } => "Wily Joker",
            Self::Clever { .. } => "Clever Joker",
            Self::Devious { .. } => "Devious Joker",
            Self::Crafty { .. } => "Crafty Joker",
            Self::Half { .. } => "Half Joker",
            Self::Stencil { .. } => "Joker Stencil",
            Self::Fingers { .. } => "Four Fingers",
            Self::Mime { .. } => "Mime",
            Self::Credit { .. } => "Credit Card",
            Self::Dagger { .. } => "Ceremonial Dagger",
            Self::Banner { .. } => "Banner",
            Self::Mystic { .. } => "Mystic Summit",
            Self::Marble { .. } => "Marble Joker",
            Self::Loyalty { .. } => "Loyalty Card",
            Self::Ball { .. } => "8 Ball",
            Self::Misprint { .. } => "Misprint",
            Self::Dusk { .. } => "Dusk",
            Self::Fist { .. } => "Raised Fist",
            Self::Chaos { .. } => "Chaos the Clown",
            Self::Fibonacci { .. } => "Fibonacci",
            Self::Steel { .. } => "Steel Joker",
            Self::Scary { .. } => "Scary Face",
            Self::Abstract { .. } => "Abstract Joker",
            Self::Gratification { .. } => "Delayed Gratification",
            Self::Hack { .. } => "Hack",
            Self::Pareidolia { .. } => "Pareidolia",
            Self::Michel { .. } => "Gros Michel",
            Self::Steven { .. } => "Even Steven",
            Self::Todd { .. } => "Odd Todd",
            Self::Scholar { .. } => "Scholar",
            Self::Business { .. } => "Business Card",
            Self::Supernova { .. } => "Supernova",
            Self::Bus { .. } => "Ride the Bus",
            Self::Space { .. } => "Space Joker",
            Self::Egg { .. } => "Egg",
            Self::Burglar { .. } => "Burglar",
            Self::Blackboard { .. } => "Blackboard",
            Self::Runner { .. } => "Runner",
            Self::Cream { .. } => "Ice Cream",
            Self::DNA { .. } => "DNA",
            Self::Splash { .. } => "Splash",
            Self::Blue { .. } => "Blue Joker",
            Self::Sixth { .. } => "Sixth Sense",
            Self::Constellation { .. } => "Constellation",
            Self::Hiker { .. } => "Hiker",
            Self::Faceless { .. } => "Faceless Joker",
            Self::Green { .. } => "Green Joker",
            Self::Superposition { .. } => "Superposition",
            Self::List { .. } => "To Do List",
            Self::Cavendish { .. } => "Cavendish",
            Self::Sharp { .. } => "Card Sharp",
            Self::Red { .. } => "Red Card",
            Self::Madness { .. } => "Madness",
            Self::Square { .. } => "Square Joker",
            Self::Seance { .. } => "Seance",
            Self::Riff { .. } => "Riff-Raff",
            Self::Vampire { .. } => "Vampire",
            Self::Shortcut { .. } => "Shortcut",
            Self::Hologram { .. } => "Hologram",
            Self::Vagabond { .. } => "Vagabond",
            Self::Baron { .. } => "Baron",
            Self::Cloud { .. } => "Cloud 9",
            Self::Rocket { .. } => "Rocket",
            Self::Obelisk { .. } => "Obelisk",
            Self::Midas { .. } => "Midas Mask",
            Self::Luchador { .. } => "Luchador",
            Self::Photograph { .. } => "Photograph",
            Self::Gift { .. } => "Gift Card",
            Self::Turtle { .. } => "Turtle Bean",
            Self::Erosion { .. } => "Erosion",
            Self::Parking { .. } => "Reserved Parking",
            Self::Rebate { .. } => "Mail-In Rebate",
            Self::Moon { .. } => "To the Moon",
            Self::Hallucination { .. } => "Hallucination",
            Self::Fortune { .. } => "Fortune Teller",
            Self::Juggler { .. } => "Juggler",
            Self::Drunkard { .. } => "Drunkard",
            Self::Stone { .. } => "Stone Joker",
            Self::Golden { .. } => "Golden Joker",
            Self::Lucky { .. } => "Lucky Cat",
            Self::Baseball { .. } => "Baseball Card",
            Self::Bull { .. } => "Bull",
            Self::Cola { .. } => "Diet Cola",
            Self::Trading { .. } => "Trading Card",
            Self::Flash { .. } => "Flash Card",
            Self::Popcorn { .. } => "Popcorn",
            Self::Trousers { .. } => "Spare Trousers",
            Self::Ancient { .. } => "Ancient Joker",
            Self::Ramen { .. } => "Ramen",
            Self::Walkie { .. } => "Walkie Talkie",
            Self::Seltzer { .. } => "Seltzer",
            Self::Castle { .. } => "Castle",
            Self::Smiley { .. } => "Smiley Face",
            Self::Campfire { .. } => "Campfire",
            Self::Ticket { .. } => "Golden Ticket",
            Self::Bones { .. } => "Mr Bones",
            Self::Acrobat { .. } => "Acrobat",
            Self::Sock { .. } => "Sock and Buskin",
            Self::Swashbuckler { .. } => "Swashbuckler",
            Self::Troubadour { .. } => "Troubadour",
            Self::Certificate { .. } => "Certificate",
            Self::Smeared { .. } => "Smeared Joker",
            Self::Throwback { .. } => "Throwback",
            Self::Chad { .. } => "Hanging Chad",
            Self::Gem { .. } => "Rough Gem",
            Self::Bloodstone { .. } => "Bloodstone",
            Self::Arrowhead { .. } => "Arrowhead",
            Self::Onyx { .. } => "Onyx Agate",
            Self::Glass { .. } => "Glass Joker",
            Self::Showman { .. } => "Showman",
            Self::Flower { .. } => "Flower Pot",
            Self::Blueprint { .. } => "Blueprint",
            Self::Wee { .. } => "Wee Joker",
            Self::Andy { .. } => "Merry Andy",
            Self::Oops { .. } => "Oops! All 6s",
            Self::Idol { .. } => "The Idol",
            Self::Double { .. } => "Seeing Double",
            Self::Matador { .. } => "Matador",
            Self::Road { .. } => "Hit the Road",
            Self::Duo { .. } => "The Duo",
            Self::Trio { .. } => "The Trio",
            Self::Family { .. } => "The Family",
            Self::Order { .. } => "The Order",
            Self::Tribe { .. } => "The Tribe",
            Self::Stuntman { .. } => "Stuntman",
            Self::Invisible { .. } => "Invisible Joker",
            Self::Brainstorm { .. } => "Brainstorm",
            Self::Satellite { .. } => "Satellite",
            Self::Shoot { .. } => "Shoot the Moon",
            Self::License { .. } => "Driver's License",
            Self::Cartomancer { .. } => "Cartomancer",
            Self::Astronomer { .. } => "Astronomer",
            Self::Burnt { .. } => "Burnt Joker",
            Self::Bootstraps { .. } => "Bootstraps",
            Self::Canio { .. } => "Canio",
            Self::Triboulet { .. } => "Triboulet",
            Self::Yorick { .. } => "Yorick",
            Self::Chicot { .. } => "Chicot",
            Self::Perkeo { .. } => "Perkeo",
        }
    }
}
