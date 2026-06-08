#[derive(Debug, PartialEq)]
pub enum ScoreModification<'a> {
    Chips(usize),
    Mult(usize),
    XMult(f32),
    Money(usize),
    Chance(usize, usize, &'a ScoreModification<'a>),
}
