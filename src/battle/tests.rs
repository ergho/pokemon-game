#[cfg(test)]
use crate::battle::*;
use crate::creature::Creature;
use crate::party::Party;
use crate::species::SpeciesId;
use crate::stats::{BaseStats, IndividualStats};

/// Helper to create a simple test creature
fn make_test_creature() -> Creature {
    let basestats = BaseStats::new(10, 10, 30, 10).unwrap();

    let individualstats = IndividualStats::from_base(&basestats);
    Creature::new(
        CreatureId::default(),
        SpeciesId(1),
        None,
        1,
        individualstats,
    )
}

/// Sets up a battle with two parties of 6 creatures each
fn setup_battle() -> Battle {
    let creatures_p1 = std::array::from_fn(|_| make_test_creature());
    let creatures_p2 = std::array::from_fn(|_| make_test_creature());

    Battle::new(Party::new(creatures_p1), Party::new(creatures_p2))
}

#[test]
fn state_transitions_follow_expected_order() {
    let mut battle = setup_battle();

    assert_eq!(battle.state, BattleState::StartTurn);
    assert_eq!(battle.current_turn.turn_number, 1);

    battle.advance_state();
    assert_eq!(battle.state, BattleState::SelectActions);

    battle.advance_state();
    assert_eq!(battle.state, BattleState::ResolveActions);

    battle.advance_state();
    assert_eq!(battle.state, BattleState::EndTurn);

    // Transition to next turn
    battle.advance_state();
    assert_eq!(battle.state, BattleState::StartTurn);
    assert_eq!(battle.current_turn.turn_number, 2);
}

#[test]
fn battle_does_not_advance_when_finished() {
    let mut battle = setup_battle();
    battle.state = BattleState::Finished;

    battle.advance_state();
    assert_eq!(battle.state, BattleState::Finished);
    assert_eq!(battle.current_turn.turn_number, 1);
}
