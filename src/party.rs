use crate::creature::Creature;

/// Represents a party of creatures (like Pokémon party)
pub struct Party {
    pub creatures: [Creature; 6], // Fixed size for simplicity
}

impl Party {
    /// Create a new party from a fixed array of creatures
    pub fn new(creatures: [Creature; 6]) -> Self {
        Self { creatures }
    }

    /// Returns a reference to the first available (non-fainted) creature
    pub fn active(&self) -> Option<&Creature> {
        self.creatures.iter().find(|c| !c.is_fainted())
    }

    /// Returns a mutable reference to the first available creature
    pub fn active_mut(&mut self) -> Option<&mut Creature> {
        self.creatures.iter_mut().find(|c| !c.is_fainted())
    }

    /// Checks if all creatures in the party are fainted
    pub fn all_fainted(&self) -> bool {
        self.creatures.iter().all(|c| c.is_fainted())
    }

    /// Returns a slice of all creatures in the party
    pub fn all(&self) -> &[Creature] {
        &self.creatures
    }

    /// Returns a mutable slice of all creatures
    pub fn all_mut(&mut self) -> &mut [Creature] {
        &mut self.creatures
    }

    /// Swaps the creatures at two indices
    ///
    /// Returns `true` if the swap succeeded, `false` if indices are out of bounds
    pub fn swap(&mut self, idx_a: usize, idx_b: usize) -> bool {
        if idx_a >= self.creatures.len() || idx_b >= self.creatures.len() {
            return false;
        }
        self.creatures.swap(idx_a, idx_b);
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::creature::{Creature, CreatureId};
    use crate::species::{Species, SpeciesId, SpeciesName};
    use crate::stats::{BaseStats, IndividualStats, Stat};

    /// Helper to create a simple test creature with a specific HP
    fn make_test_creature(hp: u16) -> Creature {
        let id = CreatureId::new(); // Unique ID for each creature
        // Construct BaseStats using the Stat newtype
        let base_stats = BaseStats {
            attack: Stat::new(10).unwrap(),
            defense: Stat::new(10).unwrap(),
            max_hp: Stat::new(hp).unwrap(),
            speed: Stat::new(10).unwrap(),
        };

        // Create Species using the base stats
        let species = Species::new(SpeciesId(1), SpeciesName::new("TestSpecies"), base_stats);

        // Generate individual stats from species.base_stats
        let individual_stats = IndividualStats::from_base(&species.base_stats);

        // Construct the creature
        Creature::new(id, species.id, None, 1, individual_stats)
    }

    #[test]
    fn active_returns_first_non_fainted() {
        let mut creatures = [
            make_test_creature(10),
            make_test_creature(15),
            make_test_creature(17),
            make_test_creature(11),
            make_test_creature(19),
            make_test_creature(100),
        ];
        let party = Party::new(creatures);

        let active = party.active().unwrap();
        assert_eq!(active.current_hp, 10);
    }

    #[test]
    fn all_fainted_detects_fainted_party() {
        let mut creatures = [
            make_test_creature(10),
            make_test_creature(10),
            make_test_creature(10),
            make_test_creature(10),
            make_test_creature(10),
            make_test_creature(10),
        ];
        // Simulate all creatures fainted
        for c in creatures.iter_mut() {
            c.current_hp = 0;
        }

        let party = Party::new(creatures);

        assert!(party.all_fainted())
    }

    #[test]
    fn swap_swaps_creatures_correctly() {
        let creatures = [
            make_test_creature(10),
            make_test_creature(20),
            make_test_creature(30),
            make_test_creature(40),
            make_test_creature(50),
            make_test_creature(60),
        ];
        let mut party = Party::new(creatures);

        let first_before = party.creatures[0].current_hp;
        let third_before = party.creatures[2].current_hp;

        let swapped = party.swap(0, 2);
        assert!(swapped);

        assert_eq!(party.creatures[0].current_hp, third_before);
        assert_eq!(party.creatures[2].current_hp, first_before);
    }

    #[test]
    fn swap_returns_false_for_out_of_bounds() {
        let creatures = [
            make_test_creature(10),
            make_test_creature(20),
            make_test_creature(30),
            make_test_creature(40),
            make_test_creature(50),
            make_test_creature(60),
        ];
        let mut party = Party::new(creatures);

        let swapped = party.swap(0, 6); // index 6 is out of bounds
        assert!(!swapped);
    }
}
