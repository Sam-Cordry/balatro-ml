use crate::{
    cards::{Enhancement, Suit},
    errors::GameError,
    planets::Planet,
};

#[derive(Debug, Clone, Copy, Default)]
pub struct Context {
    last_tarot: Option<Tarot>,
    last_planet: Option<Planet>,
    selected_cards: usize,
    empty_consumable_slots: usize,
    num_jokers: usize,
    empty_joker_slots: usize,
}

#[derive(Debug)]
pub enum TarotEffect {
    CreateConsumable(Option<Tarot>, Option<Planet>),
    SelectedCardsAddEnhancement(Enhancement),
    CreatePlanets,
    CreateTarots,
    DoubleMoney,
    RandomJokerEdition,
    SelectedCardsIncrementRank,
    SelectedCardsDestroy,
    ConvertLeftIntoRight,
    AddJokerValue,
    SelectedCardsModifySuit(Suit),
    CreateRandomJoker,
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Tarot {
    Fool,
    Magician,
    HighPriestess,
    Empress,
    Emperor,
    Hierophant,
    Lovers,
    Chariot,
    Justice,
    Hermit,
    WheelOfFortune,
    Strength,
    HangedMan,
    Death,
    Temperance,
    Devil,
    Tower,
    Star,
    Moon,
    Sun,
    Judgement,
    World,
}

impl Tarot {
    pub fn can_use(&self, context: Context) -> bool {
        match self {
            Self::Fool => {
                context.last_tarot.is_some_and(|t| t != Tarot::Fool)
                    || context.last_planet.is_some()
            }
            Self::Magician
            | Self::Empress
            | Self::Hierophant
            | Self::Strength
            | Self::HangedMan => context.selected_cards > 0 && context.selected_cards <= 2,
            Self::HighPriestess | Self::Emperor => context.empty_consumable_slots > 0,
            Self::Lovers | Self::Chariot | Self::Justice | Self::Devil | Self::Tower => {
                context.selected_cards == 1
            }
            Self::Hermit | Self::Temperance => true,
            Self::WheelOfFortune => context.num_jokers > 0,
            Self::Death => context.selected_cards == 2,
            Self::Star | Self::Moon | Self::Sun | Self::World => {
                context.selected_cards > 0 && context.selected_cards <= 3
            }
            Self::Judgement => context.empty_joker_slots > 0,
        }
    }

    pub fn on_use(&self, context: Context) -> Result<TarotEffect, GameError> {
        if !self.can_use(context) {
            return Err(GameError::UnusableTarot);
        }

        Ok(match self {
            Self::Fool => TarotEffect::CreateConsumable(context.last_tarot, context.last_planet),
            Self::Magician => TarotEffect::SelectedCardsAddEnhancement(Enhancement::Lucky),
            Self::HighPriestess => TarotEffect::CreatePlanets,
            Self::Empress => TarotEffect::SelectedCardsAddEnhancement(Enhancement::Mult),
            Self::Emperor => TarotEffect::CreateTarots,
            Self::Hierophant => TarotEffect::SelectedCardsAddEnhancement(Enhancement::Bonus),
            Self::Lovers => TarotEffect::SelectedCardsAddEnhancement(Enhancement::Wild),
            Self::Chariot => TarotEffect::SelectedCardsAddEnhancement(Enhancement::Steel),
            Self::Justice => TarotEffect::SelectedCardsAddEnhancement(Enhancement::Glass),
            Self::Hermit => TarotEffect::DoubleMoney,
            Self::WheelOfFortune => TarotEffect::RandomJokerEdition,
            Self::Strength => TarotEffect::SelectedCardsIncrementRank,
            Self::HangedMan => TarotEffect::SelectedCardsDestroy,
            Self::Death => TarotEffect::ConvertLeftIntoRight,
            Self::Temperance => TarotEffect::AddJokerValue,
            Self::Devil => TarotEffect::SelectedCardsAddEnhancement(Enhancement::Gold),
            Self::Tower => TarotEffect::SelectedCardsAddEnhancement(Enhancement::Stone),
            Self::Star => TarotEffect::SelectedCardsModifySuit(Suit::Diamond),
            Self::Moon => TarotEffect::SelectedCardsModifySuit(Suit::Club),
            Self::Sun => TarotEffect::SelectedCardsModifySuit(Suit::Heart),
            Self::Judgement => TarotEffect::CreateRandomJoker,
            Self::World => TarotEffect::SelectedCardsModifySuit(Suit::Spade),
        })
    }
}

#[cfg(test)]
mod tests {
    use std::assert_matches;

    use super::*;

    #[test]
    fn test_fool() {
        // contexts to check
        let context_valid_tarot = Context {
            last_tarot: Some(Tarot::Magician),
            last_planet: None,
            ..Default::default()
        };
        let context_planet = Context {
            last_tarot: None,
            last_planet: Some(Planet::Jupiter),
            ..Default::default()
        };
        let context_invalid_tarot = Context {
            last_tarot: Some(Tarot::Fool),
            last_planet: None,
            ..Default::default()
        };
        let context_none = Context {
            last_tarot: None,
            last_planet: None,
            ..Default::default()
        };

        // can use success
        assert!(Tarot::Fool.can_use(context_valid_tarot));
        assert!(Tarot::Fool.can_use(context_planet));

        // can use failure
        assert!(!Tarot::Fool.can_use(context_invalid_tarot));
        assert!(!Tarot::Fool.can_use(context_none));

        // on use successful
        assert_matches!(
            Tarot::Fool.on_use(context_valid_tarot),
            Ok(TarotEffect::CreateConsumable(Some(Tarot::Magician), None))
        );
        assert_matches!(
            Tarot::Fool.on_use(context_planet),
            Ok(TarotEffect::CreateConsumable(None, Some(Planet::Jupiter)))
        );

        // on use failure
        assert_matches!(
            Tarot::Fool.on_use(context_invalid_tarot),
            Err(GameError::UnusableTarot)
        );
        assert_matches!(
            Tarot::Fool.on_use(context_none),
            Err(GameError::UnusableTarot)
        );
    }

    #[test]
    fn test_magician() {
        let valid_context = Context {
            selected_cards: 2,
            ..Default::default()
        };
        let invalid_context = Context {
            selected_cards: 0,
            ..Default::default()
        };

        assert!(Tarot::Magician.can_use(valid_context));
        assert!(!Tarot::Magician.can_use(invalid_context));

        assert_matches!(
            Tarot::Magician.on_use(valid_context),
            Ok(TarotEffect::SelectedCardsAddEnhancement(Enhancement::Lucky))
        );
        assert_matches!(
            Tarot::Magician.on_use(invalid_context),
            Err(GameError::UnusableTarot)
        );
    }

    #[test]
    fn test_high_priestess() {
        let valid_context = Context {
            empty_consumable_slots: 2,
            ..Default::default()
        };
        let invalid_context = Context {
            empty_consumable_slots: 0,
            ..Default::default()
        };

        assert!(Tarot::HighPriestess.can_use(valid_context));
        assert!(!Tarot::HighPriestess.can_use(invalid_context));

        assert_matches!(
            Tarot::HighPriestess.on_use(valid_context),
            Ok(TarotEffect::CreatePlanets)
        );
        assert_matches!(
            Tarot::HighPriestess.on_use(invalid_context),
            Err(GameError::UnusableTarot)
        );
    }

    #[test]
    fn test_empress() {
        let valid_context = Context {
            selected_cards: 2,
            ..Default::default()
        };
        let invalid_context = Context {
            selected_cards: 0,
            ..Default::default()
        };

        assert!(Tarot::Empress.can_use(valid_context));
        assert!(!Tarot::Empress.can_use(invalid_context));

        assert_matches!(
            Tarot::Empress.on_use(valid_context),
            Ok(TarotEffect::SelectedCardsAddEnhancement(Enhancement::Mult))
        );
        assert_matches!(
            Tarot::Empress.on_use(invalid_context),
            Err(GameError::UnusableTarot)
        );
    }

    #[test]
    fn test_emperor() {
        let valid_context = Context {
            empty_consumable_slots: 2,
            ..Default::default()
        };
        let invalid_context = Context {
            empty_consumable_slots: 0,
            ..Default::default()
        };

        assert!(Tarot::Emperor.can_use(valid_context));
        assert!(!Tarot::Emperor.can_use(invalid_context));

        assert_matches!(
            Tarot::Emperor.on_use(valid_context),
            Ok(TarotEffect::CreateTarots)
        );
        assert_matches!(
            Tarot::Emperor.on_use(invalid_context),
            Err(GameError::UnusableTarot)
        );
    }

    #[test]
    fn test_hierophant() {
        let valid_context = Context {
            selected_cards: 2,
            ..Default::default()
        };
        let invalid_context = Context {
            selected_cards: 0,
            ..Default::default()
        };

        assert!(Tarot::Hierophant.can_use(valid_context));
        assert!(!Tarot::Hierophant.can_use(invalid_context));

        assert_matches!(
            Tarot::Hierophant.on_use(valid_context),
            Ok(TarotEffect::SelectedCardsAddEnhancement(Enhancement::Bonus))
        );
        assert_matches!(
            Tarot::Hierophant.on_use(invalid_context),
            Err(GameError::UnusableTarot)
        );
    }

    #[test]
    fn test_lovers() {
        let valid_context = Context {
            selected_cards: 1,
            ..Default::default()
        };
        let invalid_context = Context {
            selected_cards: 0,
            ..Default::default()
        };

        assert!(Tarot::Lovers.can_use(valid_context));
        assert!(!Tarot::Lovers.can_use(invalid_context));

        assert_matches!(
            Tarot::Lovers.on_use(valid_context),
            Ok(TarotEffect::SelectedCardsAddEnhancement(Enhancement::Wild))
        );
        assert_matches!(
            Tarot::Lovers.on_use(invalid_context),
            Err(GameError::UnusableTarot)
        );
    }

    #[test]
    fn test_chariot() {
        let valid_context = Context {
            selected_cards: 1,
            ..Default::default()
        };
        let invalid_context = Context {
            selected_cards: 0,
            ..Default::default()
        };

        assert!(Tarot::Chariot.can_use(valid_context));
        assert!(!Tarot::Chariot.can_use(invalid_context));

        assert_matches!(
            Tarot::Chariot.on_use(valid_context),
            Ok(TarotEffect::SelectedCardsAddEnhancement(Enhancement::Steel))
        );
        assert_matches!(
            Tarot::Chariot.on_use(invalid_context),
            Err(GameError::UnusableTarot)
        );
    }

    #[test]
    fn test_justice() {
        let valid_context = Context {
            selected_cards: 1,
            ..Default::default()
        };
        let invalid_context = Context {
            selected_cards: 0,
            ..Default::default()
        };

        assert!(Tarot::Justice.can_use(valid_context));
        assert!(!Tarot::Justice.can_use(invalid_context));

        assert_matches!(
            Tarot::Justice.on_use(valid_context),
            Ok(TarotEffect::SelectedCardsAddEnhancement(Enhancement::Glass))
        );
        assert_matches!(
            Tarot::Justice.on_use(invalid_context),
            Err(GameError::UnusableTarot)
        );
    }

    #[test]
    fn test_hermit() {
        assert!(Tarot::Hermit.can_use(Context::default()));
        assert_matches!(
            Tarot::Hermit.on_use(Context::default()),
            Ok(TarotEffect::DoubleMoney)
        );
    }

    #[test]
    fn test_wheel_of_fortune() {
        let valid_context = Context {
            num_jokers: 2,
            ..Default::default()
        };
        let invalid_context = Context {
            num_jokers: 0,
            ..Default::default()
        };

        assert!(Tarot::WheelOfFortune.can_use(valid_context));
        assert!(!Tarot::WheelOfFortune.can_use(invalid_context));

        assert_matches!(
            Tarot::WheelOfFortune.on_use(valid_context),
            Ok(TarotEffect::RandomJokerEdition)
        );
        assert_matches!(
            Tarot::WheelOfFortune.on_use(invalid_context),
            Err(GameError::UnusableTarot)
        );
    }

    #[test]
    fn test_strength() {
        let valid_context = Context {
            selected_cards: 2,
            ..Default::default()
        };
        let invalid_context = Context {
            selected_cards: 0,
            ..Default::default()
        };

        assert!(Tarot::Strength.can_use(valid_context));
        assert!(!Tarot::Strength.can_use(invalid_context));

        assert_matches!(
            Tarot::Strength.on_use(valid_context),
            Ok(TarotEffect::SelectedCardsIncrementRank)
        );
        assert_matches!(
            Tarot::Strength.on_use(invalid_context),
            Err(GameError::UnusableTarot)
        );
    }

    #[test]
    fn test_hanged_man() {
        let valid_context = Context {
            selected_cards: 2,
            ..Default::default()
        };
        let invalid_context = Context {
            selected_cards: 0,
            ..Default::default()
        };

        assert!(Tarot::HangedMan.can_use(valid_context));
        assert!(!Tarot::HangedMan.can_use(invalid_context));

        assert_matches!(
            Tarot::HangedMan.on_use(valid_context),
            Ok(TarotEffect::SelectedCardsDestroy)
        );
        assert_matches!(
            Tarot::HangedMan.on_use(invalid_context),
            Err(GameError::UnusableTarot)
        );
    }

    #[test]
    fn test_death() {
        let valid_context = Context {
            selected_cards: 2,
            ..Default::default()
        };
        let invalid_context = Context {
            selected_cards: 0,
            ..Default::default()
        };

        assert!(Tarot::Death.can_use(valid_context));
        assert!(!Tarot::Death.can_use(invalid_context));

        assert_matches!(
            Tarot::Death.on_use(valid_context),
            Ok(TarotEffect::ConvertLeftIntoRight)
        );
        assert_matches!(
            Tarot::Death.on_use(invalid_context),
            Err(GameError::UnusableTarot)
        );
    }

    #[test]
    fn test_temperance() {
        assert!(Tarot::Temperance.can_use(Context::default()));
        assert_matches!(
            Tarot::Temperance.on_use(Context::default()),
            Ok(TarotEffect::AddJokerValue)
        );
    }

    #[test]
    fn test_devil() {
        let valid_context = Context {
            selected_cards: 1,
            ..Default::default()
        };
        let invalid_context = Context {
            selected_cards: 0,
            ..Default::default()
        };

        assert!(Tarot::Devil.can_use(valid_context));
        assert!(!Tarot::Devil.can_use(invalid_context));

        assert_matches!(
            Tarot::Devil.on_use(valid_context),
            Ok(TarotEffect::SelectedCardsAddEnhancement(Enhancement::Gold))
        );
        assert_matches!(
            Tarot::Devil.on_use(invalid_context),
            Err(GameError::UnusableTarot)
        );
    }

    #[test]
    fn test_tower() {
        let valid_context = Context {
            selected_cards: 1,
            ..Default::default()
        };
        let invalid_context = Context {
            selected_cards: 0,
            ..Default::default()
        };

        assert!(Tarot::Tower.can_use(valid_context));
        assert!(!Tarot::Tower.can_use(invalid_context));

        assert_matches!(
            Tarot::Tower.on_use(valid_context),
            Ok(TarotEffect::SelectedCardsAddEnhancement(Enhancement::Stone))
        );
        assert_matches!(
            Tarot::Tower.on_use(invalid_context),
            Err(GameError::UnusableTarot)
        );
    }

    #[test]
    fn test_star() {
        let valid_context = Context {
            selected_cards: 2,
            ..Default::default()
        };
        let invalid_context = Context {
            selected_cards: 0,
            ..Default::default()
        };

        assert!(Tarot::Star.can_use(valid_context));
        assert!(!Tarot::Star.can_use(invalid_context));

        assert_matches!(
            Tarot::Star.on_use(valid_context),
            Ok(TarotEffect::SelectedCardsModifySuit(Suit::Diamond))
        );
        assert_matches!(
            Tarot::Star.on_use(invalid_context),
            Err(GameError::UnusableTarot)
        );
    }

    #[test]
    fn test_moon() {
        let valid_context = Context {
            selected_cards: 2,
            ..Default::default()
        };
        let invalid_context = Context {
            selected_cards: 0,
            ..Default::default()
        };

        assert!(Tarot::Moon.can_use(valid_context));
        assert!(!Tarot::Moon.can_use(invalid_context));

        assert_matches!(
            Tarot::Moon.on_use(valid_context),
            Ok(TarotEffect::SelectedCardsModifySuit(Suit::Club))
        );
        assert_matches!(
            Tarot::Moon.on_use(invalid_context),
            Err(GameError::UnusableTarot)
        );
    }

    #[test]
    fn test_sun() {
        let valid_context = Context {
            selected_cards: 2,
            ..Default::default()
        };
        let invalid_context = Context {
            selected_cards: 0,
            ..Default::default()
        };

        assert!(Tarot::Sun.can_use(valid_context));
        assert!(!Tarot::Sun.can_use(invalid_context));

        assert_matches!(
            Tarot::Sun.on_use(valid_context),
            Ok(TarotEffect::SelectedCardsModifySuit(Suit::Heart))
        );
        assert_matches!(
            Tarot::Sun.on_use(invalid_context),
            Err(GameError::UnusableTarot)
        );
    }

    #[test]
    fn test_judgement() {
        let valid_context = Context {
            empty_joker_slots: 1,
            ..Default::default()
        };
        let invalid_context = Context {
            empty_joker_slots: 0,
            ..Default::default()
        };

        assert!(Tarot::Judgement.can_use(valid_context));
        assert!(!Tarot::Judgement.can_use(invalid_context));

        assert_matches!(
            Tarot::Judgement.on_use(valid_context),
            Ok(TarotEffect::CreateRandomJoker)
        );
        assert_matches!(
            Tarot::Judgement.on_use(invalid_context),
            Err(GameError::UnusableTarot)
        );
    }

    #[test]
    fn test_world() {
        let valid_context = Context {
            selected_cards: 2,
            ..Default::default()
        };
        let invalid_context = Context {
            selected_cards: 0,
            ..Default::default()
        };

        assert!(Tarot::World.can_use(valid_context));
        assert!(!Tarot::World.can_use(invalid_context));

        assert_matches!(
            Tarot::World.on_use(valid_context),
            Ok(TarotEffect::SelectedCardsModifySuit(Suit::Spade))
        );
        assert_matches!(
            Tarot::World.on_use(invalid_context),
            Err(GameError::UnusableTarot)
        );
    }
}
