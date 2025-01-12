use crate::model::{
    *,
    CharacterClass::*,
    Item::*,
    ItemSlot::*
};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Item {
    BronzeSword,
    IronSword,
    SteelSword
}

impl Item {
    pub fn name(&self) -> &'static str {
        match self {
            BronzeSword => "BRONZE SWORD",
            IronSword => "IRON SWORD",
            SteelSword => "STEEL SWORD"
        }
    }

    pub fn equippable(&self, by: &Character, slot: ItemSlot) -> bool {
        match self {
            BronzeSword | IronSword | SteelSword
                => by.class == Warrior && slot == Weapon
        }
    }

    pub fn desc(self) -> &'static [&'static str] {
        match self {
            BronzeSword => &[
                "[1 HAND, WARRIOR REQ.]",
                "GIVES +3 ATK."
            ],
            IronSword => &[
                "[1 HAND, WARRIOR REQ.]",
                "GIVES +5 ATK."
            ],
            SteelSword => &[
                "[1 HAND, WARRIOR REQ.]",
                "GIVES +7 ATK."
            ]
        }
    }
}
