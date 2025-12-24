use crate::creature_type::CreatureType;

pub struct Move {
    pub name: String,
    pub move_type: CreatureType,
    pub power: u8,
}

impl Move {
    fn effectiveness_multiplier(&self, defender_types: &[CreatureType]) -> f32 {
        CreatureType::combined_multiplier(self.move_type, defender_types)
    }

    pub fn effective_power(
        &self,
        user_types: &[CreatureType],
        defender_types: &[CreatureType],
    ) -> f32 {
        let mut damage = self.power as f32 * self.effectiveness_multiplier(defender_types);
        if user_types.contains(&self.move_type) {
            damage *= 1.5
        }
        damage
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::creature_type::CreatureType::*;

    #[test]
    fn test_effectiveness_multiplier() {
        let flamethrower = Move {
            name: "Flamethrower".to_string(),
            move_type: CreatureType::Fire,
            power: 90,
        };

        // Single defender type
        assert_eq!(flamethrower.effectiveness_multiplier(&[Grass]), 2.0);

        assert_eq!(flamethrower.effectiveness_multiplier(&[Water]), 0.5);

        assert_eq!(flamethrower.effectiveness_multiplier(&[Fire]), 1.0);

        // Multiple defender types (Grass + Water)
        assert_eq!(
            flamethrower.effectiveness_multiplier(&[Grass, Water]),
            2.0 * 0.5
        );
    }

    #[test]
    fn test_effective_power_without_stab() {
        let flamethrower = Move {
            name: "Flamethrower".to_string(),
            move_type: CreatureType::Fire,
            power: 90,
        };

        // User type does not match move type (no STAB)
        let damage = flamethrower.effective_power(&[CreatureType::Water], &[Grass]);
        // Effectiveness multiplier = 2.0, no STAB
        assert_eq!(damage, 180.0);
    }

    #[test]
    fn test_effective_power_with_stab() {
        let flamethrower = Move {
            name: "Flamethrower".to_string(),
            move_type: CreatureType::Fire,
            power: 90,
        };

        // User type matches move type (STAB applies)
        let damage = flamethrower.effective_power(&[CreatureType::Fire], &[Grass]);
        // Effectiveness multiplier = 2.0, STAB = 1.5 -> total = 270
        assert_eq!(damage, 270.0);
    }

    #[test]
    fn test_effective_power_multiple_defenders_with_stab() {
        let flamethrower = Move {
            name: "Flamethrower".to_string(),
            move_type: CreatureType::Fire,
            power: 90,
        };

        let defenders = [Grass, Water]; // Fire vs Grass = 2, Fire vs Water = 0.5 -> 1.0
        let damage = flamethrower.effective_power(&[CreatureType::Fire], &defenders);
        // STAB = 1.5 -> total = 1.0 * 90 * 1.5 = 135
        assert_eq!(damage, 135.0);
    }
}
