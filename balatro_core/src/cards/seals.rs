use crate::scoring::ScoreModification;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub enum Seal {
    Gold,
    Red,
    Blue,
    Purple,
}

impl Seal {
    pub fn on_scored(&self) -> Option<ScoreModification<'_>> {
        match self {
            Self::Gold => Some(ScoreModification::Money(3)),
            Self::Red => None, // TODO: implement when retriggers are implemented
            _ => None,
        }
    }

    pub fn on_discard(&self) -> Option<ScoreModification<'_>> {
        match self {
            Self::Purple => None, // TODO: implement when tarot cards are created
            _ => None,
        }
    }

    pub fn on_round_end(&self) -> Option<ScoreModification<'_>> {
        match self {
            Self::Blue => None, // TODO: implement when planet cards can be returned
            _ => None,
        }
    }

    pub fn count_retriggers(&self) -> usize {
        match self {
            Self::Red => 1,
            _ => 0,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_gold_seal_on_scored() {
        let scoring = Seal::Gold.on_scored();

        assert!(scoring.is_some_and(|sm| sm == ScoreModification::Money(3)));
    }

    // TODO: create test for red seal scoring

    #[test]
    fn test_no_effect_on_scored() {
        assert!(Seal::Blue.on_scored().is_none());
        assert!(Seal::Purple.on_scored().is_none());
    }

    // TODO: create test for purple seal discard

    #[test]
    fn test_no_effect_on_discard() {
        assert!(Seal::Gold.on_discard().is_none());
        assert!(Seal::Red.on_discard().is_none());
        assert!(Seal::Blue.on_discard().is_none());
    }

    // TODO: create test for blue seal on round end

    #[test]
    fn test_no_effect_on_round_end() {
        assert!(Seal::Gold.on_round_end().is_none());
        assert!(Seal::Red.on_round_end().is_none());
        assert!(Seal::Purple.on_round_end().is_none());
    }

    #[test]
    fn test_red_seal_adds_retrigger() {
        assert_eq!(Seal::Red.count_retriggers(), 1);
    }

    #[test]
    fn test_no_retriggers() {
        assert_eq!(Seal::Gold.count_retriggers(), 0);
        assert_eq!(Seal::Blue.count_retriggers(), 0);
        assert_eq!(Seal::Purple.count_retriggers(), 0);
    }
}
