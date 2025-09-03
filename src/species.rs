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

/// Immutable template describing a species
#[derive(Debug, Clone)]
pub struct Species {
    pub id: SpeciesId,
    pub name: SpeciesName,
    pub base_stats: BaseStats,
}

impl Species {
    pub fn new(id: SpeciesId, name: SpeciesName, base_stats: BaseStats) -> Self {
        Self {
            id,
            name,
            base_stats,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::stats::Stat;

    #[test]
    fn species_creation() {
        let stats = BaseStats::new(
            Stat::new(10).unwrap(),
            Stat::new(12).unwrap(),
            Stat::new(35).unwrap(),
            Stat::new(8).unwrap(),
        );
        let species = Species::new(SpeciesId(1), SpeciesName::new("Bulby"), stats);

        assert_eq!(species.id.0, 1);
        assert_eq!(species.name.as_str(), "Bulby");
        assert_eq!(species.base_stats.max_hp(), 35);
    }
}
