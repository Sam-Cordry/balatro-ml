use std::collections::HashMap;

use crate::model::{
    cards::Card,
    db::{self, CardRow, ConsumableRow, JokerRow, SessionRow},
    scoring::Scoring,
    tarots::Tarot,
    traits::Consumable,
    HandType, State,
};

pub fn create_game_state(
    session: SessionRow,
    cards: Vec<CardRow>,
    consumables: Vec<ConsumableRow>,
    jokers: Vec<JokerRow>,
) -> State {
    let state_consumables: Vec<Box<dyn Consumable>> =
        consumables.into_iter().map(|c| c.into()).collect();
    cards.sort_by(|a, b| a.index.cmp(&b.index));
    jokers.sort_by(|a, b| a.index.cmp(&b.index));
    State {
        seed: session.seed as usize,
        scoring: Scoring::new(HashMap::from([
            (
                HandType::High,
                (session.high_card_level, session.high_card_played),
            ),
            (HandType::Pair, (session.pair_level, session.pair_played)),
            (
                HandType::TwoPair,
                (session.two_pair_level, session.two_pair_played),
            ),
            (
                HandType::ThreeOfAKind,
                (
                    session.three_of_a_kind_level,
                    session.three_of_a_kind_played,
                ),
            ),
            (
                HandType::Straight,
                (session.straight_level, session.straight_played),
            ),
            (HandType::Flush, (session.flush_level, session.flush_played)),
            (
                HandType::FullHouse,
                (session.full_house_level, session.full_house_played),
            ),
            (
                HandType::FourOfAKind,
                (session.four_of_a_kind_level, session.four_of_a_kind_played),
            ),
            (
                HandType::StraightFlush,
                (session.straight_flush_level, session.straight_flush_played),
            ),
            (
                HandType::FiveOfAKind,
                (session.five_of_a_kind_level, session.five_of_a_kind_played),
            ),
            (
                HandType::FlushHouse,
                (session.flush_house_level, session.flush_house_played),
            ),
            (
                HandType::FlushFive,
                (session.flush_five_level, session.flush_five_played),
            ),
        ])),
        last_tarot_planet_used: session.last_consumable,
        consumables: state_consumables,
        consumable_slots: session.consumable_slots as usize,
        money: session.money as usize,
        hand: cards
            .iter()
            .filter(|c| c.in_hand)
            .map(|c| Card::from(*c))
            .collect(),
        hand_size: session.hand_size as usize,
        deck: cards.iter().map(|c| Card::from(*c)).collect(),
        remaining_deck: cards
            .iter()
            .filter(|c| c.in_deck)
            .map(|c| Card::from(*c))
            .collect(),
        jokers: jokers.iter(),
        joker_slots: session.joker_slots as usize,
        hands: session.hands as usize,
        hands_remaining: session.hands_remaining as usize,
        discards_remaining: session.discards_remaining as usize,
        tarots_used: session.tarots_used as usize,
        // planets_used:
        // redeemed_vouchers:
    }
}
