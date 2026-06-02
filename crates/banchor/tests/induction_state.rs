use std::str::FromStr;

use banchor::{BanchorError, InductionState};
use proptest::prelude::*;

const NAMES: [&str; 3] = ["anchored", "unanchored", "malformed"];

proptest! {
    #[test]
    fn induction_state_round_trips(index in 0usize..InductionState::ALL.len()) {
        let state = InductionState::ALL[index];
        let name = state.to_string();

        prop_assert_eq!(InductionState::from_str(&name).unwrap(), state);
        prop_assert!(NAMES.contains(&name.as_str()));
    }

    #[test]
    fn unknown_induction_state_is_rejected(name in "[a-z0-9-]{1,32}") {
        prop_assume!(!NAMES.contains(&name.as_str()));

        prop_assert!(matches!(
            InductionState::from_str(&name),
            Err(BanchorError::UnknownInductionState(_))
        ));
    }
}

#[test]
fn induction_state_exit_codes_match_public_contract() {
    assert_eq!(
        bsuite_core::ExitCode::from(InductionState::Anchored).as_i32(),
        0
    );
    assert_eq!(
        bsuite_core::ExitCode::from(InductionState::Unanchored).as_i32(),
        1
    );
    assert_eq!(
        bsuite_core::ExitCode::from(InductionState::Malformed).as_i32(),
        2
    );
}
