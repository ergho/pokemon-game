use crate::creature_type::CreatureType;
use crate::experience::{GrowthRate, Level};
use crate::moves::MoveId;
use crate::stats::BaseStats;
use std::fmt;

/// Newtype for species name
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct SpeciesName(String);

impl SpeciesName {
    pub fn new(name: &str) -> Self {
        Self(name.to_string())
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for SpeciesName {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

/// Species ID (numeric for now, could be UUID later)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct SpeciesId(pub u64);

#[derive(Debug, Clone, PartialEq)]
pub struct LearnableMove {
    pub level: Level,
    pub move_id: MoveId,
}

impl LearnableMove {
    pub fn new(level: Level, move_id: MoveId) -> Self {
        Self { level, move_id }
    }
}

/// Immutable template describing a species
#[derive(Debug, Clone)]
pub struct Species {
    pub id: SpeciesId,
    pub name: SpeciesName,
    pub base_stats: BaseStats,
    pub growth_rate: GrowthRate,
    pub types: Vec<CreatureType>,
    pub learnset: Vec<LearnableMove>,
}

impl Species {
    pub fn new(
        id: SpeciesId,
        name: SpeciesName,
        base_stats: BaseStats,
        growth_rate: GrowthRate,
        types: Vec<CreatureType>,
        learnset: Vec<LearnableMove>,
    ) -> Self {
        Self {
            id,
            name,
            base_stats,
            growth_rate,
            types,
            learnset,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn species_creation() {
        let stats = BaseStats::new(10, 12, 35, 8).unwrap();
        let ct = vec![CreatureType::Fire];
        let tackle = LearnableMove::new(Level::new(1).unwrap(), MoveId::new(1));
        let learnset = vec![tackle.clone()];
        let species = Species::new(
            SpeciesId(1),
            SpeciesName::new("Bulby"),
            stats,
            GrowthRate::Fast,
            ct,
            learnset,
        );

        assert_eq!(species.id.0, 1);
        assert_eq!(species.name.as_str(), "Bulby");
        assert_eq!(species.base_stats.max_hp(), 35);
        assert_eq!(species.growth_rate, GrowthRate::Fast);
        assert_eq!(species.types, vec![CreatureType::Fire]);
        assert_eq!(species.learnset, vec![tackle])
    }
}
