use crate::creature::Creature;

/// Maximum party size
pub const MAX_PARTY_SIZE: usize = 6;

/// Represents a group of creatures belonging to a player, NPC, or AI
pub struct Party {
    pub creatures: [Option<Creature>; MAX_PARTY_SIZE],
}

impl Party {
    /// Create a new party from a vector of creatures
    pub fn new(creatures: Vec<Creature>) -> Self {
        let mut arr: [Option<Creature>; MAX_PARTY_SIZE] = Default::default();
        for (i, creature) in creatures.into_iter().take(MAX_PARTY_SIZE).enumerate() {
            arr[i] = Some(creature);
        }
        Self { creatures: arr }
    }

    /// Returns a reference to the first non-fainted creature, if any
    pub fn active(&self) -> Option<&Creature> {
        self.creatures.iter().find_map(|c| match c {
            Some(creature) if !creature.is_fainted() => Some(creature),
            _ => None,
        })
    }

    /// Returns a mutable reference to the first non-fainted creature, if any
    pub fn active_mut(&mut self) -> Option<&mut Creature> {
        self.creatures.iter_mut().find_map(|c| match c {
            Some(creature) if !creature.is_fainted() => Some(creature),
            _ => None,
        })
    }

    /// Get creature at a specific index
    pub fn get(&self, index: usize) -> Option<&Creature> {
        self.creatures.get(index).and_then(|c| c.as_ref())
    }

    /// Get mutable creature at a specific index
    pub fn get_mut(&mut self, index: usize) -> Option<&mut Creature> {
        self.creatures.get_mut(index).and_then(|c| c.as_mut())
    }

    /// Swap two creatures in the party
    pub fn swap(&mut self, i: usize, j: usize) {
        if i < self.creatures.len() && j < self.creatures.len() {
            self.creatures.swap(i, j);
        }
    }

    /// Check if all creatures have fainted
    pub fn all_fainted(&self) -> bool {
        self.creatures.iter().all(|c| match c {
            Some(creature) => creature.is_fainted(),
            None => true,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::creature::{Creature, CreatureId};
    use crate::stats::{IndividualStats, Stat};
    use crate::species::SpeciesId;

    fn create_creature(speed: u16) -> Creature {
        Creature::new(
            CreatureId::new(),
            SpeciesId(1),
            None,
            5,
            IndividualStats {
                attack: Stat::new(10).unwrap(),
                defense: Stat::new(8).unwrap(),
                max_hp: Stat::new(30).unwrap(),
                speed: Stat::new(speed).unwrap(),
            },
        )
    }

    #[test]
    fn active_skips_fainted() {
        let mut party = Party::new(vec![create_creature(10), create_creature(20)]);
        party.active_mut().unwrap().modify_hp(-100); // faint first creature
        let active = party.active().unwrap();
        assert_eq!(active.speed.get(), 20);
    }

    #[test]
    fn all_fainted_detected() {
        let mut party = Party::new(vec![create_creature(10), create_creature(20)]);
        party.get_mut(0).unwrap().modify_hp(-100);
        party.get_mut(1).unwrap().modify_hp(-100);
        assert!(party.all_fainted());
    }

    #[test]
    fn swap_creatures() {
        let mut party = Party::new(vec![create_creature(10), create_creature(20)]);
        party.swap(0, 1);
        let active = party.active().unwrap();
        assert_eq!(active.speed.get(), 20);
    }
}
