use std::fmt::Display;

use crate::model::{
    cards::{Card, Enhancement, Suit},
    db, Consumable, State,
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
    fn can_use(&self, state: &State, selected_cards: &mut [Card]) -> bool {
        match self {
            Self::Hermit(_) | Self::Temperance(_) => true,
            Self::Fool(_) => {
                if let Some(c) = &state.last_tarot_planet_used {
                    return *c == db::Consumable::Fool;
                }
                false
            }
            Self::Magician(_)
            | Self::Empress(_)
            | Self::Hierophant(_)
            | Self::Strength(_)
            | Self::Hanged(_) => selected_cards.len() == 1 || selected_cards.len() == 2,
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
            | Self::Tower(_) => selected_cards.len() == 1,
            Self::Wheel(_) => todo!("can be done once jokers are implememted"),
            Self::Death(_) => selected_cards.len() == 2,
            Self::Star(_) | Self::Moon(_) | Self::Sun(_) | Self::World(_) => {
                selected_cards.len() == 1 || selected_cards.len() == 2 || selected_cards.len() == 3
            }
            Self::Judgement(_) => todo!("can be done once jokers are implemented"),
        }
    }

    fn consume(&self, state: &mut State, selected_cards: &mut [Card]) {
        match self {
            Self::Fool(_) => todo!("can implement once random is implemented for consumables"),
            Self::Magician(_) => {
                for card in selected_cards.iter_mut() {
                    card.enhancement = Some(Enhancement::Lucky);
                }
            }
            Self::Priestess(_) => {
                todo!("can implement once random is implemented for consumables")
            }
            Self::Empress(_) => {
                for card in selected_cards.iter_mut() {
                    card.enhancement = Some(Enhancement::Mult);
                }
            }
            Self::Emperor(_) => todo!("can implement once jokers are implemented"),
            Self::Hierophant(_) => {
                for card in selected_cards.iter_mut() {
                    card.enhancement = Some(Enhancement::Bonus);
                }
            }
            Self::Lovers(_) => {
                selected_cards.get_mut(0).unwrap().enhancement = Some(Enhancement::Wild);
            }
            Self::Chariot(_) => {
                selected_cards.get_mut(0).unwrap().enhancement = Some(Enhancement::Steel);
            }
            Self::Justice(_) => {
                selected_cards.get_mut(0).unwrap().enhancement = Some(Enhancement::Glass);
            }
            Self::Hermit(_) => state.money += state.money.clamp(0, 20),
            Self::Wheel(_) => todo!("can implement once jokers are implemented"),
            Self::Strength(_) => {
                for card in selected_cards.iter_mut() {
                    card.increment();
                }
            }
            Self::Hanged(_) => {
                for card in selected_cards.iter() {
                    state
                        .hand
                        .remove(state.hand.iter().position(|c| *c == *card).unwrap());
                    state
                        .deck
                        .remove(state.hand.iter().position(|c| *c == *card).unwrap());
                }
            }
            Self::Death(_) => {
                let copied: Card = *selected_cards.get(1).unwrap();
                selected_cards.get_mut(0).unwrap().duplicate(&copied);
            }
            Self::Temperance(_) => todo!("can implement once jokers are implemented"),
            Self::Devil(_) => {
                selected_cards.get_mut(0).unwrap().enhancement = Some(Enhancement::Gold);
            }
            Self::Tower(_) => {
                selected_cards.get_mut(0).unwrap().enhancement = Some(Enhancement::Stone);
            }
            Self::Star(_) => {
                for card in selected_cards.iter_mut() {
                    card.suit = Suit::Diamond
                }
            }
            Self::Moon(_) => {
                for card in selected_cards.iter_mut() {
                    card.suit = Suit::Club
                }
            }
            Self::Sun(_) => {
                for card in selected_cards.iter_mut() {
                    card.suit = Suit::Heart;
                }
            }
            Self::Judgement(_) => todo!("can implement once jokers are implemented"),
            Self::World(_) => {
                for card in selected_cards.iter_mut() {
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
        match *self {
            Self::Fool(ref mut negative)
            | Self::Magician(ref mut negative)
            | Self::Priestess(ref mut negative)
            | Self::Empress(ref mut negative)
            | Self::Emperor(ref mut negative)
            | Self::Hierophant(ref mut negative)
            | Self::Lovers(ref mut negative)
            | Self::Chariot(ref mut negative)
            | Self::Justice(ref mut negative)
            | Self::Hermit(ref mut negative)
            | Self::Wheel(ref mut negative)
            | Self::Strength(ref mut negative)
            | Self::Hanged(ref mut negative)
            | Self::Death(ref mut negative)
            | Self::Temperance(ref mut negative)
            | Self::Devil(ref mut negative)
            | Self::Tower(ref mut negative)
            | Self::Star(ref mut negative)
            | Self::Moon(ref mut negative)
            | Self::Sun(ref mut negative)
            | Self::Judgement(ref mut negative)
            | Self::World(ref mut negative) => *negative = true,
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
