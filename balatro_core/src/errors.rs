#[derive(Debug, PartialEq)]
pub enum GameError {
    InvalidPhase,
    NoCardsSelected,
    TooManyCardsSelected,
    InvalidIndex,
    DuplicateIndex,
    NoRemainingDiscards,
    GameOver,
    UnusableTarot,
}
