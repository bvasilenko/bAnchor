use banchor::InductionState;

const NAMES: [&str; 3] = ["anchored", "unanchored", "malformed"];

#[test]
fn induction_state_display_names_are_stable_and_unique() {
    let names: Vec<&str> = InductionState::ALL.iter().map(|s| s.as_str()).collect();
    assert_eq!(names, NAMES);

    let unique: std::collections::BTreeSet<_> = names.into_iter().collect();
    assert_eq!(unique.len(), InductionState::ALL.len());
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
