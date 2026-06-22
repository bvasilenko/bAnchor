use banchor::BanchorError;

#[test]
fn malformed_input_variants_map_to_usage_exit_code() {
    let cases = [
        BanchorError::MalformedTaskDescription("empty".to_owned()),
        BanchorError::MissionAnchorUnresolved(String::new()),
        BanchorError::UnknownTaskClass("unknown".to_owned()),
        BanchorError::MalformedEvidence("missing".to_owned()),
    ];

    for error in cases {
        assert_eq!(error.exit_code(), bsuite_core::ExitCode::Usage);
        assert!(error.is_malformed_input());
    }
}

#[test]
fn internal_variants_map_to_internal_error_exit_code() {
    let cases = [
        BanchorError::CorpusLoad("bad toml".to_owned()),
        BanchorError::Core(bsuite_core::BsuiteCoreError::PromptResolution(
            "failed".to_owned(),
        )),
    ];

    for error in cases {
        assert_eq!(error.exit_code(), bsuite_core::ExitCode::InternalError);
        assert!(!error.is_malformed_input());
    }
}
