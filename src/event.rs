use crate::creature::CreatureId;

/// Represents a battle event
#[derive(Debug, Clone)]
pub enum BattleEvent {
    Damage {
        source: CreatureId,
        target: CreatureId,
        amount: u16,
    },
    Heal {
        source: CreatureId,
        target: CreatureId,
        amount: u16,
    },
    Fainted {
        creature: CreatureId,
    },
    Miss {
        source: CreatureId,
        target: CreatureId,
    },
    Custom {
        description: String,
    },
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::creature::CreatureId;

    #[test]
    fn create_damage_event() {
        let source = CreatureId::new();
        let target = CreatureId::new();
        let event = BattleEvent::Damage { source, target, amount: 10 };

        match event {
            BattleEvent::Damage { source: s, target: t, amount } => {
                assert_eq!(s, source);
                assert_eq!(t, target);
                assert_eq!(amount, 10);
            }
            _ => panic!("Expected Damage event"),
        }
    }

    #[test]
    fn create_fainted_event() {
        let creature = CreatureId::new();
        let event = BattleEvent::Fainted { creature };

        match event {
            BattleEvent::Fainted { creature: c } => assert_eq!(c, creature),
            _ => panic!("Expected Fainted event"),
        }
    }
}
