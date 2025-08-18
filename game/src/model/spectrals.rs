use rand::Rng;
use std::fmt::Display;
use strum::IntoEnumIterator;

use crate::model::{
    cards::{Card, Seal},
    traits::Generatable,
    Consumable, HandType, State,
};

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Spectral {
    Familiar(bool),
    Grim(bool),
    Incantation(bool),
    Talisman(bool),
    Aura(bool),
    Wraith(bool),
    Sigil(bool),
    Ouija(bool),
    Ectoplasm(bool),
    Immolate(bool),
    Ankh(bool),
    DejaVu(bool),
    Hex(bool),
    Trance(bool),
    Medium(bool),
    Cryptid(bool),
    Soul(bool),
    BlackHole(bool),
}

impl Display for Spectral {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Familiar(negative)
            | Self::Grim(negative)
            | Self::Incantation(negative)
            | Self::Talisman(negative)
            | Self::Aura(negative)
            | Self::Wraith(negative)
            | Self::Sigil(negative)
            | Self::Ouija(negative)
            | Self::Ectoplasm(negative)
            | Self::Immolate(negative)
            | Self::Ankh(negative)
            | Self::DejaVu(negative)
            | Self::Hex(negative)
            | Self::Trance(negative)
            | Self::Medium(negative)
            | Self::Cryptid(negative)
            | Self::Soul(negative)
            | Self::BlackHole(negative) => write!(f, "{} ({})", self.name(), negative),
        }
    }
}

impl Consumable for Spectral {
    fn can_use(&self, state: &State, selected_cards: &mut [Card]) -> bool {
        match self {
            Self::Familiar(_)
            | Self::Grim(_)
            | Self::Incantation(_)
            | Self::Sigil(_)
            | Self::Ouija(_)
            | Self::Immolate(_) => state.hand_size != 1 && !state.hand.is_empty(),
            Self::Talisman(_)
            | Self::DejaVu(_)
            | Self::Trance(_)
            | Self::Medium(_)
            | Self::Cryptid(_) => selected_cards.len() == 1,
            Self::Aura(_) => !state.hand.is_empty(),
            Self::Wraith(_) | Self::Soul(_) => todo!("can implement once jokers are implemented"),
            Self::Ectoplasm(_) | Self::Ankh(_) | Self::Hex(_) => {
                todo!("can implement once jokers are implemented")
            }
            Self::BlackHole(_) => true,
        }
    }

    fn consume(&self, state: &mut State, selected_cards: &mut [Card]) {
        match self {
            Self::Familiar(_) => todo!("randomness required"),
            Self::Grim(_) => todo!("randomness required"),
            Self::Incantation(_) => todo!("randomness required"),
            Self::Talisman(_) => selected_cards.get_mut(0).unwrap().seal = Some(Seal::Gold),
            Self::Aura(_) => todo!("randomness"),
            Self::Wraith(_) => todo!("randomness"),
            Self::Sigil(_) => todo!("randomness"),
            Self::Ouija(_) => todo!("randomness"),
            Self::Ectoplasm(_) => todo!("randomness"),
            Self::Immolate(_) => todo!("randomness"),
            Self::Ankh(_) => todo!("randomness"),
            Self::DejaVu(_) => selected_cards.get_mut(0).unwrap().seal = Some(Seal::Red),
            Self::Hex(_) => todo!("randomness"),
            Self::Trance(_) => selected_cards.get_mut(0).unwrap().seal = Some(Seal::Blue),
            Self::Medium(_) => selected_cards.get_mut(0).unwrap().seal = Some(Seal::Purple),
            Self::Cryptid(_) => {
                for _ in 0..2 {
                    state.hand.push(*selected_cards.first().unwrap());
                    state.deck.push(*selected_cards.first().unwrap());
                }
            }
            Self::Soul(_) => todo!("randomness/jokers"),
            Self::BlackHole(_) => {
                for hand_type in HandType::iter() {
                    state.scoring.level_hand(&hand_type, 1);
                }
            }
        }
        if self.is_negative() {
            state.consumable_slots -= 1;
        }
    }

    fn is_negative(&self) -> bool {
        match self {
            Self::Familiar(negative)
            | Self::Grim(negative)
            | Self::Incantation(negative)
            | Self::Talisman(negative)
            | Self::Aura(negative)
            | Self::Wraith(negative)
            | Self::Sigil(negative)
            | Self::Ouija(negative)
            | Self::Ectoplasm(negative)
            | Self::Immolate(negative)
            | Self::Ankh(negative)
            | Self::DejaVu(negative)
            | Self::Hex(negative)
            | Self::Trance(negative)
            | Self::Medium(negative)
            | Self::Cryptid(negative)
            | Self::Soul(negative)
            | Self::BlackHole(negative) => *negative,
        }
    }

    fn make_negative(&mut self) {
        match *self {
            Self::Familiar(ref mut negative)
            | Self::Grim(ref mut negative)
            | Self::Incantation(ref mut negative)
            | Self::Talisman(ref mut negative)
            | Self::Aura(ref mut negative)
            | Self::Wraith(ref mut negative)
            | Self::Sigil(ref mut negative)
            | Self::Ouija(ref mut negative)
            | Self::Ectoplasm(ref mut negative)
            | Self::Immolate(ref mut negative)
            | Self::Ankh(ref mut negative)
            | Self::DejaVu(ref mut negative)
            | Self::Hex(ref mut negative)
            | Self::Trance(ref mut negative)
            | Self::Medium(ref mut negative)
            | Self::Cryptid(ref mut negative)
            | Self::Soul(ref mut negative)
            | Self::BlackHole(ref mut negative) => *negative = true,
        }
    }

    fn name(&self) -> &'static str {
        match self {
            Self::Familiar(_) => "Familiar",
            Self::Grim(_) => "Grim",
            Self::Incantation(_) => "Incantation",
            Self::Talisman(_) => "Talisman",
            Self::Aura(_) => "Aura",
            Self::Wraith(_) => "Wraith",
            Self::Sigil(_) => "Sigil",
            Self::Ouija(_) => "Ouija",
            Self::Ectoplasm(_) => "Ectoplasm",
            Self::Immolate(_) => "Immolate",
            Self::Ankh(_) => "Ankh",
            Self::DejaVu(_) => "Deja Vu",
            Self::Hex(_) => "Hex",
            Self::Trance(_) => "Trance",
            Self::Medium(_) => "Medium",
            Self::Cryptid(_) => "Cryptid",
            Self::Soul(_) => "The Soul",
            Self::BlackHole(_) => "Black Hole",
        }
    }
}

impl Generatable for Spectral {
    fn gen_single(state: &mut State, negative: bool) -> Self {
        match state.rng.random_range(0..16) {
            0 => Self::Familiar(negative),
            1 => Self::Grim(negative),
            2 => Self::Incantation(negative),
            3 => Self::Talisman(negative),
            4 => Self::Aura(negative),
            5 => Self::Wraith(negative),
            6 => Self::Sigil(negative),
            7 => Self::Ouija(negative),
            8 => Self::Ectoplasm(negative),
            9 => Self::Immolate(negative),
            10 => Self::Ankh(negative),
            11 => Self::DejaVu(negative),
            12 => Self::Hex(negative),
            13 => Self::Trance(negative),
            14 => Self::Medium(negative),
            _ => Self::Cryptid(negative),
        }
    }

    fn gen_pack_single(state: &mut State) -> Box<dyn Consumable> {
        let check = state.rng.random_range(0..1000);
        if check < 3 {
            Box::new(Self::Soul(false))
        } else if check < 6 {
            Box::new(Self::BlackHole(false))
        } else {
            Box::new(Self::gen_single(state, false))
        }
    }
}
