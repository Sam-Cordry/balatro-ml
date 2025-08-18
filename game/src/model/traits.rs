use crate::model::{cards::Card, State};

pub trait Consumable {
    fn can_use(&self, state: &State, selected_cards: &mut [Card]) -> bool;
    fn consume(&self, state: &mut State, selected_cards: &mut [Card]);

    fn is_negative(&self) -> bool;
    fn make_negative(&mut self);

    fn name(&self) -> &'static str;
}

pub trait Generatable: Consumable + Sized + 'static {
    fn gen_single(state: &mut State, negative: bool) -> Self;

    fn gen_pack_single(state: &mut State) -> Box<dyn Consumable>;

    fn gen_pack(state: &mut State, n: usize) -> Vec<Box<dyn Consumable>> {
        let duplicate = state.jokers.iter().any(|j| j.name() == "Showman");
        let mut result = vec![];

        while result.len() < n {
            let new = Self::gen_pack_single(state);
            if duplicate
                || result
                    .iter()
                    .all(|t: &Box<dyn Consumable>| t.name() != new.name())
            {
                result.push(new);
            }
        }

        result
    }
}
