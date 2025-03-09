use std::collections::HashMap;

use crate::model::HandType;

pub struct Scoring {
    hand_scoring: HashMap<HandType, Score>,
    scoring_times: HashMap<HandType, u32>,
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
    pub fn update_score(&mut self, hand_type: HandType, increasing: bool) {
        let direction: i8 = if increasing { 1 } else { -1 };
        match hand_type {
            HandType::High => {
                self.hand_scoring.get_mut(&hand_type).unwrap().update(
                    Some(10 * direction as u64),
                    Some(1.0 * direction as f64),
                    None,
                );
            }
            HandType::Pair => {
                self.hand_scoring.get_mut(&hand_type).unwrap().update(
                    Some(15 * direction as u64),
                    Some(1.0 * direction as f64),
                    None,
                );
            }
            HandType::TwoPair => {
                self.hand_scoring.get_mut(&hand_type).unwrap().update(
                    Some(20 * direction as u64),
                    Some(1.0 * direction as f64),
                    None,
                );
            }
            HandType::ThreeOfAKind => {
                self.hand_scoring.get_mut(&hand_type).unwrap().update(
                    Some(20 * direction as u64),
                    Some(2.0 * direction as f64),
                    None,
                );
            }
            HandType::Straight => {
                self.hand_scoring.get_mut(&hand_type).unwrap().update(
                    Some(30 * direction as u64),
                    Some(3.0 * direction as f64),
                    None,
                );
            }
            HandType::Flush => {
                self.hand_scoring.get_mut(&hand_type).unwrap().update(
                    Some(15 * direction as u64),
                    Some(2.0 as f64),
                    None,
                );
            }
            HandType::FullHouse => {
                self.hand_scoring.get_mut(&hand_type).unwrap().update(
                    Some(25 * direction as u64),
                    Some(2.0 * direction as f64),
                    None,
                );
            }
            HandType::FourOfAKind => {
                self.hand_scoring.get_mut(&hand_type).unwrap().update(
                    Some(30 * direction as u64),
                    Some(3.0 * direction as f64),
                    None,
                );
            }
            HandType::StraightFlush => {
                self.hand_scoring.get_mut(&hand_type).unwrap().update(
                    Some(40 * direction as u64),
                    Some(4.0 * direction as f64),
                    None,
                );
            }
            HandType::FiveOfAKind => {
                self.hand_scoring.get_mut(&hand_type).unwrap().update(
                    Some(35 * direction as u64),
                    Some(3.0 * direction as f64),
                    None,
                );
            }
            HandType::FlushHouse => {
                self.hand_scoring.get_mut(&hand_type).unwrap().update(
                    Some(40 * direction as u64),
                    Some(4.0 * direction as f64),
                    None,
                );
            }
            HandType::FlushFive => {
                self.hand_scoring.get_mut(&hand_type).unwrap().update(
                    Some(50 * direction as u64),
                    Some(3.0 * direction as f64),
                    None,
                );
            }
        }
    }

    pub fn calculate(&self, hand_type: HandType) -> f64 {
        self.hand_scoring.get(&hand_type).unwrap().calculate()
    }
}

struct Score {
    pub chips: u64,
    pub mult: f64,
}

impl Score {
    pub fn calculate(&self) -> f64 {
        self.chips as f64 * self.mult
    }

    pub fn update(&mut self, chips: Option<u64>, mult: Option<f64>, xmult: Option<f64>) {
        if let Some(chips) = chips {
            self.chips += chips;
        }
        if let Some(mult) = mult {
            self.mult += mult;
        }
        if let Some(xmult) = xmult {
            self.mult *= xmult;
        }
    }
}
