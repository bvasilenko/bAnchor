use serde::{Deserialize, Serialize};
use std::{fmt, str::FromStr};

use crate::BanchorError;

#[derive(Debug, Clone, Copy, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum TaskClass {
    Refactor,
    Migration,
    Feature,
    BugFix,
    Spike,
    Research,
    Scaffolding,
    Draft,
    Rewrite,
    Localize,
    BrandConform,
}

impl TaskClass {
    pub const ALL: [Self; 11] = [
        Self::Refactor,
        Self::Migration,
        Self::Feature,
        Self::BugFix,
        Self::Spike,
        Self::Research,
        Self::Scaffolding,
        Self::Draft,
        Self::Rewrite,
        Self::Localize,
        Self::BrandConform,
    ];

    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Refactor => "refactor",
            Self::Migration => "migration",
            Self::Feature => "feature",
            Self::BugFix => "bug-fix",
            Self::Spike => "spike",
            Self::Research => "research",
            Self::Scaffolding => "scaffolding",
            Self::Draft => "draft",
            Self::Rewrite => "rewrite",
            Self::Localize => "localize",
            Self::BrandConform => "brand-conform",
        }
    }
}

impl fmt::Display for TaskClass {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

impl FromStr for TaskClass {
    type Err = BanchorError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        Self::ALL
            .into_iter()
            .find(|task_class| task_class.as_str() == value)
            .ok_or_else(|| BanchorError::UnknownTaskClass(value.to_owned()))
    }
}
