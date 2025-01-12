use crate::scene::*;

#[derive(Clone, Debug)]
pub enum Scene {
    Explore(ExploreScene),
    ViewCharacter(ViewCharacterScene),
    ViewInventory(ViewInventoryScene),
    Default // should never be initialized
}

impl Default for Scene {
    fn default() -> Self {
        Self::Default
    }
}
