use std::fmt::Display;

#[derive(Debug)]
pub enum Blind {
    Small,
    Big,
    Hook,
    Ox,
    House,
    Wall,
    Wheel,
    Arm,
    Club,
    Fish,
    Psychic,
    Goad,
    Water,
    Window,
    Manacle,
    Eye,
    Mouth,
    Plant,
    Serpent,
    Pillar,
    Needle,
    Head,
    Tooth,
    Flint,
    Mark,
    Acorn,
    Leaf,
    Vessel,
    Heart,
    Bell,
}

impl Display for Blind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{} (x{} base chips)",
            self.name(),
            self.chips_required(2) / 2
        )
    }
}

impl Blind {
    pub fn valid_boss_blinds(ante: usize) -> Vec<Self> {
        if ante % 8 == 0 {
            return vec![
                Self::Acorn,
                Self::Leaf,
                Self::Vessel,
                Self::Heart,
                Self::Bell,
            ];
        }

        let mut blinds: Vec<Self> = vec![];
        if ante >= 1 {
            blinds.append(&mut vec![
                Self::Hook,
                Self::Club,
                Self::Psychic,
                Self::Goad,
                Self::Window,
                Self::Manacle,
                Self::Pillar,
                Self::Head,
            ]);

            if ante >= 2 {
                blinds.append(&mut vec![
                    Self::House,
                    Self::Wheel,
                    Self::Arm,
                    Self::Fish,
                    Self::Water,
                    Self::Mouth,
                    Self::Needle,
                    Self::Flint,
                    Self::Mark,
                ]);

                if ante >= 3 {
                    blinds.append(&mut vec![Self::Eye, Self::Tooth]);

                    if ante >= 4 {
                        blinds.push(Self::Plant);

                        if ante >= 5 {
                            blinds.push(Self::Serpent);

                            if ante >= 6 {
                                blinds.push(Self::Ox);
                            }
                        }
                    }
                }
            }
        }
        blinds
    }

    pub fn chips_required(&self, base: usize) -> usize {
        match self {
            Self::Small | Self::Needle => base,
            Self::Big => (base as f64 * 1.5) as usize,
            Self::Wall => base * 4,
            Self::Vessel => base * 6,
            _ => base * 2,
        }
    }

    pub fn reward_money(&self) -> usize {
        match self {
            Self::Small => 3,
            Self::Big => 4,
            _ => 5,
        }
    }

    pub fn name(&self) -> &'static str {
        match self {
            Self::Small => "Small Blind",
            Self::Big => "Big Blind",
            Self::Hook => "The Hook",
            Self::Ox => "The Ox",
            Self::House => "The House",
            Self::Wall => "The Wall",
            Self::Wheel => "The Wheel",
            Self::Arm => "The Arm",
            Self::Club => "The Club",
            Self::Fish => "The Fish",
            Self::Psychic => "The Psychic",
            Self::Goad => "The Goad",
            Self::Water => "The Water",
            Self::Window => "The Window",
            Self::Manacle => "The Manacle",
            Self::Eye => "The Eye",
            Self::Mouth => "The Mouth",
            Self::Plant => "The Plant",
            Self::Serpent => "The Serpent",
            Self::Pillar => "The Pillar",
            Self::Needle => "The Needle",
            Self::Head => "The Head",
            Self::Tooth => "The Tooth",
            Self::Flint => "The Flint",
            Self::Mark => "The Mark",
            Self::Acorn => "Amber Acorn",
            Self::Leaf => "Verdant Leaf",
            Self::Vessel => "Violet Vessel",
            Self::Heart => "Crimson Heart",
            Self::Bell => "Cerulean Bell",
        }
    }
}
