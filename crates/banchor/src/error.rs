use thiserror::Error;

#[derive(Debug, Error)]
pub enum BanchorError {
    #[error("task description malformed: {0}")]
    MalformedTaskDescription(String),
    #[error("mission anchor unresolved: {0}")]
    MissionAnchorUnresolved(String),
    #[error("unknown task class: {0}")]
    UnknownTaskClass(String),
    #[error("malformed evidence: {0}")]
    MalformedEvidence(String),
    #[error("corpus load failed: {0}")]
    CorpusLoad(String),
    #[error(transparent)]
    Core(#[from] bsuite_core::BsuiteCoreError),
}

impl BanchorError {
    pub fn is_malformed_input(&self) -> bool {
        matches!(
            self,
            Self::MalformedTaskDescription(_)
                | Self::MissionAnchorUnresolved(_)
                | Self::UnknownTaskClass(_)
                | Self::MalformedEvidence(_)
        )
    }

    pub fn into_core(self) -> bsuite_core::BsuiteCoreError {
        match self {
            Self::Core(e) => e,
            Self::CorpusLoad(msg) => bsuite_core::BsuiteCoreError::CorpusDeserializationFailed(msg),
            other => bsuite_core::BsuiteCoreError::PromptResolution(other.to_string()),
        }
    }

    pub fn exit_code(&self) -> bsuite_core::ExitCode {
        match self {
            Self::MalformedTaskDescription(_)
            | Self::MissionAnchorUnresolved(_)
            | Self::UnknownTaskClass(_)
            | Self::MalformedEvidence(_) => bsuite_core::ExitCode::Usage,
            Self::CorpusLoad(_) | Self::Core(_) => bsuite_core::ExitCode::InternalError,
        }
    }
}
