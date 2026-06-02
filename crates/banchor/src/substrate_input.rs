use std::{collections::BTreeMap, fmt, path::PathBuf, str::FromStr};

use crate::BanchorError;

#[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum MissionAnchorRef {
    Path(PathBuf),
    Alias(String),
}

impl MissionAnchorRef {
    pub fn resolve(value: impl AsRef<str>) -> Result<Self, BanchorError> {
        let value = value.as_ref();

        if value.is_empty() {
            return Err(BanchorError::MissionAnchorUnresolved(value.to_owned()));
        }

        if value.starts_with('/') || value.starts_with("./") || value.starts_with("../") {
            Ok(Self::Path(PathBuf::from(value)))
        } else {
            Ok(Self::Alias(value.to_owned()))
        }
    }
}

impl fmt::Display for MissionAnchorRef {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Path(path) => write!(formatter, "{}", path.display()),
            Self::Alias(alias) => formatter.write_str(alias),
        }
    }
}

impl FromStr for MissionAnchorRef {
    type Err = BanchorError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        Self::resolve(value)
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Default)]
pub struct EvidenceMap(BTreeMap<String, String>);

impl EvidenceMap {
    pub fn from_pairs(
        pairs: impl IntoIterator<Item = (String, String)>,
    ) -> Result<Self, BanchorError> {
        let mut evidence = BTreeMap::new();

        for (key, value) in pairs {
            if key.trim().is_empty() {
                return Err(BanchorError::MalformedEvidence(
                    "empty evidence id".to_owned(),
                ));
            }

            evidence.insert(key, value);
        }

        Ok(Self(evidence))
    }

    pub fn as_map(&self) -> &BTreeMap<String, String> {
        &self.0
    }
}
