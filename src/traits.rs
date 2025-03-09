use std::fmt::{Debug, Display};

use crate::model::{PackType, State};

pub trait Consumable: Debug + Display {
    fn can_use(&self, state: &State) -> bool;
    fn consume(&self, state: &mut State);

    fn is_negative(&self) -> bool;
    fn make_negative(&mut self);

    // fn random() -> impl Consumable;
    // fn pack(pack_type: PackType) -> Vec<impl Consumable>;

    fn name(&self) -> &'static str;
}
