use super::State;

pub trait Consumable {
    fn can_use(&self, state: &State) -> bool;
    fn consume(&self, state: &mut State);

    fn is_negative(&self) -> bool;
    fn make_negative(&mut self);

    fn name(&self) -> &'static str;
}
