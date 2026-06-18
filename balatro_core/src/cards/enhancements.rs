use crate::scoring::ScoreModification;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub enum Enhancement {
    Bonus,
    Mult,
    Wild,
    Glass,
    Steel,
    Stone,
    Gold,
    Lucky,
}

impl Enhancement {
    pub fn on_scored(&self) -> Vec<ScoreModification<'_>> {
        match self {
            Self::Bonus => vec![ScoreModification::Chips(30)],
            Self::Mult => vec![ScoreModification::Mult(4)],
            Self::Glass => vec![ScoreModification::XMult(2.0)],
            Self::Stone => vec![ScoreModification::Chips(50)],
            Self::Lucky => vec![
                ScoreModification::Chance(1, 5, &ScoreModification::Mult(20)),
                ScoreModification::Chance(1, 15, &ScoreModification::Money(20)),
            ],
            _ => vec![],
        }
    }

    pub fn on_held(&self) -> Vec<ScoreModification<'_>> {
        match self {
            Self::Steel => vec![ScoreModification::XMult(1.5)],
            _ => vec![],
        }
    }

    pub fn on_round_end(&self) -> Vec<ScoreModification<'_>> {
        match self {
            Self::Gold => vec![ScoreModification::Money(3)],
            _ => vec![],
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bonus_on_scored() {
        let scoring = Enhancement::Bonus.on_scored();

        assert_eq!(scoring.len(), 1);
        assert!(scoring.contains(&ScoreModification::Chips(30)));
    }

    #[test]
    fn test_mult_on_scored() {
        let scoring = Enhancement::Mult.on_scored();

        assert_eq!(scoring.len(), 1);
        assert!(scoring.contains(&ScoreModification::Mult(4)));
    }

    #[test]
    fn test_wild_on_scored() {
        assert!(Enhancement::Wild.on_scored().is_empty());
    }

    #[test]
    fn test_glass_on_scored() {
        let scoring = Enhancement::Glass.on_scored();

        assert_eq!(scoring.len(), 1);
        assert!(scoring.contains(&ScoreModification::XMult(2.0)));
    }

    #[test]
    fn test_steel_on_scored() {
        assert!(Enhancement::Steel.on_scored().is_empty());
    }

    #[test]
    fn test_stone_on_scored() {
        let scoring = Enhancement::Stone.on_scored();

        assert_eq!(scoring.len(), 1);
        assert!(scoring.contains(&ScoreModification::Chips(50)));
    }

    #[test]
    fn test_gold_on_scored() {
        assert!(Enhancement::Gold.on_scored().is_empty());
    }

    #[test]
    fn test_lucky_on_scored() {
        let scoring = Enhancement::Lucky.on_scored();

        assert_eq!(scoring.len(), 2);
        assert!(scoring.contains(&ScoreModification::Chance(
            1,
            5,
            &ScoreModification::Mult(20)
        )));
        assert!(scoring.contains(&ScoreModification::Chance(
            1,
            15,
            &ScoreModification::Money(20)
        )));
    }

    #[test]
    fn test_bonus_on_held() {
        assert!(Enhancement::Bonus.on_held().is_empty());
    }

    #[test]
    fn test_mult_on_held() {
        assert!(Enhancement::Mult.on_held().is_empty());
    }

    #[test]
    fn test_wild_on_held() {
        assert!(Enhancement::Wild.on_held().is_empty());
    }

    #[test]
    fn test_glass_on_held() {
        assert!(Enhancement::Glass.on_held().is_empty());
    }

    #[test]
    fn test_steel_on_held() {
        let scoring = Enhancement::Steel.on_held();

        assert_eq!(scoring.len(), 1);
        assert!(scoring.contains(&ScoreModification::XMult(1.5)));
    }

    #[test]
    fn test_stone_on_held() {
        assert!(Enhancement::Stone.on_held().is_empty());
    }

    #[test]
    fn test_gold_on_held() {
        assert!(Enhancement::Gold.on_held().is_empty());
    }

    #[test]
    fn test_lucky_on_held() {
        assert!(Enhancement::Lucky.on_held().is_empty());
    }

    #[test]
    fn test_bonus_on_round_end() {
        assert!(Enhancement::Bonus.on_round_end().is_empty());
    }

    #[test]
    fn test_mult_on_round_end() {
        assert!(Enhancement::Mult.on_round_end().is_empty());
    }

    #[test]
    fn test_wild_on_round_end() {
        assert!(Enhancement::Wild.on_round_end().is_empty());
    }

    #[test]
    fn test_glass_on_round_end() {
        assert!(Enhancement::Glass.on_round_end().is_empty());
    }

    #[test]
    fn test_steel_on_round_end() {
        assert!(Enhancement::Steel.on_round_end().is_empty());
    }

    #[test]
    fn test_stone_on_round_end() {
        assert!(Enhancement::Stone.on_round_end().is_empty());
    }

    #[test]
    fn test_gold_on_round_end() {
        let scoring = Enhancement::Gold.on_round_end();

        assert_eq!(scoring.len(), 1);
        assert!(scoring.contains(&ScoreModification::Money(3)));
    }

    #[test]
    fn test_lucky_on_round_end() {
        assert!(Enhancement::Lucky.on_round_end().is_empty());
    }
}
