use crate::{
    experience::GrowthRate,
    species::{Species, SpeciesId, SpeciesName},
    species_registry::SpeciesRegistry,
    stats::BaseStats,
};

pub struct MockRegistry {
    species: Box<Species>,
}
impl MockRegistry {
    pub fn new() -> Self {
        Self {
            species: Box::new(Species {
                id: SpeciesId(1),
                name: SpeciesName::new("Bulby"),
                base_stats: BaseStats::new(50, 50, 50, 50).unwrap(),
                growth_rate: GrowthRate::Fast,
            }),
        }
    }
}

impl SpeciesRegistry for MockRegistry {
    fn get_species(&self, _species_id: SpeciesId) -> Option<&Species> {
        Some(&self.species)
    }
}
