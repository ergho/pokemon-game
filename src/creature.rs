use crate::experience::{GrowthRate, Level};
use crate::species::{Species, SpeciesId};
use crate::species_registry::SpeciesRegistry;
use crate::stats::IndividualStats;
use uuid::Uuid;

/// Globally unique identifier for each persistent creature
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct CreatureId(Uuid);

impl Default for CreatureId {
    fn default() -> Self {
        Self::new()
    }
}

impl CreatureId {
    pub fn new() -> Self {
        CreatureId(Uuid::new_v4())
    }

    pub fn from_uuid(uuid: Uuid) -> Self {
        CreatureId(uuid)
    }

    pub fn as_uuid(&self) -> Uuid {
        self.0
    }
}

/// Persistent creature instance
#[derive(Debug, Clone)]
pub struct Creature {
    pub id: CreatureId,
    pub species_id: SpeciesId,
    pub name: String,
    pub level: Level,
    pub experience: u32,
    pub individual_stats: IndividualStats,
    pub current_hp: u16, // Plain u16, allows 0 for fainted state
}

impl Creature {
    pub fn new(species: &Species, starting_level: u8) -> Option<Self> {
        let id = CreatureId::new();
        let species_id = species.id;
        let name = species.name.clone().to_string();
        let individual_stats = IndividualStats::from_base(&species.base_stats);
        let current_hp = individual_stats.max_hp.get();
        let level = Level::new(starting_level)?;
        let experience = species.growth_rate.exp_for_level(level);

        Some(Self {
            id,
            species_id,
            name,
            level,
            experience,
            individual_stats,
            current_hp,
        })
    }

    pub fn name(&self) -> String {
        self.name.clone()
    }

    pub fn modify_hp(&mut self, amount: i16) {
        let hp = self.current_hp as i16 + amount;
        let max_hp = self.individual_stats.max_hp.get();
        self.current_hp = hp.clamp(0, max_hp as i16) as u16;
    }

    pub fn is_fainted(&self) -> bool {
        self.current_hp == 0
    }

    pub fn gain_exp<R: SpeciesRegistry>(&mut self, amount: u32, registry: &R) {
        self.experience += amount;

        let growth_rate = match registry.get_growth_rate(self.species_id) {
            Some(gr) => gr,
            None => panic!("things are broken with your registry"), // things are wonky if this happens
        };
        while let Some(next_level) = self.level.next() {
            let next_level_exp = growth_rate.exp_for_level(next_level);
            if self.experience >= next_level_exp {
                self.level_up(next_level, registry);
            } else {
                break;
            }
        }
    }

    fn level_up<R: SpeciesRegistry>(&mut self, next_level: Level, _registry: &R) {
        self.on_level_up(next_level);
        self.level = next_level;
    }

    fn on_level_up(&self, _level: Level) {}
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::species::SpeciesId;
    use crate::tests::helpers::MockRegistry;

    fn test_creature(level: u8, registry: &MockRegistry) -> Creature {
        Creature::new(registry.get_species(SpeciesId(1)).unwrap(), level).unwrap()
    }

    #[test]
    fn single_level_up() {
        let registry = MockRegistry::new();
        let growth_rate = registry.get_growth_rate(SpeciesId(1)).unwrap();
        let mut creature = test_creature(5, &registry);

        let target_level = creature.level.next().unwrap();
        let required_exp = growth_rate.exp_for_level(target_level);

        creature.gain_exp(required_exp, &registry);

        assert_eq!(creature.level, Level::new(6).unwrap());
    }

    #[test]
    fn reach_max_level() {
        let registry = MockRegistry::new();
        let growth_rate = registry.get_growth_rate(SpeciesId(1)).unwrap();
        let mut creature = test_creature(95, &registry);
        let exp_needed =
            growth_rate.exp_for_level(Level::new(Level::MAX).unwrap()) - creature.experience;
        creature.gain_exp(exp_needed, &registry);

        assert_eq!(creature.level.get(), Level::MAX);
        assert_eq!(
            creature.experience,
            growth_rate.exp_for_level(Level::new(Level::MAX).unwrap())
        );
    }

    #[test]
    fn multiple_level_ups() {
        let registry = MockRegistry::new();
        let growth_rate = registry.get_growth_rate(SpeciesId(1)).unwrap();
        let mut creature = test_creature(5, &registry);

        let exp_to_gain = growth_rate.exp_for_level(Level::new(8).unwrap()) - creature.experience;
        creature.gain_exp(exp_to_gain, &registry);

        assert_eq!(creature.level, Level::new(8).unwrap());
        assert_eq!(
            creature.experience,
            growth_rate.exp_for_level(Level::new(8).unwrap())
        );
    }

    #[test]
    fn creature_creation() {
        let registry = MockRegistry::new();
        let c = test_creature(5, &registry);

        assert_eq!(c.current_hp, 50);
        assert_eq!(c.level.get(), 5);
        assert!(c.name().contains("Bulby"));
        assert_eq!(c.individual_stats.attack.get(), 50);
    }

    #[test]
    fn hp_modification() {
        let registry = MockRegistry::new();
        let mut c = test_creature(5, &registry);

        c.modify_hp(-10);
        assert_eq!(c.current_hp, 40);

        c.modify_hp(5);
        assert_eq!(c.current_hp, 45);

        c.modify_hp(100);
        assert_eq!(c.current_hp, 50);

        c.modify_hp(-100);
        assert_eq!(c.current_hp, 0);
        assert!(c.is_fainted());
    }
}
