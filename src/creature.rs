use crate::species::SpeciesId;
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
    pub nickname: Option<String>,
    pub level: u8,
    pub individual_stats: IndividualStats,
    pub current_hp: u16, // Plain u16, allows 0 for fainted state
}

impl Creature {
    pub fn new(
        id: CreatureId,
        species_id: SpeciesId,
        nickname: Option<String>,
        level: u8,
        individual_stats: IndividualStats,
    ) -> Self {
        let current_hp = individual_stats.max_hp.get();
        Self {
            id,
            species_id,
            nickname,
            level,
            individual_stats,
            current_hp,
        }
    }

    pub fn name(&self) -> String {
        self.nickname
            .clone()
            .unwrap_or_else(|| format!("Creature#{}", self.id.as_uuid()))
    }

    pub fn modify_hp(&mut self, amount: i16) {
        let hp = self.current_hp as i16 + amount;
        let max_hp = self.individual_stats.max_hp.get();
        self.current_hp = hp.clamp(0, max_hp as i16) as u16;
    }

    pub fn is_fainted(&self) -> bool {
        self.current_hp == 0
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::species::SpeciesId;
    use crate::stats::{IndividualStats, Stat};

    #[test]
    fn creature_creation() {
        let individual_stats = IndividualStats {
            attack: Stat::new(10).unwrap(),
            defense: Stat::new(8).unwrap(),
            max_hp: Stat::new(30).unwrap(),
            speed: Stat::new(12).unwrap(),
        };

        let c = Creature::new(
            CreatureId::new(),
            SpeciesId(1),
            Some("Leafy".to_string()),
            5,
            individual_stats.clone(),
        );

        assert_eq!(c.current_hp, 30);
        assert_eq!(c.level, 5);
        assert!(c.name().contains("Leafy"));
        assert_eq!(c.individual_stats.attack.get(), 10);
    }

    #[test]
    fn hp_modification() {
        let individual_stats = IndividualStats {
            attack: Stat::new(10).unwrap(),
            defense: Stat::new(8).unwrap(),
            max_hp: Stat::new(30).unwrap(),
            speed: Stat::new(12).unwrap(),
        };
        let mut c = Creature::new(
            CreatureId::new(),
            SpeciesId(1),
            None,
            5,
            individual_stats.clone(),
        );

        c.modify_hp(-10);
        assert_eq!(c.current_hp, 20);

        c.modify_hp(5);
        assert_eq!(c.current_hp, 25);

        c.modify_hp(100);
        assert_eq!(c.current_hp, 30);

        c.modify_hp(-100);
        assert_eq!(c.current_hp, 0);
        assert!(c.is_fainted());
    }
}
