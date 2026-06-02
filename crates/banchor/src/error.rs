use thiserror::Error;

#[derive(Debug, Error)]
pub enum BanchorError {
    #[error("task description malformed: {0}")]
    MalformedTaskDescription(String),
    #[error("mission anchor unresolved: {0}")]
    MissionAnchorUnresolved(String),
    #[error("unknown task class: {0}")]
    UnknownTaskClass(String),
    #[error("unknown induction state: {0}")]
    UnknownInductionState(String),
    #[error("malformed evidence: {0}")]
    MalformedEvidence(String),
    #[error("not yet implemented: {0}")]
    NotYetImplemented(&'static str),
    #[error(transparent)]
    BsuiteCore(#[from] bsuite_core::BsuiteCoreError),
}

impl BanchorError {
    pub const fn exit_code(&self) -> bsuite_core::ExitCode {
        match self {
            Self::MalformedTaskDescription(_)
            | Self::MissionAnchorUnresolved(_)
            | Self::UnknownTaskClass(_)
            | Self::UnknownInductionState(_)
            | Self::MalformedEvidence(_) => bsuite_core::ExitCode::Usage,
            Self::NotYetImplemented(_) | Self::BsuiteCore(_) => {
                bsuite_core::ExitCode::InternalError
            }
        }
    }
}
