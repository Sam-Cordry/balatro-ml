use crate::scoring::ScoreModification;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub enum Seal {
    Gold,
    Red,
    Blue,
    Purple,
}

impl Seal {
    pub fn on_scored(&self) -> Vec<ScoreModification<'_>> {
        match self {
            Self::Gold => vec![ScoreModification::Money(3)],
            Self::Red => vec![], // TODO: implement when retriggers are implemented
            _ => vec![],
        }
    }

    pub fn on_discard(&self) -> Vec<ScoreModification<'_>> {
        match self {
            Self::Purple => vec![], // TODO: implement when tarot cards are created
            _ => vec![],
        }
    }

    pub fn on_round_end(&self) -> Vec<ScoreModification<'_>> {
        match self {
            Self::Blue => vec![], // TODO: implement when planet cards can be returned
            _ => vec![],
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_gold_seal_on_scored() {
        let scoring = Seal::Gold.on_scored();

        assert_eq!(scoring.len(), 1);
        assert!(scoring.contains(&ScoreModification::Money(3)));
    }

    // TODO: create test for red seal scoring

    #[test]
    fn test_blue_seal_on_scored() {
        assert!(Seal::Blue.on_scored().is_empty());
    }

    #[test]
    fn test_purple_seal_on_scored() {
        assert!(Seal::Purple.on_scored().is_empty());
    }

    #[test]
    fn test_gold_seal_on_discard() {
        assert!(Seal::Gold.on_discard().is_empty());
    }

    #[test]
    fn test_red_seal_on_discard() {
        assert!(Seal::Red.on_discard().is_empty());
    }

    #[test]
    fn test_blue_seal_on_discard() {
        assert!(Seal::Blue.on_discard().is_empty());
    }

    // TODO: create test for purple seal discard

    #[test]
    fn test_gold_seal_on_round_end() {
        assert!(Seal::Gold.on_round_end().is_empty());
    }

    #[test]
    fn test_red_seal_on_round_end() {
        assert!(Seal::Red.on_round_end().is_empty());
    }

    // TODO: create test for blue seal on round end

    #[test]
    fn test_purple_seal_on_round_end() {
        assert!(Seal::Purple.on_round_end().is_empty());
    }
}
