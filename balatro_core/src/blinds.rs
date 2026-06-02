const ANTE_BASE_SCORE: [usize; 8] = [300, 800, 2000, 5000, 11000, 20000, 35000, 50000];

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum BlindType {
    Small,
    Big,
    Boss,
}

pub struct RunState {
    ante: usize,
    blind: BlindType,
    target_score: usize,
}

impl RunState {
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        let mut state = Self {
            ante: 1,
            blind: BlindType::Small,
            target_score: 0,
        };

        state.target_score = state.get_target_score();
        state
    }

    pub fn ante(&self) -> usize {
        self.ante
    }

    pub fn blind(&self) -> BlindType {
        self.blind
    }

    pub fn target_score(&self) -> usize {
        self.target_score
    }

    pub fn advance(&mut self) {
        match self.blind {
            BlindType::Small => self.blind = BlindType::Big,
            BlindType::Big => self.blind = BlindType::Boss,
            BlindType::Boss => {
                self.ante += 1;
                self.blind = BlindType::Small;
            }
        };
        self.target_score = self.get_target_score();
    }

    fn get_target_score(&self) -> usize {
        (ANTE_BASE_SCORE[self.ante - 1] as f32
            * match self.blind {
                BlindType::Small => 1.0,
                BlindType::Big => 1.5,
                BlindType::Boss => 2.0,
            }) as usize
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_run_state_init() {
        let state = RunState::new();

        assert_eq!(state.ante(), 1);
        assert_eq!(state.blind(), BlindType::Small);
        assert_eq!(state.target_score(), 300);
    }

    #[test]
    fn test_run_state_advancement() {
        let mut state = RunState::new();
        assert_eq!(state.ante(), 1);
        assert_eq!(state.blind(), BlindType::Small);

        state.advance();
        assert_eq!(state.ante(), 1);
        assert_eq!(state.blind(), BlindType::Big);

        state.advance();
        assert_eq!(state.ante(), 1);
        assert_eq!(state.blind(), BlindType::Boss);

        state.advance();
        assert_eq!(state.ante(), 2);
        assert_eq!(state.blind(), BlindType::Small);
    }

    #[test]
    fn test_target_chip_score() {
        let mut state = RunState::new();
        assert_eq!(state.target_score, 300);

        state.advance();
        assert_eq!(state.target_score, 450);

        state.advance();
        assert_eq!(state.target_score, 600);

        state.ante = 2;
        state.advance();
        assert_eq!(state.target_score, 2000);

        state.advance();
        assert_eq!(state.target_score, 3000);

        state.advance();
        assert_eq!(state.target_score, 4000);
    }
}

