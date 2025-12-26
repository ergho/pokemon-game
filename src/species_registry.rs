use crate::experience::GrowthRate;
use crate::species::{LearnableMove, Species, SpeciesId};
use crate::stats::BaseStats;

pub trait SpeciesRegistry {
    fn get_species(&self, species_id: SpeciesId) -> Option<&Species>;

    fn get_base_stats(&self, species_id: SpeciesId) -> Option<&BaseStats> {
        self.get_species(species_id).map(|s| &s.base_stats)
    }

    fn get_growth_rate(&self, species_id: SpeciesId) -> Option<&GrowthRate> {
        self.get_species(species_id).map(|s| &s.growth_rate)
    }

    fn get_learnset(&self, species_id: SpeciesId) -> Option<&[LearnableMove]> {
        self.get_species(species_id).map(|s| s.learnset.as_slice())
    }
}
