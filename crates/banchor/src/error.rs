use std::io;

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
    #[error("stdout write failed: {0}")]
    Stdout(io::Error),
    #[error("stdout pipe closed")]
    BrokenStdoutPipe,
    #[error(transparent)]
    BsuiteCore(#[from] bsuite_core::BsuiteCoreError),
}

impl BanchorError {
    pub fn from_stdout_error(error: io::Error) -> Self {
        if error.kind() == io::ErrorKind::BrokenPipe {
            return Self::BrokenStdoutPipe;
        }

        Self::Stdout(error)
    }

    pub const fn is_reportable(&self) -> bool {
        !matches!(self, Self::BrokenStdoutPipe)
    }

    pub const fn exit_code(&self) -> bsuite_core::ExitCode {
        match self {
            Self::MalformedTaskDescription(_)
            | Self::MissionAnchorUnresolved(_)
            | Self::UnknownTaskClass(_)
            | Self::UnknownInductionState(_)
            | Self::MalformedEvidence(_) => bsuite_core::ExitCode::Usage,
            Self::BrokenStdoutPipe => bsuite_core::ExitCode::Success,
            Self::NotYetImplemented(_) | Self::BsuiteCore(_) | Self::Stdout(_) => {
                bsuite_core::ExitCode::InternalError
            }
        }
    }
}
