use crate::{hands::HandType, scoring::ScoreModification};

const HAND_SCALING: [(usize, usize, usize, usize); 12] = [
    (5, 1, 10, 1),
    (10, 2, 15, 1),
    (20, 2, 20, 1),
    (30, 3, 20, 2),
    (30, 4, 30, 3),
    (35, 4, 15, 2),
    (40, 4, 25, 2),
    (60, 7, 30, 3),
    (100, 8, 40, 4),
    (120, 12, 35, 3),
    (140, 14, 40, 4),
    (160, 16, 50, 3),
];

pub struct HandLevels {
    levels: [usize; 12],
}

impl Default for HandLevels {
    fn default() -> Self {
        Self { levels: [1; 12] }
    }
}

impl HandLevels {
    pub fn get_scoring(&self, hand_type: &HandType) -> Vec<ScoreModification<'_>> {
        let idx = *hand_type as usize;
        let add_levels = self.levels[idx] - 1;

        let chips = HAND_SCALING[idx].0 + HAND_SCALING[idx].2 * add_levels;
        let mult = HAND_SCALING[idx].1 + HAND_SCALING[idx].3 * add_levels;

        vec![
            ScoreModification::Chips(chips),
            ScoreModification::Mult(mult),
        ]
    }

    pub fn level_up(&mut self, hand_type: &HandType) {
        self.levels[*hand_type as usize] += 1;
    }

    pub fn level_down(&mut self, hand_type: &HandType) {
        let idx = *hand_type as usize;
        self.levels[idx] = (self.levels[idx] - 1).max(1)
    }
}

#[cfg(test)]
mod tests {
    use strum::IntoEnumIterator;

    use super::*;

    #[test]
    fn test_hand_type_initial_scoring() {
        let hand_levels = HandLevels::default();

        let hc_scoring = hand_levels.get_scoring(&HandType::HighCard);
        assert!(hc_scoring.contains(&ScoreModification::Chips(5)));
        assert!(hc_scoring.contains(&ScoreModification::Mult(1)));

        let pair_scoring = hand_levels.get_scoring(&HandType::Pair);
        assert!(pair_scoring.contains(&ScoreModification::Chips(10)));
        assert!(pair_scoring.contains(&ScoreModification::Mult(2)));

        let two_pair_scoring = hand_levels.get_scoring(&HandType::TwoPair);
        assert!(two_pair_scoring.contains(&ScoreModification::Chips(20)));
        assert!(two_pair_scoring.contains(&ScoreModification::Mult(2)));

        let three_of_a_kind_scoring = hand_levels.get_scoring(&HandType::ThreeOfAKind);
        assert!(three_of_a_kind_scoring.contains(&ScoreModification::Chips(30)));
        assert!(three_of_a_kind_scoring.contains(&ScoreModification::Mult(3)));

        let straight_scoring = hand_levels.get_scoring(&HandType::Straight);
        assert!(straight_scoring.contains(&ScoreModification::Chips(30)));
        assert!(straight_scoring.contains(&ScoreModification::Mult(4)));

        let flush_scoring = hand_levels.get_scoring(&HandType::Flush);
        assert!(flush_scoring.contains(&ScoreModification::Chips(35)));
        assert!(flush_scoring.contains(&ScoreModification::Mult(4)));

        let full_house_scoring = hand_levels.get_scoring(&HandType::FullHouse);
        assert!(full_house_scoring.contains(&ScoreModification::Chips(40)));
        assert!(full_house_scoring.contains(&ScoreModification::Mult(4)));

        let four_of_a_kind_scoring = hand_levels.get_scoring(&HandType::FourOfAKind);
        assert!(four_of_a_kind_scoring.contains(&ScoreModification::Chips(60)));
        assert!(four_of_a_kind_scoring.contains(&ScoreModification::Mult(7)));

        let straight_flush_scoring = hand_levels.get_scoring(&HandType::StraightFlush);
        assert!(straight_flush_scoring.contains(&ScoreModification::Chips(100)));
        assert!(straight_flush_scoring.contains(&ScoreModification::Mult(8)));

        let five_of_a_kind_scoring = hand_levels.get_scoring(&HandType::FiveOfAKind);
        assert!(five_of_a_kind_scoring.contains(&ScoreModification::Chips(120)));
        assert!(five_of_a_kind_scoring.contains(&ScoreModification::Mult(12)));

        let flush_house_scoring = hand_levels.get_scoring(&HandType::FlushHouse);
        assert!(flush_house_scoring.contains(&ScoreModification::Chips(140)));
        assert!(flush_house_scoring.contains(&ScoreModification::Mult(14)));

        let flush_five_scoring = hand_levels.get_scoring(&HandType::FlushFive);
        assert!(flush_five_scoring.contains(&ScoreModification::Chips(160)));
        assert!(flush_five_scoring.contains(&ScoreModification::Mult(16)));
    }

    #[test]
    fn test_hand_type_level_up() {
        let mut hand_levels = HandLevels::default();

        for hand_type in HandType::iter() {
            hand_levels.level_up(&hand_type);
        }

        let hc_scoring = hand_levels.get_scoring(&HandType::HighCard);
        assert!(hc_scoring.contains(&ScoreModification::Chips(15)));
        assert!(hc_scoring.contains(&ScoreModification::Mult(2)));

        let pair_scoring = hand_levels.get_scoring(&HandType::Pair);
        assert!(pair_scoring.contains(&ScoreModification::Chips(25)));
        assert!(pair_scoring.contains(&ScoreModification::Mult(3)));

        let two_pair_scoring = hand_levels.get_scoring(&HandType::TwoPair);
        assert!(two_pair_scoring.contains(&ScoreModification::Chips(40)));
        assert!(two_pair_scoring.contains(&ScoreModification::Mult(3)));

        let three_of_a_kind_scoring = hand_levels.get_scoring(&HandType::ThreeOfAKind);
        assert!(three_of_a_kind_scoring.contains(&ScoreModification::Chips(50)));
        assert!(three_of_a_kind_scoring.contains(&ScoreModification::Mult(5)));

        let straight_scoring = hand_levels.get_scoring(&HandType::Straight);
        assert!(straight_scoring.contains(&ScoreModification::Chips(60)));
        assert!(straight_scoring.contains(&ScoreModification::Mult(7)));

        let flush_scoring = hand_levels.get_scoring(&HandType::Flush);
        assert!(flush_scoring.contains(&ScoreModification::Chips(50)));
        assert!(flush_scoring.contains(&ScoreModification::Mult(6)));

        let full_house_scoring = hand_levels.get_scoring(&HandType::FullHouse);
        assert!(full_house_scoring.contains(&ScoreModification::Chips(65)));
        assert!(full_house_scoring.contains(&ScoreModification::Mult(6)));

        let four_of_a_kind_scoring = hand_levels.get_scoring(&HandType::FourOfAKind);
        assert!(four_of_a_kind_scoring.contains(&ScoreModification::Chips(90)));
        assert!(four_of_a_kind_scoring.contains(&ScoreModification::Mult(10)));

        let straight_flush_scoring = hand_levels.get_scoring(&HandType::StraightFlush);
        assert!(straight_flush_scoring.contains(&ScoreModification::Chips(140)));
        assert!(straight_flush_scoring.contains(&ScoreModification::Mult(12)));

        let five_of_a_kind_scoring = hand_levels.get_scoring(&HandType::FiveOfAKind);
        assert!(five_of_a_kind_scoring.contains(&ScoreModification::Chips(155)));
        assert!(five_of_a_kind_scoring.contains(&ScoreModification::Mult(15)));

        let flush_house_scoring = hand_levels.get_scoring(&HandType::FlushHouse);
        assert!(flush_house_scoring.contains(&ScoreModification::Chips(180)));
        assert!(flush_house_scoring.contains(&ScoreModification::Mult(18)));

        let flush_five_scoring = hand_levels.get_scoring(&HandType::FlushFive);
        assert!(flush_five_scoring.contains(&ScoreModification::Chips(210)));
        assert!(flush_five_scoring.contains(&ScoreModification::Mult(19)));
    }

    #[test]
    fn test_hand_type_level_down() {
        let mut hand_levels = HandLevels::default();

        hand_levels.level_up(&HandType::HighCard);

        let mut scoring = hand_levels.get_scoring(&HandType::HighCard);
        assert!(scoring.contains(&ScoreModification::Chips(15)));
        assert!(scoring.contains(&ScoreModification::Mult(2)));

        hand_levels.level_down(&HandType::HighCard);

        scoring = hand_levels.get_scoring(&HandType::HighCard);
        assert!(scoring.contains(&ScoreModification::Chips(5)));
        assert!(scoring.contains(&ScoreModification::Mult(1)));

        hand_levels.level_down(&HandType::HighCard);

        scoring = hand_levels.get_scoring(&HandType::HighCard);
        assert!(scoring.contains(&ScoreModification::Chips(5)));
        assert!(scoring.contains(&ScoreModification::Mult(1)));
    }
}
