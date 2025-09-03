use crate::creature::{Creature, CreatureId};
use crate::event::BattleEvent;
use crate::event_queue::EventQueue;

#[derive(Debug, Clone)]
pub enum TurnAction {
    Attack { target: CreatureId, power: u16 },
    Heal { target: CreatureId, amount: u16 },
    Pass,
}
#[derive(Debug)]
pub struct Battle {
    pub creature_one: Creature,
    pub creature_two: Creature,
    pub queue: EventQueue,
}

impl Battle {
    /// Initialize a new battle with two creatures
    pub fn new(creature_one: Creature, creature_two: Creature) -> Self {
        Self {
            creature_one,
            creature_two,
            queue: EventQueue::new(),
        }
    }

    /// Get a mutable reference to a creature by ID
    fn get_mut_creature(&mut self, id: &CreatureId) -> Option<&mut Creature> {
        if &self.creature_one.id == id {
            Some(&mut self.creature_one)
        } else if &self.creature_two.id == id {
            Some(&mut self.creature_two)
        } else {
            None
        }
    }

    /// Perform an attack from one creature to another
    pub fn attack(&mut self, attacker: CreatureId, target: CreatureId, damage: u16) {
        // Push a Damage event
        self.queue.push(BattleEvent::Damage {
            source: attacker,
            target,
            amount: damage,
        });
    }

    /// Process all events in the queue
    pub fn process_events(&mut self) {
        while let Some(event) = self.queue.pop() {
            match event {
                BattleEvent::Damage {
                    source,
                    target,
                    amount,
                } => {
                    if let Some(target_creature) = self.get_mut_creature(&target) {
                        target_creature.modify_hp(-(amount as i16));
                        if target_creature.is_fainted() {
                            self.queue.push(BattleEvent::Fainted { creature: target });
                        }
                    }
                }
                BattleEvent::Fainted { creature } => {
                    println!("Creature {:?} fainted!", creature.as_uuid());
                }
                _ => {
                    println!("Unhandled event: {:?}", event);
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::species::SpeciesId;
    use crate::stats::{IndividualStats, Stat};

    fn create_test_creature() -> Creature {
        Creature::new(
            CreatureId::new(),
            SpeciesId(1),
            Some("TestMon".to_string()),
            5,
            IndividualStats {
                attack: Stat::new(10).unwrap(),
                defense: Stat::new(8).unwrap(),
                max_hp: Stat::new(30).unwrap(),
                speed: Stat::new(12).unwrap(),
            },
        )
    }

    #[test]
    fn basic_attack_and_fainting() {
        let mut battle = Battle::new(create_test_creature(), create_test_creature());

        let attacker_id = battle.creature_one.id;
        let target_id = battle.creature_two.id;

        // Attack with enough damage to faint
        battle.attack(attacker_id, target_id, 30);
        battle.process_events();

        assert!(battle.creature_two.is_fainted());
    }

    #[test]
    fn basic_attack_partial_damage() {
        let mut battle = Battle::new(create_test_creature(), create_test_creature());

        let attacker_id = battle.creature_one.id;
        let target_id = battle.creature_two.id;

        // Attack with partial damage
        battle.attack(attacker_id, target_id, 10);
        battle.process_events();

        assert_eq!(battle.creature_two.current_hp, 20);
        assert!(!battle.creature_two.is_fainted());
    }
}
