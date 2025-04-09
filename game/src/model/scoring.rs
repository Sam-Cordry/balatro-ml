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
    pub fn level_hand(&mut self, hand_type: &HandType, levels: i32) {
        self.hand_scoring
            .get_mut(&hand_type)
            .unwrap()
            .apply(match hand_type {
                HandType::High => ScoreModification {
                    chips: 10 * levels as isize,
                    mult: 1 * levels as isize,
                    ..Default::default()
                },
                HandType::Pair => ScoreModification {
                    chips: 15 * levels as isize,
                    mult: 1 * levels as isize,
                    ..Default::default()
                },
                HandType::TwoPair => ScoreModification {
                    chips: 20 * levels as isize,
                    mult: 1 * levels as isize,
                    ..Default::default()
                },
                HandType::ThreeOfAKind => ScoreModification {
                    chips: 20 * levels as isize,
                    mult: 2 * levels as isize,
                    ..Default::default()
                },
                HandType::Straight => ScoreModification {
                    chips: 30 * levels as isize,
                    mult: 3 * levels as isize,
                    ..Default::default()
                },
                HandType::Flush => ScoreModification {
                    chips: 15 * levels as isize,
                    mult: 2 * levels as isize,
                    ..Default::default()
                },
                HandType::FullHouse => ScoreModification {
                    chips: 25 * levels as isize,
                    mult: 2 * levels as isize,
                    ..Default::default()
                },
                HandType::FourOfAKind => ScoreModification {
                    chips: 30 * levels as isize,
                    mult: 3 * levels as isize,
                    ..Default::default()
                },
                HandType::StraightFlush => ScoreModification {
                    chips: 40 * levels as isize,
                    mult: 4 * levels as isize,
                    ..Default::default()
                },
                HandType::FiveOfAKind => ScoreModification {
                    chips: 35 * levels as isize,
                    mult: 3 * levels as isize,
                    ..Default::default()
                },
                HandType::FlushHouse => ScoreModification {
                    chips: 40 * levels as isize,
                    mult: 4 * levels as isize,
                    ..Default::default()
                },
                HandType::FlushFive => ScoreModification {
                    chips: 50 * levels as isize,
                    mult: 3 * levels as isize,
                    ..Default::default()
                },
            });
    }

    pub fn new(levels: HashMap<HandType, (i32, i32)>) -> Self {
        let mut new = Self::default();
        levels.into_iter().for_each(|e| {
            new.level_hand(&e.0, e.1 .0);
            new.scoring_times.insert(e.0, e.1 .1 as u32);
        });
        new
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
            } else if -modification.chips < <usize as TryInto<isize>>::try_into(self.chips).unwrap()
            {
                todo!("implement")
            }
        }
    }
}
