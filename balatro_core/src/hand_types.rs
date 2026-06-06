use std::collections::{HashMap, HashSet};

use strum_macros::EnumIter;

use crate::cards::{Card, Rank, Suit};

#[derive(Debug, PartialEq, Clone, Copy, EnumIter)]
pub enum HandType {
    HighCard = 0,
    Pair = 1,
    TwoPair = 2,
    ThreeOfAKind = 3,
    Straight = 4,
    Flush = 5,
    FullHouse = 6,
    FourOfAKind = 7,
    StraightFlush = 8,
    FiveOfAKind = 9,
    FlushHouse = 10,
    FlushFive = 11,
}

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
    pub fn get_scoring(&self, hand_type: &HandType) -> (usize, usize) {
        let idx = *hand_type as usize;
        let add_levels = self.levels[idx] - 1;

        (
            HAND_SCALING[idx].0 + HAND_SCALING[idx].2 * add_levels,
            HAND_SCALING[idx].1 + HAND_SCALING[idx].3 * add_levels,
        )
    }

    pub fn level_up(&mut self, hand_type: &HandType) {
        self.levels[*hand_type as usize] += 1;
    }

    pub fn level_down(&mut self, hand_type: &HandType) {
        let idx = *hand_type as usize;
        self.levels[idx] = (self.levels[idx] - 1).max(1)
    }
}

pub fn identify_hand_type(cards: &[Card]) -> HandType {
    if has_flush_five(cards) {
        HandType::FlushFive
    } else if has_flush_house(cards) {
        HandType::FlushHouse
    } else if has_five_of_a_kind(cards) {
        HandType::FiveOfAKind
    } else if has_straight_flush(cards) {
        HandType::StraightFlush
    } else if has_four_of_a_kind(cards) {
        HandType::FourOfAKind
    } else if has_full_house(cards) {
        HandType::FullHouse
    } else if has_flush(cards) {
        HandType::Flush
    } else if has_straight(cards) {
        HandType::Straight
    } else if has_three_of_a_kind(cards) {
        HandType::ThreeOfAKind
    } else if has_two_pair(cards) {
        HandType::TwoPair
    } else if has_pair(cards) {
        HandType::Pair
    } else {
        HandType::HighCard
    }
}

pub fn has_pair(cards: &[Card]) -> bool {
    get_rank_freq(cards).values().any(|f| *f >= 2)
}

pub fn has_two_pair(cards: &[Card]) -> bool {
    get_rank_freq(cards).values().filter(|f| **f >= 2).count() == 2
}

pub fn has_three_of_a_kind(cards: &[Card]) -> bool {
    get_rank_freq(cards).values().any(|f| *f >= 3)
}

pub fn has_straight(cards: &[Card]) -> bool {
    if cards.len() != 5 {
        return false;
    }

    let mut ranks: Vec<u8> = cards.iter().map(|c| c.rank() as u8).collect();
    ranks.sort();

    ranks[4] - ranks[0] == 4 || ranks == vec![14, 5, 4, 3, 2]
}

pub fn has_flush(cards: &[Card]) -> bool {
    cards.len() == 5 && HashSet::<Suit>::from_iter(cards.iter().map(|c| c.suit())).len() == 1
}

pub fn has_full_house(cards: &[Card]) -> bool {
    let freq = get_rank_freq(cards);

    freq.values().all(|f| *f == 2 || *f == 3)
}

pub fn has_four_of_a_kind(cards: &[Card]) -> bool {
    get_rank_freq(cards).values().any(|f| *f >= 4)
}

pub fn has_straight_flush(cards: &[Card]) -> bool {
    has_straight(cards) && has_flush(cards)
}

pub fn has_five_of_a_kind(cards: &[Card]) -> bool {
    get_rank_freq(cards).values().any(|f| *f == 5)
}

pub fn has_flush_house(cards: &[Card]) -> bool {
    has_flush(cards) && has_full_house(cards)
}

pub fn has_flush_five(cards: &[Card]) -> bool {
    has_flush(cards) && has_five_of_a_kind(cards)
}

fn get_rank_freq(cards: &[Card]) -> HashMap<Rank, usize> {
    cards.iter().fold(HashMap::new(), |mut map, c| {
        map.entry(c.rank()).and_modify(|f| *f += 1).or_insert(1);
        map
    })
}

pub fn get_scoring_cards(cards: &[Card], hand_type: &HandType) -> Vec<Card> {
    match *hand_type {
        HandType::HighCard => {
            let mut vec = cards.to_vec();
            vec.sort();
            vec![vec[0]]
        }
        HandType::Pair => {
            let rank = *get_rank_freq(cards)
                .into_iter()
                .filter_map(|e| if e.1 == 2 { Some(e.0) } else { None })
                .collect::<Vec<Rank>>()
                .first()
                .unwrap();

            cards.iter().copied().filter(|c| c.rank() == rank).collect()
        }
        HandType::TwoPair => {
            let ranks = get_rank_freq(cards)
                .into_iter()
                .filter_map(|e| if e.1 == 2 { Some(e.0) } else { None })
                .collect::<Vec<Rank>>();

            cards
                .iter()
                .copied()
                .filter(|c| ranks.contains(&c.rank()))
                .collect()
        }
        HandType::ThreeOfAKind => {
            let rank = *get_rank_freq(cards)
                .into_iter()
                .filter_map(|e| if e.1 == 3 { Some(e.0) } else { None })
                .collect::<Vec<Rank>>()
                .first()
                .unwrap();

            cards.iter().copied().filter(|c| c.rank() == rank).collect()
        }
        HandType::FourOfAKind => {
            let rank = *get_rank_freq(cards)
                .into_iter()
                .filter_map(|e| if e.1 == 4 { Some(e.0) } else { None })
                .collect::<Vec<Rank>>()
                .first()
                .unwrap();

            cards.iter().copied().filter(|c| c.rank() == rank).collect()
        }
        HandType::Straight
        | HandType::Flush
        | HandType::FullHouse
        | HandType::StraightFlush
        | HandType::FiveOfAKind
        | HandType::FlushHouse
        | HandType::FlushFive => cards.to_vec(),
    }
}

#[cfg(test)]
mod tests {
    use std::assert_matches;

    use strum::IntoEnumIterator;

    use super::*;

    #[test]
    fn test_hand_type_initial_scoring() {
        let hand_levels = HandLevels::default();

        let (hc_chips, hc_mult) = hand_levels.get_scoring(&HandType::HighCard);
        assert_eq!(hc_chips, 5);
        assert_eq!(hc_mult, 1);

        let (pair_chips, pair_mult) = hand_levels.get_scoring(&HandType::Pair);
        assert_eq!(pair_chips, 10);
        assert_eq!(pair_mult, 2);

        let (two_pair_chips, two_pair_mult) = hand_levels.get_scoring(&HandType::TwoPair);
        assert_eq!(two_pair_chips, 20);
        assert_eq!(two_pair_mult, 2);

        let (three_of_a_kind_chips, three_of_a_kind_mult) =
            hand_levels.get_scoring(&HandType::ThreeOfAKind);
        assert_eq!(three_of_a_kind_chips, 30);
        assert_eq!(three_of_a_kind_mult, 3);

        let (straight_chips, straight_mult) = hand_levels.get_scoring(&HandType::Straight);
        assert_eq!(straight_chips, 30);
        assert_eq!(straight_mult, 4);

        let (flush_chips, flush_mult) = hand_levels.get_scoring(&HandType::Flush);
        assert_eq!(flush_chips, 35);
        assert_eq!(flush_mult, 4);

        let (full_house_chips, full_house_mult) = hand_levels.get_scoring(&HandType::FullHouse);
        assert_eq!(full_house_chips, 40);
        assert_eq!(full_house_mult, 4);

        let (four_of_a_kind_chips, four_of_a_kind_mult) =
            hand_levels.get_scoring(&HandType::FourOfAKind);
        assert_eq!(four_of_a_kind_chips, 60);
        assert_eq!(four_of_a_kind_mult, 7);

        let (straight_flush_chips, straight_flush_mult) =
            hand_levels.get_scoring(&HandType::StraightFlush);
        assert_eq!(straight_flush_chips, 100);
        assert_eq!(straight_flush_mult, 8);

        let (five_of_a_kind_chips, five_of_a_kind_mult) =
            hand_levels.get_scoring(&HandType::FiveOfAKind);
        assert_eq!(five_of_a_kind_chips, 120);
        assert_eq!(five_of_a_kind_mult, 12);

        let (flush_house_chips, flush_house_mult) = hand_levels.get_scoring(&HandType::FlushHouse);
        assert_eq!(flush_house_chips, 140);
        assert_eq!(flush_house_mult, 14);

        let (flush_five_chips, flush_five_mult) = hand_levels.get_scoring(&HandType::FlushFive);
        assert_eq!(flush_five_chips, 160);
        assert_eq!(flush_five_mult, 16);
    }

    #[test]
    fn test_hand_type_level_up() {
        let mut hand_levels = HandLevels::default();

        for hand_type in HandType::iter() {
            hand_levels.level_up(&hand_type);
        }

        let (hc_chips, hc_mult) = hand_levels.get_scoring(&HandType::HighCard);
        assert_eq!(hc_chips, 15);
        assert_eq!(hc_mult, 2);

        let (pair_chips, pair_mult) = hand_levels.get_scoring(&HandType::Pair);
        assert_eq!(pair_chips, 25);
        assert_eq!(pair_mult, 3);

        let (two_pair_chips, two_pair_mult) = hand_levels.get_scoring(&HandType::TwoPair);
        assert_eq!(two_pair_chips, 40);
        assert_eq!(two_pair_mult, 3);

        let (three_of_a_kind_chips, three_of_a_kind_mult) =
            hand_levels.get_scoring(&HandType::ThreeOfAKind);
        assert_eq!(three_of_a_kind_chips, 50);
        assert_eq!(three_of_a_kind_mult, 5);

        let (straight_chips, straight_mult) = hand_levels.get_scoring(&HandType::Straight);
        assert_eq!(straight_chips, 60);
        assert_eq!(straight_mult, 7);

        let (flush_chips, flush_mult) = hand_levels.get_scoring(&HandType::Flush);
        assert_eq!(flush_chips, 50);
        assert_eq!(flush_mult, 6);

        let (full_house_chips, full_house_mult) = hand_levels.get_scoring(&HandType::FullHouse);
        assert_eq!(full_house_chips, 65);
        assert_eq!(full_house_mult, 6);

        let (four_of_a_kind_chips, four_of_a_kind_mult) =
            hand_levels.get_scoring(&HandType::FourOfAKind);
        assert_eq!(four_of_a_kind_chips, 90);
        assert_eq!(four_of_a_kind_mult, 10);

        let (straight_flush_chips, straight_flush_mult) =
            hand_levels.get_scoring(&HandType::StraightFlush);
        assert_eq!(straight_flush_chips, 140);
        assert_eq!(straight_flush_mult, 12);

        let (five_of_a_kind_chips, five_of_a_kind_mult) =
            hand_levels.get_scoring(&HandType::FiveOfAKind);
        assert_eq!(five_of_a_kind_chips, 155);
        assert_eq!(five_of_a_kind_mult, 15);

        let (flush_house_chips, flush_house_mult) = hand_levels.get_scoring(&HandType::FlushHouse);
        assert_eq!(flush_house_chips, 180);
        assert_eq!(flush_house_mult, 18);

        let (flush_five_chips, flush_five_mult) = hand_levels.get_scoring(&HandType::FlushFive);
        assert_eq!(flush_five_chips, 210);
        assert_eq!(flush_five_mult, 19);
    }

    #[test]
    fn test_hand_type_level_down() {
        let mut hand_levels = HandLevels::default();

        hand_levels.level_up(&HandType::HighCard);
        assert_eq!(hand_levels.get_scoring(&HandType::HighCard), (15, 2));

        hand_levels.level_down(&HandType::HighCard);
        assert_eq!(hand_levels.get_scoring(&HandType::HighCard), (5, 1));

        hand_levels.level_down(&HandType::HighCard);
        assert_eq!(hand_levels.get_scoring(&HandType::HighCard), (5, 1));
    }

    #[test]
    fn test_identify_high_card() {
        let cards = vec![
            Card::new(Rank::Ace, Suit::Spade),
            Card::new(Rank::Seven, Suit::Club),
            Card::new(Rank::Queen, Suit::Diamond),
            Card::new(Rank::Three, Suit::Heart),
        ];

        let hand_type = identify_hand_type(&cards);

        assert_matches!(hand_type, HandType::HighCard);
    }

    #[test]
    fn test_identify_pair() {
        let cards = vec![
            Card::new(Rank::Ace, Suit::Spade),
            Card::new(Rank::Ace, Suit::Heart),
            Card::new(Rank::Seven, Suit::Club),
            Card::new(Rank::Queen, Suit::Diamond),
            Card::new(Rank::Three, Suit::Heart),
        ];

        let hand_type = identify_hand_type(&cards);

        assert_matches!(hand_type, HandType::Pair);
    }

    #[test]
    fn test_identify_two_pair() {
        let cards = vec![
            Card::new(Rank::Ace, Suit::Spade),
            Card::new(Rank::Ace, Suit::Heart),
            Card::new(Rank::Queen, Suit::Diamond),
            Card::new(Rank::Queen, Suit::Club),
            Card::new(Rank::Three, Suit::Heart),
        ];

        let hand_type = identify_hand_type(&cards);

        assert_matches!(hand_type, HandType::TwoPair);
    }

    #[test]
    fn test_identify_three_of_a_kind() {
        let cards = vec![
            Card::new(Rank::Ace, Suit::Spade),
            Card::new(Rank::Ace, Suit::Heart),
            Card::new(Rank::Ace, Suit::Club),
            Card::new(Rank::Seven, Suit::Club),
            Card::new(Rank::Queen, Suit::Diamond),
        ];

        let hand_type = identify_hand_type(&cards);

        assert_matches!(hand_type, HandType::ThreeOfAKind);
    }

    #[test]
    fn test_identify_straight() {
        let cards = vec![
            Card::new(Rank::Ace, Suit::Spade),
            Card::new(Rank::King, Suit::Club),
            Card::new(Rank::Queen, Suit::Diamond),
            Card::new(Rank::Jack, Suit::Heart),
            Card::new(Rank::Ten, Suit::Diamond),
        ];

        let hand_type = identify_hand_type(&cards);

        assert_matches!(hand_type, HandType::Straight);
    }

    #[test]
    fn test_identify_flush() {
        let cards = vec![
            Card::new(Rank::Ace, Suit::Spade),
            Card::new(Rank::Jack, Suit::Spade),
            Card::new(Rank::Nine, Suit::Spade),
            Card::new(Rank::Six, Suit::Spade),
            Card::new(Rank::Five, Suit::Spade),
        ];

        let hand_type = identify_hand_type(&cards);

        assert_matches!(hand_type, HandType::Flush);
    }

    #[test]
    fn test_identify_full_house() {
        let cards = vec![
            Card::new(Rank::Ace, Suit::Spade),
            Card::new(Rank::Ace, Suit::Heart),
            Card::new(Rank::Ace, Suit::Club),
            Card::new(Rank::Queen, Suit::Diamond),
            Card::new(Rank::Queen, Suit::Spade),
        ];

        let hand_type = identify_hand_type(&cards);

        assert_matches!(hand_type, HandType::FullHouse);
    }

    #[test]
    fn test_identify_four_of_a_kind() {
        let cards = vec![
            Card::new(Rank::Ace, Suit::Spade),
            Card::new(Rank::Ace, Suit::Heart),
            Card::new(Rank::Ace, Suit::Club),
            Card::new(Rank::Ace, Suit::Diamond),
            Card::new(Rank::Queen, Suit::Diamond),
        ];

        let hand_type = identify_hand_type(&cards);

        assert_matches!(hand_type, HandType::FourOfAKind);
    }

    #[test]
    fn test_identify_straight_flush() {
        let cards = vec![
            Card::new(Rank::Ace, Suit::Spade),
            Card::new(Rank::King, Suit::Spade),
            Card::new(Rank::Queen, Suit::Spade),
            Card::new(Rank::Jack, Suit::Spade),
            Card::new(Rank::Ten, Suit::Spade),
        ];

        let hand_type = identify_hand_type(&cards);

        assert_matches!(hand_type, HandType::StraightFlush);
    }

    #[test]
    fn test_identify_five_of_a_kind() {
        let cards = vec![
            Card::new(Rank::Ace, Suit::Spade),
            Card::new(Rank::Ace, Suit::Heart),
            Card::new(Rank::Ace, Suit::Club),
            Card::new(Rank::Ace, Suit::Diamond),
            Card::new(Rank::Ace, Suit::Heart),
        ];

        let hand_type = identify_hand_type(&cards);

        assert_matches!(hand_type, HandType::FiveOfAKind);
    }

    #[test]
    fn test_identify_flush_house() {
        let cards = vec![
            Card::new(Rank::Ace, Suit::Spade),
            Card::new(Rank::Ace, Suit::Spade),
            Card::new(Rank::Ace, Suit::Spade),
            Card::new(Rank::Queen, Suit::Spade),
            Card::new(Rank::Queen, Suit::Spade),
        ];

        let hand_type = identify_hand_type(&cards);

        assert_matches!(hand_type, HandType::FlushHouse);
    }

    #[test]
    fn test_identify_flush_five() {
        let cards = vec![
            Card::new(Rank::Ace, Suit::Spade),
            Card::new(Rank::Ace, Suit::Spade),
            Card::new(Rank::Ace, Suit::Spade),
            Card::new(Rank::Ace, Suit::Spade),
            Card::new(Rank::Ace, Suit::Spade),
        ];

        let hand_type = identify_hand_type(&cards);

        assert_matches!(hand_type, HandType::FlushFive);
    }

    #[test]
    fn test_scoring_cards_high_card() {
        let cards = vec![
            Card::new(Rank::King, Suit::Club),
            Card::new(Rank::Jack, Suit::Heart),
            Card::new(Rank::Nine, Suit::Diamond),
            Card::new(Rank::Six, Suit::Heart),
            Card::new(Rank::Four, Suit::Heart),
        ];

        let scoring = get_scoring_cards(&cards, &HandType::HighCard);

        assert_eq!(scoring[0], Card::new(Rank::King, Suit::Club));
    }

    #[test]
    fn test_scoring_cards_pair() {
        let cards = vec![
            Card::new(Rank::Ace, Suit::Spade),
            Card::new(Rank::Ace, Suit::Club),
            Card::new(Rank::Nine, Suit::Diamond),
            Card::new(Rank::Seven, Suit::Diamond),
            Card::new(Rank::Five, Suit::Club),
        ];

        let scoring = get_scoring_cards(&cards, &HandType::Pair);

        assert_eq!(scoring.len(), 2);
        assert!(scoring.iter().all(|c| c.rank() == Rank::Ace))
    }

    #[test]
    fn test_scoring_cards_two_pair() {
        let cards = vec![
            Card::new(Rank::Queen, Suit::Spade),
            Card::new(Rank::Queen, Suit::Diamond),
            Card::new(Rank::Jack, Suit::Spade),
            Card::new(Rank::Ten, Suit::Club),
            Card::new(Rank::Ten, Suit::Diamond),
        ];

        let scoring = get_scoring_cards(&cards, &HandType::TwoPair);

        assert_eq!(scoring.len(), 4);
        assert!(
            scoring
                .iter()
                .all(|c| c.rank() == Rank::Queen || c.rank() == Rank::Ten)
        );
    }

    #[test]
    fn test_scoring_cards_three_of_a_kind() {
        let cards = vec![
            Card::new(Rank::Eight, Suit::Spade),
            Card::new(Rank::Eight, Suit::Heart),
            Card::new(Rank::Eight, Suit::Club),
            Card::new(Rank::Three, Suit::Club),
            Card::new(Rank::Two, Suit::Spade),
        ];

        let scoring = get_scoring_cards(&cards, &HandType::ThreeOfAKind);

        assert_eq!(scoring.len(), 3);
        assert!(scoring.iter().all(|c| c.rank() == Rank::Eight));
    }

    #[test]
    fn test_scoring_cards_straight() {
        let cards = vec![
            Card::new(Rank::Eight, Suit::Spade),
            Card::new(Rank::Seven, Suit::Heart),
            Card::new(Rank::Six, Suit::Club),
            Card::new(Rank::Five, Suit::Club),
            Card::new(Rank::Four, Suit::Spade),
        ];

        let scoring = get_scoring_cards(&cards, &HandType::Straight);

        assert_eq!(scoring, cards);
    }

    #[test]
    fn test_scoring_cards_flush() {
        let cards = vec![
            Card::new(Rank::Jack, Suit::Club),
            Card::new(Rank::Nine, Suit::Club),
            Card::new(Rank::Six, Suit::Club),
            Card::new(Rank::Five, Suit::Club),
            Card::new(Rank::Four, Suit::Club),
        ];

        let scoring = get_scoring_cards(&cards, &HandType::Flush);

        assert_eq!(scoring, cards);
    }

    #[test]
    fn test_scoring_cards_full_house() {
        let cards = vec![
            Card::new(Rank::Eight, Suit::Spade),
            Card::new(Rank::Eight, Suit::Heart),
            Card::new(Rank::Eight, Suit::Club),
            Card::new(Rank::Three, Suit::Heart),
            Card::new(Rank::Three, Suit::Club),
        ];

        let scoring = get_scoring_cards(&cards, &HandType::FullHouse);

        assert_eq!(scoring, cards);
    }

    #[test]
    fn test_scoring_cards_four_of_a_kind() {
        let cards = vec![
            Card::new(Rank::Jack, Suit::Spade),
            Card::new(Rank::Jack, Suit::Heart),
            Card::new(Rank::Jack, Suit::Club),
            Card::new(Rank::Jack, Suit::Diamond),
            Card::new(Rank::Ten, Suit::Heart),
        ];

        let scoring = get_scoring_cards(&cards, &HandType::FourOfAKind);

        assert_eq!(scoring.len(), 4);
        assert!(scoring.iter().all(|c| c.rank() == Rank::Jack));
    }

    #[test]
    fn test_scoring_cards_straight_flush() {
        let cards = vec![
            Card::new(Rank::Two, Suit::Diamond),
            Card::new(Rank::Three, Suit::Diamond),
            Card::new(Rank::Four, Suit::Diamond),
            Card::new(Rank::Five, Suit::Diamond),
            Card::new(Rank::Six, Suit::Diamond),
        ];

        let scoring = get_scoring_cards(&cards, &HandType::StraightFlush);

        assert_eq!(scoring, cards);
    }

    #[test]
    fn test_scoring_cards_five_of_a_kind() {
        let cards = vec![
            Card::new(Rank::Four, Suit::Spade),
            Card::new(Rank::Four, Suit::Heart),
            Card::new(Rank::Four, Suit::Heart),
            Card::new(Rank::Four, Suit::Club),
            Card::new(Rank::Four, Suit::Diamond),
        ];

        let scoring = get_scoring_cards(&cards, &HandType::FiveOfAKind);

        assert_eq!(scoring, cards);
    }

    #[test]
    fn test_scoring_cards_flush_house() {
        let cards = vec![
            Card::new(Rank::Seven, Suit::Club),
            Card::new(Rank::Seven, Suit::Club),
            Card::new(Rank::Seven, Suit::Club),
            Card::new(Rank::Six, Suit::Club),
            Card::new(Rank::Six, Suit::Club),
        ];

        let scoring = get_scoring_cards(&cards, &HandType::FlushHouse);

        assert_eq!(scoring, cards);
    }

    #[test]
    fn test_scoring_cards_flush_five() {
        let cards = vec![
            Card::new(Rank::Ace, Suit::Spade),
            Card::new(Rank::Ace, Suit::Spade),
            Card::new(Rank::Ace, Suit::Spade),
            Card::new(Rank::Ace, Suit::Spade),
            Card::new(Rank::Ace, Suit::Spade),
        ];

        let scoring = get_scoring_cards(&cards, &HandType::FlushFive);

        assert_eq!(scoring, cards);
    }
}
