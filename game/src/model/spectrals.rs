use std::fmt::Display;
use strum::IntoEnumIterator;

use crate::model::{
    cards::{Card, Seal},
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
                    state.hand.push(selected_cards.get(0).unwrap().clone());
                    state.deck.push(selected_cards.get(0).unwrap().clone())
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
        match self {
            Self::Familiar(mut negative)
            | Self::Grim(mut negative)
            | Self::Incantation(mut negative)
            | Self::Talisman(mut negative)
            | Self::Aura(mut negative)
            | Self::Wraith(mut negative)
            | Self::Sigil(mut negative)
            | Self::Ouija(mut negative)
            | Self::Ectoplasm(mut negative)
            | Self::Immolate(mut negative)
            | Self::Ankh(mut negative)
            | Self::DejaVu(mut negative)
            | Self::Hex(mut negative)
            | Self::Trance(mut negative)
            | Self::Medium(mut negative)
            | Self::Cryptid(mut negative)
            | Self::Soul(mut negative)
            | Self::BlackHole(mut negative) => negative = true,
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
