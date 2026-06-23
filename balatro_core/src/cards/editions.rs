use crate::scoring::ScoreModification;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub enum Edition {
    Foil,
    Holographic,
    Polychrome,
}

impl Edition {
    pub fn on_scored(&self) -> ScoreModification<'_> {
        match self {
            Self::Foil => ScoreModification::Chips(50),
            Self::Holographic => ScoreModification::Mult(10),
            Self::Polychrome => ScoreModification::XMult(1.5),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_foil_on_scored() {
        assert_eq!(Edition::Foil.on_scored(), ScoreModification::Chips(50));
    }

    #[test]
    fn test_holographic_on_scored() {
        assert_eq!(
            Edition::Holographic.on_scored(),
            ScoreModification::Mult(10)
        );
    }

    #[test]
    fn test_polychrome_on_scored() {
        assert_eq!(
            Edition::Polychrome.on_scored(),
            ScoreModification::XMult(1.5)
        );
    }
}
