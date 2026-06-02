use banchor::BanchorError;

#[test]
fn user_input_errors_map_to_usage() {
    let errors = [
        BanchorError::MalformedTaskDescription("empty".to_owned()),
        BanchorError::MissionAnchorUnresolved("".to_owned()),
        BanchorError::UnknownTaskClass("unknown".to_owned()),
        BanchorError::UnknownInductionState("unknown".to_owned()),
        BanchorError::MalformedEvidence("missing".to_owned()),
    ];

    for error in errors {
        assert_eq!(error.exit_code(), bsuite_core::ExitCode::Usage);
    }
}

#[test]
fn non_user_input_errors_map_to_internal_error() {
    let errors = [
        BanchorError::NotYetImplemented("command"),
        BanchorError::BsuiteCore(bsuite_core::BsuiteCoreError::ExitCode("failed".to_owned())),
    ];

    for error in errors {
        assert_eq!(error.exit_code(), bsuite_core::ExitCode::InternalError);
    }
}
