use crate::{
    creature_type::CreatureType,
    experience::{GrowthRate, Level},
    moves::{Move, MoveId, MoveRegistry},
    species::{LearnableMove, Species, SpeciesId, SpeciesName},
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
                types: vec![CreatureType::Grass],
                learnset: vec![
                    LearnableMove {
                        level: Level::new(5).unwrap(),
                        move_id: MoveId(1),
                    },
                    LearnableMove {
                        level: Level::new(10).unwrap(),
                        move_id: MoveId(2),
                    },
                    LearnableMove {
                        level: Level::new(15).unwrap(),
                        move_id: MoveId(3),
                    },
                ],
            }),
        }
    }
}

impl SpeciesRegistry for MockRegistry {
    fn get_species(&self, _species_id: SpeciesId) -> Option<&Species> {
        Some(&self.species)
    }
}

pub struct MockMoveRegistry {
    moves: Vec<Move>,
}
impl MockMoveRegistry {
    pub fn new() -> Self {
        Self {
            moves: vec![
                Move {
                    id: MoveId(1),
                    move_type: CreatureType::Water,
                    power: 80,
                    name: "Water Gun".to_string(),
                    max_pp: 20,
                },
                Move {
                    id: MoveId(2),
                    move_type: CreatureType::Fire,
                    power: 20,
                    name: "Fire Wheel".to_string(),
                    max_pp: 20,
                },
                Move {
                    id: MoveId(3),
                    move_type: CreatureType::Grass,
                    power: 95,
                    name: "Grass Cut".to_string(),
                    max_pp: 20,
                },
            ],
        }
    }
}
impl MoveRegistry for MockMoveRegistry {
    fn get(&self, id: &MoveId) -> Option<&Move> {
        self.moves.iter().find(|m| &m.id == id)
    }
}
