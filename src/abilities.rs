#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct AbilityId(pub u16);

#[derive(Debug, Clone)]
pub struct Ability {
    // Triggers, modifiers, hooks (later)
}

pub trait AbilityRegistry {
    fn get(&self, id: AbilityId) -> Option<&Ability>;
}

