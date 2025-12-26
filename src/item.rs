#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct ItemId(pub u16);

#[derive(Debug, Clone)]
pub struct Item {
    // Engine-level behavior flags / data
}

pub trait ItemRegistry {
    fn get(&self, id: ItemId) -> Option<&Item>;
}
