use crate::{
    *,
    model::{
        *,
        ItemPredicate::*
    }
};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ItemPredicate {
    Equippable(usize, ItemSlot),
    Usable
}

impl ItemPredicate {
    pub fn matches(&self, item: Item, game: &Game) -> bool {
        match self {
            &Equippable(i, slot) => {
                if let Some(character) = game.state.party.get(i) {
                    item.equippable(character, slot)
                } else {
                    false
                }
            },
            Usable => false
        }
    }
}
