use std::collections::HashSet;

use rand::{Rng, rand_core::SeedableRng, rngs::Xoshiro128PlusPlus, seq::SliceRandom};
use strum::IntoEnumIterator;

use crate::{
    blinds::BlindType,
    cards::{Card, Rank, Suit},
    hand_types::{HandLevels, get_scoring_cards, identify_hand_type},
    run_state::RunState,
};

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum GamePhase {
    BlindSelect,
    Round,
    Shop,
    Lost,
    Won,
}

#[derive(Debug, Clone)]
pub enum GameAction {
    SelectBlind,
    NextRound,
    PlayHand(Vec<usize>),
    DiscardHand(Vec<usize>),
}

#[derive(Debug, PartialEq)]
pub enum EngineError {
    InvalidPhase,
    NoCardsSelected,
    TooManyCardsSelected,
    InvalidIndex,
    DuplicateIndex,
    NoRemainingDiscards,
    GameOver,
}

pub struct GameEngine {
    rng: Xoshiro128PlusPlus,
    run_state: RunState,
    phase: GamePhase,
    hand_levels: HandLevels,
    hand: Vec<Card>,
    deck: Vec<Card>,
    full_deck: Vec<Card>,
    current_score: usize,
    hands: usize,
    discards: usize,
    hands_left: usize,
    discards_left: usize,
    money: usize,
}

impl GameEngine {
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        let deck: Vec<Card> = Rank::iter()
            .flat_map(|rank| Suit::iter().map(move |suit| Card::new(rank, suit)))
            .collect();

        GameEngine {
            run_state: RunState::new(),
            rng: Xoshiro128PlusPlus::seed_from_u64(rand::rng().next_u64()),
            phase: GamePhase::BlindSelect,
            hand_levels: HandLevels::default(),
            hand: vec![],
            deck: deck.clone(),
            full_deck: deck,
            current_score: 0,
            hands: 4,
            discards: 4,
            hands_left: 4,
            discards_left: 4,
            money: 4,
        }
    }

    pub fn new_with_seed(seed: u64) -> Self {
        let mut engine = Self::new();
        engine.rng = Xoshiro128PlusPlus::seed_from_u64(seed);
        engine
    }

    pub fn phase(&self) -> GamePhase {
        self.phase
    }

    pub fn hand(&self) -> &[Card] {
        &self.hand
    }

    pub fn deck(&self) -> &[Card] {
        &self.deck
    }

    pub fn current_score(&self) -> usize {
        self.current_score
    }

    pub fn hands_left(&self) -> usize {
        self.hands_left
    }

    pub fn discards_left(&self) -> usize {
        self.discards_left
    }

    pub fn money(&self) -> usize {
        self.money
    }

    fn shuffle_deck(&mut self) {
        self.deck.shuffle(&mut self.rng);
    }

    fn draw_to_hand_size(&mut self) {
        self.hand
            .append(&mut self.deck.split_off(self.deck.len() - (8 - self.hand.len())));
        self.hand.sort();
    }

    pub fn handle_action(&mut self, action: GameAction) -> Result<(), EngineError> {
        if self.phase == GamePhase::Lost || self.phase == GamePhase::Won {
            return Result::Err(EngineError::GameOver);
        }

        match action {
            GameAction::SelectBlind => {
                self.validate_select_blind()?;
                self.select_blind();
                Result::Ok(())
            }
            GameAction::NextRound => {
                self.validate_next_round()?;
                self.next_round();
                Result::Ok(())
            }
            GameAction::PlayHand(indices) => {
                self.validate_play_hand(&indices)?;
                self.play(&indices);
                Result::Ok(())
            }
            GameAction::DiscardHand(indices) => {
                self.validate_discard_hand(&indices)?;
                self.discard(&indices);
                Result::Ok(())
            }
        }
    }

    fn validate_select_blind(&self) -> Result<(), EngineError> {
        if self.phase != GamePhase::BlindSelect {
            Result::Err(EngineError::InvalidPhase)
        } else {
            Result::Ok(())
        }
    }

    fn select_blind(&mut self) {
        self.phase = GamePhase::Round;
        self.shuffle_deck();
        self.hand = self.deck.split_off(self.deck.len() - 8);
        self.hand.sort();
    }

    fn validate_next_round(&self) -> Result<(), EngineError> {
        if self.phase != GamePhase::Shop {
            Result::Err(EngineError::InvalidPhase)
        } else {
            Result::Ok(())
        }
    }

    fn next_round(&mut self) {
        self.phase = GamePhase::BlindSelect;
    }

    fn validate_hand_indices(&self, indices: &[usize]) -> Result<(), EngineError> {
        if indices.is_empty() {
            Result::Err(EngineError::NoCardsSelected)
        } else if indices.len() > 5 {
            Result::Err(EngineError::TooManyCardsSelected)
        } else if indices.len() != HashSet::<&usize>::from_iter(indices.iter()).len() {
            Result::Err(EngineError::DuplicateIndex)
        } else if indices.iter().any(|&i| i >= self.hand.len()) {
            Result::Err(EngineError::InvalidIndex)
        } else {
            Result::Ok(())
        }
    }

    fn validate_play_hand(&self, indices: &[usize]) -> Result<(), EngineError> {
        if self.phase != GamePhase::Round {
            Result::Err(EngineError::InvalidPhase)
        } else {
            self.validate_hand_indices(indices)
        }
    }

    fn play(&mut self, indices: &[usize]) {
        self.hands_left -= 1;

        let mut played_cards = vec![];
        let mut mask = vec![true; self.hand.len()];

        for i in indices {
            mask[*i] = false;
            played_cards.push(self.hand[*i]);
        }

        let mut mask_iter = mask.iter();
        self.hand.retain(|_| *mask_iter.next().unwrap());

        let hand_type = identify_hand_type(&played_cards);
        let (mut chips, mult) = self.hand_levels.get_scoring(&hand_type);
        let scoring_cards = get_scoring_cards(&played_cards, &hand_type);

        for card in scoring_cards {
            chips += card.base_chips();
        }

        self.current_score = chips * mult;

        if self.current_score >= self.run_state.target_score() {
            if self.run_state.ante() == 8 && self.run_state.blind() == BlindType::Boss {
                self.phase = GamePhase::Won;
            } else {
                self.end_round();
            }
        } else if self.hands_left == 0 {
            self.phase = GamePhase::Lost;
        } else {
            self.draw_to_hand_size();
        }
    }

    fn end_round(&mut self) {
        self.hand = vec![];
        self.deck = self.full_deck.clone();
        self.current_score = 0;

        self.money += self.hands_left
            + (self.money / 5).min(5)
            + match self.run_state.blind() {
                BlindType::Small => 3,
                BlindType::Big => 4,
                BlindType::Boss => 5,
            };

        self.hands_left = self.hands;
        self.discards_left = self.discards;

        self.run_state.advance();
        self.phase = GamePhase::Shop;
    }

    fn validate_discard_hand(&self, indices: &[usize]) -> Result<(), EngineError> {
        if self.phase != GamePhase::Round {
            Result::Err(EngineError::InvalidPhase)
        } else if self.discards_left == 0 {
            Result::Err(EngineError::NoRemainingDiscards)
        } else {
            self.validate_hand_indices(indices)
        }
    }

    fn discard(&mut self, indices: &[usize]) {
        self.discards_left -= 1;

        let mut mask = vec![true; self.hand.len()];
        for i in indices {
            mask[*i] = false;
        }

        let mut mask_iter = mask.into_iter();
        self.hand.retain(|_| mask_iter.next().unwrap());

        self.draw_to_hand_size();
    }
}

#[allow(unused_must_use)]
#[cfg(test)]
mod tests {
    use std::assert_matches;

    use super::*;

    #[test]
    fn test_engine_init() {
        let engine = GameEngine::new();

        assert_matches!(engine.phase(), GamePhase::BlindSelect);
        assert!(engine.hand().is_empty());
        assert_eq!(engine.deck().len(), 52);
    }

    #[test]
    fn test_select_blind_transition_to_round() {
        let mut engine = GameEngine::new();

        engine.handle_action(GameAction::SelectBlind).unwrap();
        assert_matches!(engine.phase(), GamePhase::Round);
    }

    #[test]
    fn test_select_blind_draws_initial_hand() {
        let mut engine = GameEngine::new();

        engine.handle_action(GameAction::SelectBlind).unwrap();

        assert_eq!(engine.hand().len(), 8);
    }

    #[test]
    fn test_invalid_phase_errors() {
        let mut engine = GameEngine::new();

        // in blind select
        let mut result = engine.handle_action(GameAction::PlayHand(vec![]));
        assert_matches!(result, Result::Err(EngineError::InvalidPhase));

        result = engine.handle_action(GameAction::DiscardHand(vec![]));
        assert_matches!(result, Result::Err(EngineError::InvalidPhase));

        result = engine.handle_action(GameAction::NextRound);
        assert_matches!(result, Result::Err(EngineError::InvalidPhase));

        // move to round
        engine.handle_action(GameAction::SelectBlind).unwrap();

        // in round
        result = engine.handle_action(GameAction::SelectBlind);
        assert_matches!(result, Result::Err(EngineError::InvalidPhase));

        result = engine.handle_action(GameAction::NextRound);
        assert_matches!(result, Result::Err(EngineError::InvalidPhase));

        // beat round
        engine.hand = vec![
            Card::new(Rank::Ace, Suit::Spade),
            Card::new(Rank::Ace, Suit::Spade),
            Card::new(Rank::Ace, Suit::Spade),
            Card::new(Rank::Ace, Suit::Spade),
            Card::new(Rank::Ace, Suit::Spade),
        ];
        engine
            .handle_action(GameAction::PlayHand(vec![0, 1, 2, 3, 4]))
            .unwrap();

        // in shop
        result = engine.handle_action(GameAction::SelectBlind);
        assert_matches!(result, Result::Err(EngineError::InvalidPhase));

        result = engine.handle_action(GameAction::PlayHand(vec![]));
        assert_matches!(result, Result::Err(EngineError::InvalidPhase));

        result = engine.handle_action(GameAction::DiscardHand(vec![]));
        assert_matches!(result, Result::Err(EngineError::InvalidPhase));
    }

    #[test]
    fn test_playing_empty_hand_error() {
        let mut engine = GameEngine::new();
        engine.handle_action(GameAction::SelectBlind).unwrap();

        let result = engine.handle_action(GameAction::PlayHand(vec![]));

        assert_matches!(result, Result::Err(EngineError::NoCardsSelected));
    }

    #[test]
    fn test_playing_too_many_cards_error() {
        let mut engine = GameEngine::new();
        engine.handle_action(GameAction::SelectBlind).unwrap();

        let result = engine.handle_action(GameAction::PlayHand(vec![0, 1, 2, 3, 4, 5, 6, 7]));

        assert_matches!(result, Result::Err(EngineError::TooManyCardsSelected));
    }

    #[test]
    fn test_playing_duplicate_indices() {
        let mut engine = GameEngine::new();
        engine.handle_action(GameAction::SelectBlind).unwrap();

        let result = engine.handle_action(GameAction::PlayHand(vec![0, 0]));

        assert_matches!(result, Result::Err(EngineError::DuplicateIndex));
    }

    #[test]
    fn test_playing_invalid_index() {
        let mut engine = GameEngine::new();
        engine.handle_action(GameAction::SelectBlind).unwrap();

        let result = engine.handle_action(GameAction::PlayHand(vec![8]));

        assert_matches!(result, Result::Err(EngineError::InvalidIndex));
    }

    #[test]
    fn test_playing_card_increases_score() {
        let mut engine = GameEngine::new();
        engine.handle_action(GameAction::SelectBlind);

        engine.handle_action(GameAction::PlayHand(vec![0])).unwrap();

        assert!(engine.current_score() > 0);
    }

    #[test]
    fn test_new_engine_starts_with_full_deck() {
        let engine = GameEngine::new();

        assert_eq!(engine.deck().len(), 52);
    }

    #[test]
    fn test_drawing_cards_from_deck() {
        let mut engine = GameEngine::new();

        engine.handle_action(GameAction::SelectBlind).unwrap();

        assert_eq!(engine.hand().len(), 8);
        assert_eq!(engine.deck().len(), 44);
    }

    #[test]
    fn test_random_hand_drawing() {
        let mut engine_a = GameEngine::new_with_seed(47);
        let mut engine_b = GameEngine::new_with_seed(329);

        engine_a.handle_action(GameAction::SelectBlind).unwrap();
        engine_b.handle_action(GameAction::SelectBlind).unwrap();

        assert_ne!(engine_a.hand(), engine_b.hand());
        assert_ne!(engine_a.deck(), engine_b.deck());
    }

    #[test]
    fn test_playing_card_scores_and_refills() {
        let mut engine = GameEngine::new();
        engine.handle_action(GameAction::SelectBlind).unwrap();

        let original_card = engine.hand()[0];

        engine.handle_action(GameAction::PlayHand(vec![0])).unwrap();

        assert_eq!(engine.hand().len(), 8);
        assert_ne!(engine.hand()[0], original_card);
        assert!(engine.current_score() > 0);
    }

    #[test]
    fn test_hand_is_always_sorted() {
        let mut engine = GameEngine::new();
        engine.handle_action(GameAction::SelectBlind).unwrap();

        assert!(engine.hand().is_sorted(), "Hand is not sorted correctly");
    }

    #[test]
    fn test_playing_hand_proper_scoring() {
        let mut engine = GameEngine::new();
        engine.handle_action(GameAction::SelectBlind).unwrap();

        engine.hand = vec![
            Card::new(Rank::Ace, Suit::Spade),
            Card::new(Rank::Ace, Suit::Heart),
            Card::new(Rank::Five, Suit::Spade),
        ];
        engine
            .handle_action(GameAction::PlayHand(vec![0, 1, 2]))
            .unwrap();

        assert_eq!(engine.current_score(), 64);
    }

    #[test]
    fn test_discard_removes_and_refills() {
        let mut engine = GameEngine::new();
        engine.handle_action(GameAction::SelectBlind).unwrap();

        let original_hand = engine.hand();
        let discarded_cards = vec![original_hand[0], original_hand[1], original_hand[3]];

        engine
            .handle_action(GameAction::DiscardHand(vec![0, 1, 3]))
            .unwrap();

        let new_hand = engine.hand();
        assert_eq!(new_hand.len(), 8);
        assert_eq!(engine.deck().len(), 41);
        for card in discarded_cards {
            assert!(!new_hand.contains(&card));
        }
    }

    #[test]
    fn test_playing_hand_decrements_hands_left() {
        let mut engine = GameEngine::new();
        engine.handle_action(GameAction::SelectBlind).unwrap();
        assert_eq!(engine.hands_left(), 4);

        engine.handle_action(GameAction::PlayHand(vec![0])).unwrap();
        assert_eq!(engine.hands_left(), 3);
    }

    #[test]
    fn test_playing_last_hand_ends_game() {
        let mut engine = GameEngine::new();
        engine.handle_action(GameAction::SelectBlind).unwrap();
        engine.hands_left = 1;

        engine.handle_action(GameAction::PlayHand(vec![0])).unwrap();

        assert_matches!(engine.phase(), GamePhase::Lost);
    }

    #[test]
    fn test_actions_fail_after_ending_game() {
        let mut engine = GameEngine::new();
        engine.phase = GamePhase::Lost;

        let mut result = engine.handle_action(GameAction::SelectBlind);
        assert_matches!(result, Result::Err(EngineError::GameOver));

        result = engine.handle_action(GameAction::NextRound);
        assert_matches!(result, Result::Err(EngineError::GameOver));

        result = engine.handle_action(GameAction::PlayHand(vec![]));
        assert_matches!(result, Result::Err(EngineError::GameOver));

        result = engine.handle_action(GameAction::DiscardHand(vec![]));
        assert_matches!(result, Result::Err(EngineError::GameOver));
    }

    #[test]
    fn test_discard_decrements_discards_left() {
        let mut engine = GameEngine::new();
        engine.handle_action(GameAction::SelectBlind).unwrap();
        assert_eq!(engine.discards_left(), 4);

        engine.handle_action(GameAction::DiscardHand(vec![0]));
        assert_eq!(engine.discards_left(), 3);
    }

    #[test]
    fn test_discarding_past_limit_errors() {
        let mut engine = GameEngine::new();
        engine.handle_action(GameAction::SelectBlind).unwrap();
        engine.discards_left = 0;

        let result = engine.handle_action(GameAction::DiscardHand(vec![0]));

        assert_matches!(result, Result::Err(EngineError::NoRemainingDiscards));
    }

    #[test]
    fn test_beating_round_target() {
        let mut engine = GameEngine::new();
        engine.handle_action(GameAction::SelectBlind).unwrap();
        engine.hand = vec![
            Card::new(Rank::Ace, Suit::Spade),
            Card::new(Rank::Ace, Suit::Spade),
            Card::new(Rank::Ace, Suit::Spade),
            Card::new(Rank::Ace, Suit::Spade),
            Card::new(Rank::Ace, Suit::Spade),
        ];

        engine
            .handle_action(GameAction::PlayHand(vec![0, 1, 2, 3, 4]))
            .unwrap();

        assert_matches!(engine.phase(), GamePhase::Shop);
        assert_eq!(engine.run_state.ante(), 1);
        assert_matches!(engine.run_state.blind(), BlindType::Big);
        assert_eq!(engine.run_state.target_score(), 450);
        assert_eq!(engine.hand().len(), 0);
        assert_eq!(engine.deck().len(), 52);
    }

    #[test]
    fn test_shop_next_round() {
        let mut engine = GameEngine::new();
        engine.handle_action(GameAction::SelectBlind).unwrap();
        engine.hand = vec![
            Card::new(Rank::Ace, Suit::Spade),
            Card::new(Rank::Ace, Suit::Spade),
            Card::new(Rank::Ace, Suit::Spade),
            Card::new(Rank::Ace, Suit::Spade),
            Card::new(Rank::Ace, Suit::Spade),
        ];

        engine
            .handle_action(GameAction::PlayHand(vec![0, 1, 2, 3, 4]))
            .unwrap();

        engine.handle_action(GameAction::NextRound).unwrap();

        assert_matches!(engine.phase(), GamePhase::BlindSelect);
    }

    #[test]
    fn test_beating_ante_8_triggers_victory() {
        let mut engine = GameEngine::new();
        engine.handle_action(GameAction::SelectBlind).unwrap();

        engine.run_state.set_ante(8);
        engine.run_state.set_blind(BlindType::Boss);
        engine.run_state.set_target_score(100);

        engine.hand = vec![
            Card::new(Rank::Ace, Suit::Spade),
            Card::new(Rank::Ace, Suit::Spade),
            Card::new(Rank::Ace, Suit::Spade),
            Card::new(Rank::Ace, Suit::Spade),
            Card::new(Rank::Ace, Suit::Spade),
        ];

        engine
            .handle_action(GameAction::PlayHand(vec![0, 1, 2, 3, 4]))
            .unwrap();

        assert_matches!(engine.phase(), GamePhase::Won);
    }

    #[test]
    fn test_actions_after_victory_errors() {
        let mut engine = GameEngine::new();
        engine.phase = GamePhase::Won;

        let mut result = engine.handle_action(GameAction::SelectBlind);
        assert_matches!(result, Result::Err(EngineError::GameOver));

        result = engine.handle_action(GameAction::NextRound);
        assert_matches!(result, Result::Err(EngineError::GameOver));

        result = engine.handle_action(GameAction::PlayHand(vec![]));
        assert_matches!(result, Result::Err(EngineError::GameOver));

        result = engine.handle_action(GameAction::DiscardHand(vec![]));
        assert_matches!(result, Result::Err(EngineError::GameOver));
    }

    #[test]
    fn test_beating_round_earns_money() {
        let mut engine = GameEngine::new();
        engine.handle_action(GameAction::SelectBlind).unwrap();

        assert_eq!(engine.money(), 4);

        engine.hand = vec![
            Card::new(Rank::Ace, Suit::Spade),
            Card::new(Rank::Ace, Suit::Spade),
            Card::new(Rank::Ace, Suit::Spade),
            Card::new(Rank::Ace, Suit::Spade),
            Card::new(Rank::Ace, Suit::Spade),
        ];
        engine
            .handle_action(GameAction::PlayHand(vec![0, 1, 2, 3, 4]))
            .unwrap();

        assert_eq!(engine.money(), 10);

        engine.handle_action(GameAction::NextRound).unwrap();
        engine.handle_action(GameAction::SelectBlind).unwrap();

        engine.hand = vec![
            Card::new(Rank::Ace, Suit::Spade),
            Card::new(Rank::Ace, Suit::Spade),
            Card::new(Rank::Ace, Suit::Spade),
            Card::new(Rank::Ace, Suit::Spade),
            Card::new(Rank::Ace, Suit::Spade),
        ];
        engine
            .handle_action(GameAction::PlayHand(vec![0, 1, 2, 3, 4]))
            .unwrap();

        assert_eq!(engine.money(), 19);

        engine.handle_action(GameAction::NextRound).unwrap();
        engine.handle_action(GameAction::SelectBlind).unwrap();

        engine.hand = vec![
            Card::new(Rank::Ace, Suit::Spade),
            Card::new(Rank::Ace, Suit::Spade),
            Card::new(Rank::Ace, Suit::Spade),
            Card::new(Rank::Ace, Suit::Spade),
            Card::new(Rank::Ace, Suit::Spade),
        ];
        engine
            .handle_action(GameAction::PlayHand(vec![0, 1, 2, 3, 4]))
            .unwrap();

        assert_eq!(engine.money(), 30);
    }
}
