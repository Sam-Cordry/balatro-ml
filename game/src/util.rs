use std::collections::{HashMap, HashSet};
use strum::IntoEnumIterator;

use crate::model::{
    cards::Card,
    db::{CardRow, ConsumableRow, JokerRow, SessionRow},
    jokers::Joker,
    planets::Planet,
    scoring::Scoring,
    traits::Consumable,
    HandType, State, Voucher,
};

pub fn create_game_state(
    session: SessionRow,
    mut cards: Vec<CardRow>,
    consumables: Vec<ConsumableRow>,
    mut jokers: Vec<JokerRow>,
) -> State {
    let state_consumables: Vec<Box<dyn Consumable>> =
        consumables.into_iter().map(|c| c.into()).collect();
    cards.sort_by(|a, b| a.index.cmp(&b.index));
    jokers.sort_by(|a, b| a.index.cmp(&b.index));

    let mut planets_used: HashSet<Planet> = HashSet::default();
    for (planet, used) in Planet::iter().zip(vec![
        session.pluto_used,
        session.mercury_used,
        session.uranus_used,
        session.venus_used,
        session.saturn_used,
        session.jupiter_used,
        session.earth_used,
        session.mars_used,
        session.neptune_used,
        session.planet_x_used,
        session.ceres_used,
        session.eris_used,
    ]) {
        if used {
            planets_used.insert(planet);
        }
    }

    let mut redeemed_vouchers: HashSet<Voucher> = HashSet::default();
    for (voucher, used) in Voucher::iter().zip(vec![
        session.overstock_redeemed,
        session.overstock_plus_redeemed,
        session.clearance_sale_redeemed,
        session.liquidation_redeemed,
        session.hone_redeemed,
        session.glow_up_redeemed,
        session.reroll_surplus_redeemed,
        session.reroll_glut_redeemed,
        session.crystal_ball_redeemed,
        session.omen_globe_redeemed,
        session.telescope_redeemed,
        session.observatory_redeemed,
        session.grabber_redeemed,
        session.nacho_tong_redeemed,
        session.wasteful_redeemed,
        session.recyclomancy_redeemed,
        session.tarot_merchant_redeemed,
        session.tarot_tycoon_redeemed,
        session.planet_merchant_redeemed,
        session.planet_tycoon_redeemed,
        session.seed_money_redeemed,
        session.money_tree_redeemed,
        session.blank_redeemed,
        session.antimatter_redeemed,
        session.magic_trick_redeemed,
        session.illusion_redeemed,
        session.hieroglyph_redeemed,
        session.petroglyph_redeemed,
        session.directors_cut_redeemed,
        session.retcon_redeemed,
        session.paint_brush_redeemed,
        session.palette_redeemed,
    ]) {
        if used {
            redeemed_vouchers.insert(voucher);
        }
    }

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
            .clone()
            .into_iter()
            .filter(|c| c.in_hand)
            .map(Card::from)
            .collect(),
        hand_size: session.hand_size as usize,
        deck: cards.clone().into_iter().map(Card::from).collect(),
        remaining_deck: cards
            .clone()
            .into_iter()
            .filter(|c| c.in_deck)
            .map(Card::from)
            .collect(),
        jokers: jokers.into_iter().map(Joker::from).collect(),
        joker_slots: session.joker_slots as usize,
        hands: session.hands as usize,
        hands_remaining: session.hands_remaining as usize,
        discards: session.discards as usize,
        discards_remaining: session.discards_remaining as usize,
        tarots_used: session.tarots_used as usize,
        planets_used,
        redeemed_vouchers,
    }
}
