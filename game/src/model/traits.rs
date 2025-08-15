use crate::model::{cards::Card, State};

pub trait Consumable {
    fn can_use(&self, state: &State, selected_cards: &mut [Card]) -> bool;
    fn consume(&self, state: &mut State, selected_cards: &mut [Card]);

    fn is_negative(&self) -> bool;
    fn make_negative(&mut self);

    fn name(&self) -> &'static str;
}
//
// pub trait Generate {
//     fn choose(rng: Rng, negative: bool) -> Self {}
// }
