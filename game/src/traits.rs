use std::fmt::{Debug, Display};

use crate::model::{cards::Card, PackType, State};

pub trait Consumable: Debug + Display {
    fn can_use(&self, state: &State) -> bool;
    fn consume(&self, state: &mut State);

    fn is_negative(&self) -> bool;
    fn make_negative(&mut self);

    // fn random() -> impl Consumable;
    // fn pack(pack_type: PackType) -> Vec<impl Consumable>;

    fn name(&self) -> &'static str;
}

pub trait JokerTrait: Debug + Display {
    fn on_blind_select(&mut self, state: &mut State);
    fn on_hand_played(&mut self, state: &mut State);
    fn on_card_score(&mut self, state: &mut State, card: &Card);
    fn on_card_in_hand_effect(&mut self, state: &mut State, card: &Card);
    fn on_independent(&mut self, state: &mut State);
    fn on_discard(&mut self, state: &mut State);
    fn on_shop_close(&mut self, state: &mut State);
    fn on_shop_reroll(&mut self, state: &mut State);
    fn on_pack_open(&mut self, state: &mut State);
    fn get_cost(&self) -> usize;
    fn get_sell_value(&self) -> usize;
    fn name() -> &'static str;
}
