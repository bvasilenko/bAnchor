use std::io;

use banchor::BanchorError;

#[test]
fn errors_map_to_exit_codes_and_reporting_policy() {
    let cases = [
        (
            BanchorError::MalformedTaskDescription("empty".to_owned()),
            bsuite_core::ExitCode::Usage,
            true,
        ),
        (
            BanchorError::MissionAnchorUnresolved(String::new()),
            bsuite_core::ExitCode::Usage,
            true,
        ),
        (
            BanchorError::UnknownTaskClass("unknown".to_owned()),
            bsuite_core::ExitCode::Usage,
            true,
        ),
        (
            BanchorError::UnknownInductionState("unknown".to_owned()),
            bsuite_core::ExitCode::Usage,
            true,
        ),
        (
            BanchorError::MalformedEvidence("missing".to_owned()),
            bsuite_core::ExitCode::Usage,
            true,
        ),
        (
            BanchorError::NotYetImplemented("command"),
            bsuite_core::ExitCode::InternalError,
            true,
        ),
        (
            BanchorError::BsuiteCore(bsuite_core::BsuiteCoreError::ExitCode("failed".to_owned())),
            bsuite_core::ExitCode::InternalError,
            true,
        ),
        (
            BanchorError::from_stdout_error(io::Error::other("failed")),
            bsuite_core::ExitCode::InternalError,
            true,
        ),
        (
            BanchorError::from_stdout_error(io::Error::from(io::ErrorKind::BrokenPipe)),
            bsuite_core::ExitCode::Success,
            false,
        ),
    ];

    for (error, exit_code, reportable) in cases {
        assert_eq!(error.exit_code(), exit_code);
        assert_eq!(error.is_reportable(), reportable);
    }
}
