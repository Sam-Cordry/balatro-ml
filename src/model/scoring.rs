use std::collections::HashMap;

use crate::model::{HandType, ScoreModification};

pub struct Scoring {
    pub hand_scoring: HashMap<HandType, Score>,
    pub scoring_times: HashMap<HandType, u32>,
}

impl Default for Scoring {
    fn default() -> Self {
        Self {
            hand_scoring: HashMap::from([
                (
                    HandType::High,
                    Score {
                        chips: 5,
                        mult: 1.0,
                    },
                ),
                (
                    HandType::Pair,
                    Score {
                        chips: 10,
                        mult: 2.0,
                    },
                ),
                (
                    HandType::TwoPair,
                    Score {
                        chips: 20,
                        mult: 2.0,
                    },
                ),
                (
                    HandType::ThreeOfAKind,
                    Score {
                        chips: 30,
                        mult: 3.0,
                    },
                ),
                (
                    HandType::Straight,
                    Score {
                        chips: 30,
                        mult: 4.0,
                    },
                ),
                (
                    HandType::Flush,
                    Score {
                        chips: 35,
                        mult: 4.0,
                    },
                ),
                (
                    HandType::FullHouse,
                    Score {
                        chips: 40,
                        mult: 4.0,
                    },
                ),
                (
                    HandType::FourOfAKind,
                    Score {
                        chips: 60,
                        mult: 7.0,
                    },
                ),
                (
                    HandType::StraightFlush,
                    Score {
                        chips: 100,
                        mult: 8.0,
                    },
                ),
                (
                    HandType::FiveOfAKind,
                    Score {
                        chips: 120,
                        mult: 12.0,
                    },
                ),
                (
                    HandType::FlushHouse,
                    Score {
                        chips: 140,
                        mult: 14.0,
                    },
                ),
                (
                    HandType::FlushFive,
                    Score {
                        chips: 160,
                        mult: 16.0,
                    },
                ),
            ]),
            scoring_times: HashMap::from([
                (HandType::High, 0),
                (HandType::Pair, 0),
                (HandType::TwoPair, 0),
                (HandType::ThreeOfAKind, 0),
                (HandType::Straight, 0),
                (HandType::Flush, 0),
                (HandType::FullHouse, 0),
                (HandType::FourOfAKind, 0),
                (HandType::StraightFlush, 0),
                (HandType::FiveOfAKind, 0),
                (HandType::FlushHouse, 0),
                (HandType::FlushFive, 0),
            ]),
        }
    }
}

impl Scoring {
    pub fn level_hand(&mut self, hand_type: HandType, increasing: bool) {
        let direction: isize = if increasing { 1 } else { -1 };
        self.hand_scoring
            .get_mut(&hand_type)
            .unwrap()
            .apply(match hand_type {
                HandType::High => ScoreModification {
                    chips: 10 * direction,
                    mult: 1 * direction,
                    ..Default::default()
                },
                HandType::Pair => ScoreModification {
                    chips: 15 * direction,
                    mult: 1 * direction,
                    ..Default::default()
                },
                HandType::TwoPair => ScoreModification {
                    chips: 20 * direction,
                    mult: 1 * direction,
                    ..Default::default()
                },
                HandType::ThreeOfAKind => ScoreModification {
                    chips: 20 * direction,
                    mult: 2 * direction,
                    ..Default::default()
                },
                HandType::Straight => ScoreModification {
                    chips: 30 * direction,
                    mult: 3 * direction,
                    ..Default::default()
                },
                HandType::Flush => ScoreModification {
                    chips: 15 * direction,
                    mult: 2 * direction,
                    ..Default::default()
                },
                HandType::FullHouse => ScoreModification {
                    chips: 25 * direction,
                    mult: 2 * direction,
                    ..Default::default()
                },
                HandType::FourOfAKind => ScoreModification {
                    chips: 30 * direction,
                    mult: 3 * direction,
                    ..Default::default()
                },
                HandType::StraightFlush => ScoreModification {
                    chips: 40 * direction,
                    mult: 4 * direction,
                    ..Default::default()
                },
                HandType::FiveOfAKind => ScoreModification {
                    chips: 35 * direction,
                    mult: 3 * direction,
                    ..Default::default()
                },
                HandType::FlushHouse => ScoreModification {
                    chips: 40 * direction,
                    mult: 4 * direction,
                    ..Default::default()
                },
                HandType::FlushFive => ScoreModification {
                    chips: 50 * direction,
                    mult: 3 * direction,
                    ..Default::default()
                },
            });
    }
}

pub struct Score {
    chips: usize,
    mult: f64,
}

impl Score {
    pub fn calculate(&self) -> f64 {
        self.chips as f64 * self.mult
    }

    pub fn apply(&mut self, modification: ScoreModification) {
        for _ in 0..modification.triggers {
            if modification.chips >= 0 {
                self.chips += modification.chips as usize;
                self.mult += modification.mult as f64;
                self.mult *= modification.xmult;
            } else if -modification.chips < self.chips.try_into().unwrap() {
                todo!("implement")
            }
        }
    }
}
