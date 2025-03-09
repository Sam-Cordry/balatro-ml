use std::fmt::Display;

use crate::{
    model::{
        cards::{Card, Enhancement, Suit},
        State,
    },
    traits::Consumable,
};

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Tarot {
    Fool(bool),
    Magician(bool),
    Priestess(bool),
    Empress(bool),
    Emperor(bool),
    Hierophant(bool),
    Lovers(bool),
    Chariot(bool),
    Justice(bool),
    Hermit(bool),
    Wheel(bool),
    Strength(bool),
    Hanged(bool),
    Death(bool),
    Temperance(bool),
    Devil(bool),
    Tower(bool),
    Star(bool),
    Moon(bool),
    Sun(bool),
    Judgement(bool),
    World(bool),
}

impl Display for Tarot {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Fool(negative)
            | Self::Magician(negative)
            | Self::Priestess(negative)
            | Self::Empress(negative)
            | Self::Emperor(negative)
            | Self::Hierophant(negative)
            | Self::Lovers(negative)
            | Self::Chariot(negative)
            | Self::Justice(negative)
            | Self::Hermit(negative)
            | Self::Wheel(negative)
            | Self::Strength(negative)
            | Self::Hanged(negative)
            | Self::Death(negative)
            | Self::Temperance(negative)
            | Self::Devil(negative)
            | Self::Tower(negative)
            | Self::Star(negative)
            | Self::Moon(negative)
            | Self::Sun(negative)
            | Self::Judgement(negative)
            | Self::World(negative) => write!(f, "{} ({})", self.name(), negative),
        }
    }
}

impl Consumable for Tarot {
    fn can_use(&self, state: &State) -> bool {
        match self {
            Self::Hermit(_) | Self::Temperance(_) => true,
            Self::Fool(_) => state.last_tarot_planet_used.name() != "The Fool",
            Self::Magician(_)
            | Self::Empress(_)
            | Self::Hierophant(_)
            | Self::Strength(_)
            | Self::Hanged(_) => state.selected_cards.len() == 1 || state.selected_cards.len() == 2,
            Self::Priestess(negative) | Self::Emperor(negative) => {
                if *negative {
                    state.consumable_slots > state.consumables.len()
                } else {
                    state.consumable_slots >= state.consumables.len()
                }
            }
            Self::Lovers(_)
            | Self::Chariot(_)
            | Self::Justice(_)
            | Self::Devil(_)
            | Self::Tower(_) => state.selected_cards.len() == 1,
            Self::Wheel(_) => todo!("can be done once jokers are implememted"),
            Self::Death(_) => state.selected_cards.len() == 2,
            Self::Star(_) | Self::Moon(_) | Self::Sun(_) | Self::World(_) => {
                state.selected_cards.len() == 1
                    || state.selected_cards.len() == 2
                    || state.selected_cards.len() == 3
            }
            Self::Judgement(_) => todo!("can be done once jokers are implemented"),
        }
    }

    fn consume(&self, state: &mut State) {
        match self {
            Self::Fool(_) => todo!("can implement once random is implemented for consumables"),
            Self::Magician(_) => {
                for card in state.selected_cards.iter_mut() {
                    card.enhancement = Enhancement::Lucky;
                }
            }
            Self::Priestess(_) => {
                todo!("can implement once random is implemented for consumables")
            }
            Self::Empress(_) => {
                for card in state.selected_cards.iter_mut() {
                    card.enhancement = Enhancement::Mult;
                }
            }
            Self::Emperor(_) => todo!("can implement once jokers are implemented"),
            Self::Hierophant(_) => {
                for card in state.selected_cards.iter_mut() {
                    card.enhancement = Enhancement::Bonus;
                }
            }
            Self::Lovers(_) => {
                state.selected_cards.get_mut(0).unwrap().enhancement = Enhancement::Wild
            }
            Self::Chariot(_) => {
                state.selected_cards.get_mut(0).unwrap().enhancement = Enhancement::Steel
            }
            Self::Justice(_) => {
                state.selected_cards.get_mut(0).unwrap().enhancement = Enhancement::Glass
            }
            Self::Hermit(_) => state.money += state.money.clamp(0, 20),
            Self::Wheel(_) => todo!("can implement once jokers are implemented"),
            Self::Strength(_) => {
                for card in state.selected_cards.iter_mut() {
                    card.increment();
                }
            }
            Self::Hanged(_) => {
                for card in state.selected_cards.iter() {
                    state
                        .hand
                        .remove(state.hand.iter().position(|c| *c == *card).unwrap());
                    state
                        .deck
                        .remove(state.hand.iter().position(|c| *c == *card).unwrap());
                }
            }
            Self::Death(_) => {
                let copied: Card = state.selected_cards.get(1).unwrap().clone();
                state.selected_cards.get_mut(0).unwrap().duplicate(&copied);
            }
            Self::Temperance(_) => todo!("can implement once jokers are implemented"),
            Self::Devil(_) => {
                state.selected_cards.get_mut(0).unwrap().enhancement = Enhancement::Gold
            }
            Self::Tower(_) => {
                state.selected_cards.get_mut(0).unwrap().enhancement = Enhancement::Stone
            }
            Self::Star(_) => {
                for card in state.selected_cards.iter_mut() {
                    card.suit = Suit::Diamond
                }
            }
            Self::Moon(_) => {
                for card in state.selected_cards.iter_mut() {
                    card.suit = Suit::Club
                }
            }
            Self::Sun(_) => {
                for card in state.selected_cards.iter_mut() {
                    card.suit = Suit::Heart;
                }
            }
            Self::Judgement(_) => todo!("can implement once jokers are implemented"),
            Self::World(_) => {
                for card in state.selected_cards.iter_mut() {
                    card.suit = Suit::Spade;
                }
            }
        }
        if self.is_negative() {
            state.consumable_slots -= 1;
        }
    }

    fn is_negative(&self) -> bool {
        match self {
            Self::Fool(negative)
            | Self::Magician(negative)
            | Self::Priestess(negative)
            | Self::Empress(negative)
            | Self::Emperor(negative)
            | Self::Hierophant(negative)
            | Self::Lovers(negative)
            | Self::Chariot(negative)
            | Self::Justice(negative)
            | Self::Hermit(negative)
            | Self::Wheel(negative)
            | Self::Strength(negative)
            | Self::Hanged(negative)
            | Self::Death(negative)
            | Self::Temperance(negative)
            | Self::Devil(negative)
            | Self::Tower(negative)
            | Self::Star(negative)
            | Self::Moon(negative)
            | Self::Sun(negative)
            | Self::Judgement(negative)
            | Self::World(negative) => *negative,
        }
    }

    fn make_negative(&mut self) {
        match self {
            Self::Fool(mut negative)
            | Self::Magician(mut negative)
            | Self::Priestess(mut negative)
            | Self::Empress(mut negative)
            | Self::Emperor(mut negative)
            | Self::Hierophant(mut negative)
            | Self::Lovers(mut negative)
            | Self::Chariot(mut negative)
            | Self::Justice(mut negative)
            | Self::Hermit(mut negative)
            | Self::Wheel(mut negative)
            | Self::Strength(mut negative)
            | Self::Hanged(mut negative)
            | Self::Death(mut negative)
            | Self::Temperance(mut negative)
            | Self::Devil(mut negative)
            | Self::Tower(mut negative)
            | Self::Star(mut negative)
            | Self::Moon(mut negative)
            | Self::Sun(mut negative)
            | Self::Judgement(mut negative)
            | Self::World(mut negative) => negative = true,
        }
    }

    fn name(&self) -> &'static str {
        match self {
            Self::Fool(_) => "The Fool",
            Self::Magician(_) => "The Magician",
            Self::Priestess(_) => "The High Priestess",
            Self::Empress(_) => "The Empress",
            Self::Emperor(_) => "The Emperor",
            Self::Hierophant(_) => "The Hierophant",
            Self::Lovers(_) => "The Lovers",
            Self::Chariot(_) => "The Chariot",
            Self::Justice(_) => "Justice",
            Self::Hermit(_) => "The Hermit",
            Self::Wheel(_) => "The Wheel of Fortune",
            Self::Strength(_) => "Strength",
            Self::Hanged(_) => "The Hanged Man",
            Self::Death(_) => "Death",
            Self::Temperance(_) => "Temperance",
            Self::Devil(_) => "The Devil",
            Self::Tower(_) => "The Tower",
            Self::Star(_) => "The Star",
            Self::Moon(_) => "The Moon",
            Self::Sun(_) => "The Sun",
            Self::Judgement(_) => "Judgement",
            Self::World(_) => "The World",
        }
    }
}
