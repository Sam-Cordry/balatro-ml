use crate::hand_types::HandType;

pub enum Planet {
    Pluto,
    Mercury,
    Uranus,
    Venus,
    Saturn,
    Jupiter,
    Earth,
    Mars,
    Neptune,
    PlanetX,
    Ceres,
    Eris,
}

impl Planet {
    pub fn get_hand(&self) -> HandType {
        match self {
            Self::Pluto => HandType::HighCard,
            Self::Mercury => HandType::Pair,
            Self::Uranus => HandType::TwoPair,
            Self::Venus => HandType::ThreeOfAKind,
            Self::Saturn => HandType::Straight,
            Self::Jupiter => HandType::Flush,
            Self::Earth => HandType::FullHouse,
            Self::Mars => HandType::FourOfAKind,
            Self::Neptune => HandType::StraightFlush,
            Self::PlanetX => HandType::FiveOfAKind,
            Self::Ceres => HandType::FlushHouse,
            Self::Eris => HandType::FlushFive,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::hand_types::HandType;

    use super::*;

    #[test]
    fn test_pluto_effect() {
        assert_eq!(Planet::Pluto.get_hand(), HandType::HighCard);
    }

    #[test]
    fn test_mercury_effect() {
        assert_eq!(Planet::Mercury.get_hand(), HandType::Pair);
    }

    #[test]
    fn test_uranus_effect() {
        assert_eq!(Planet::Uranus.get_hand(), HandType::TwoPair);
    }

    #[test]
    fn test_venus_effect() {
        assert_eq!(Planet::Venus.get_hand(), HandType::ThreeOfAKind);
    }

    #[test]
    fn test_saturn_effect() {
        assert_eq!(Planet::Saturn.get_hand(), HandType::Straight);
    }

    #[test]
    fn test_jupiter_effect() {
        assert_eq!(Planet::Jupiter.get_hand(), HandType::Flush);
    }

    #[test]
    fn test_earth_effect() {
        assert_eq!(Planet::Earth.get_hand(), HandType::FullHouse);
    }

    #[test]
    fn test_mars_effect() {
        assert_eq!(Planet::Mars.get_hand(), HandType::FourOfAKind);
    }

    #[test]
    fn test_neptune_effect() {
        assert_eq!(Planet::Neptune.get_hand(), HandType::StraightFlush);
    }

    #[test]
    fn test_planet_x_effect() {
        assert_eq!(Planet::PlanetX.get_hand(), HandType::FiveOfAKind);
    }

    #[test]
    fn test_ceres_effect() {
        assert_eq!(Planet::Ceres.get_hand(), HandType::FlushHouse);
    }

    #[test]
    fn test_eris_effect() {
        assert_eq!(Planet::Eris.get_hand(), HandType::FlushFive);
    }
}
