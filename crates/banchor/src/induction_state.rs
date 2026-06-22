use serde::{Deserialize, Serialize};
use std::fmt;

/// Verdict class for the induction ladder evaluation.
/// At the fixture-corpus tier only `Unanchored` is emitted at runtime;
/// `Anchored` becomes reachable once a real evaluation engine ships.
#[derive(Debug, Clone, Copy, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum InductionState {
    Anchored,
    Unanchored,
    Malformed,
}

impl InductionState {
    pub const ALL: [Self; 3] = [Self::Anchored, Self::Unanchored, Self::Malformed];

    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Anchored => "anchored",
            Self::Unanchored => "unanchored",
            Self::Malformed => "malformed",
        }
    }
}

impl fmt::Display for InductionState {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

impl From<InductionState> for bsuite_core::ExitCode {
    fn from(value: InductionState) -> Self {
        match value {
            InductionState::Anchored => Self::Success,
            InductionState::Unanchored => Self::Finding,
            InductionState::Malformed => Self::InternalError,
        }
    }
}
