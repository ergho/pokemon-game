#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum CreatureType {
    Normal,
    Fire,
    Water,
    Grass,
    Electric,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Effectiveness {
    Immune,
    Resistant,
    Normal,
    Super,
}

impl Effectiveness {
    pub fn multiplier(self) -> f32 {
        match self {
            Effectiveness::Immune => 0.0,
            Effectiveness::Resistant => 0.5,
            Effectiveness::Normal => 1.0,
            Effectiveness::Super => 2.0,
        }
    }
}

impl CreatureType {
    const COUNT: usize = 5;
    const TYPE_CHART: [[u8; Self::COUNT]; Self::COUNT] = [
        [2, 2, 2, 2, 2],
        [2, 2, 1, 4, 2],
        [2, 4, 2, 1, 2],
        [2, 1, 4, 2, 2],
        [2, 2, 2, 2, 2],
    ];

    pub fn effectiveness(attacker: CreatureType, defender: CreatureType) -> Effectiveness {
        match Self::TYPE_CHART[attacker as usize][defender as usize] {
            0 => Effectiveness::Immune,
            1 => Effectiveness::Resistant,
            2 => Effectiveness::Normal,
            4 => Effectiveness::Super,
            _ => unreachable!(),
        }
    }

    pub fn combined_multiplier(attacker: CreatureType, defender: &[CreatureType]) -> f32 {
        defender
            .iter()
            .map(|&d| Self::effectiveness(attacker, d).multiplier())
            .product()
    }
}

#[cfg(test)]
mod tests {
    use crate::creature_type::{CreatureType, CreatureType::*, Effectiveness};

    #[test]
    fn test_single_effectiveness() {
        // Normal vs Fire -> Normal
        assert_eq!(
            CreatureType::effectiveness(Normal, Fire),
            Effectiveness::Normal
        );

        // Fire vs Grass -> Super
        assert_eq!(
            CreatureType::effectiveness(Fire, Grass),
            Effectiveness::Super
        );

        // Fire vs Water -> Resistant
        assert_eq!(
            CreatureType::effectiveness(Fire, Water),
            Effectiveness::Resistant
        );

        // Water vs Grass -> Resistant
        assert_eq!(
            CreatureType::effectiveness(Water, Grass),
            Effectiveness::Resistant
        );

        // Water vs Fire -> Super
        assert_eq!(
            CreatureType::effectiveness(Water, Fire),
            Effectiveness::Super
        );

        // Grass vs Fire -> Resistant
        assert_eq!(
            CreatureType::effectiveness(Grass, Fire),
            Effectiveness::Resistant
        );

        // Grass vs Water -> Super
        assert_eq!(
            CreatureType::effectiveness(Grass, Water),
            Effectiveness::Super
        );
    }

    #[test]
    fn test_combined_multiplier() {
        // Fire attacking Grass + Water
        let defenders = [Grass, Water];
        // Fire vs Grass = 2.0, Fire vs Water = 0.5 -> product = 1.0
        assert_eq!(CreatureType::combined_multiplier(Fire, &defenders), 1.0);

        // Water attacking Fire + Grass
        let defenders = [Fire, Grass];
        // Water vs Fire = 2.0, Water vs Grass = 0.5 -> product = 1.0
        assert_eq!(CreatureType::combined_multiplier(Water, &defenders), 1.0);

        // Grass attacking Water + Fire
        let defenders = [Water, Fire];
        // Grass vs Water = 2.0, Grass vs Fire = 0.5 -> product = 1.0
        assert_eq!(CreatureType::combined_multiplier(Grass, &defenders), 1.0);
    }
}
