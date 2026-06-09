use strum_macros::EnumIter;

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
