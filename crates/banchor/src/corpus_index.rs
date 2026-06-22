use crate::{BanchorError, TaskClass};
use bsuite_core::{corpus::parse_signed_corpus, prompt_resolver::DirectiveString};
use ed25519_dalek::VerifyingKey;
use serde::Deserialize;
use std::collections::HashMap;

/// bCore's `CorpusEntry` carries `routing_key`, `directive`, `provenance` and does not use
/// `deny_unknown_fields`, so the `task_class` field is present in the TOML but invisible to
/// bCore's signature computation.  The signature still covers all other fields; the
/// `task_class` discriminator is a banchor-local extension validated separately at construction.
#[derive(Deserialize)]
struct ExtendedCorpusFile {
    entries: Vec<ExtendedCorpusEntry>,
}

#[derive(Deserialize)]
struct ExtendedCorpusEntry {
    task_class: String,
    directive: String,
}

/// Every `TaskClass` variant maps to exactly one directive.
/// Invariant upheld at construction; `resolve` is therefore infallible after construction.
pub struct TaskClassCorpusIndex {
    by_task_class: HashMap<TaskClass, DirectiveString>,
}

impl std::fmt::Debug for TaskClassCorpusIndex {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("TaskClassCorpusIndex")
            .field("entry_count", &self.by_task_class.len())
            .finish()
    }
}

impl TaskClassCorpusIndex {
    pub fn from_toml_signed(
        corpus_toml: &str,
        pubkey: &VerifyingKey,
    ) -> Result<Self, BanchorError> {
        parse_signed_corpus(corpus_toml, pubkey).map_err(BanchorError::Core)?;

        let extended: ExtendedCorpusFile =
            toml::from_str(corpus_toml).map_err(|e| BanchorError::CorpusLoad(e.to_string()))?;

        Self::build_index(extended.entries)
    }

    fn build_index(entries: Vec<ExtendedCorpusEntry>) -> Result<Self, BanchorError> {
        let mut by_task_class: HashMap<TaskClass, DirectiveString> =
            HashMap::with_capacity(TaskClass::ALL.len());

        for entry in entries {
            let task_class = entry.task_class.parse::<TaskClass>().map_err(|_| {
                BanchorError::CorpusLoad(format!(
                    "unrecognised task_class in corpus: {}",
                    entry.task_class
                ))
            })?;

            if by_task_class.contains_key(&task_class) {
                return Err(BanchorError::CorpusLoad(format!(
                    "duplicate task_class in corpus: {}",
                    entry.task_class
                )));
            }

            by_task_class.insert(task_class, DirectiveString::new(entry.directive));
        }

        for variant in TaskClass::ALL {
            if !by_task_class.contains_key(&variant) {
                return Err(BanchorError::CorpusLoad(format!(
                    "corpus missing entry for task_class: {}",
                    variant.as_str()
                )));
            }
        }

        Ok(Self { by_task_class })
    }

    /// Panics only if the construction invariant is violated, which is impossible via the
    /// public API.
    pub fn resolve(&self, task_class: TaskClass) -> &DirectiveString {
        self.by_task_class.get(&task_class).expect(
            "TaskClassCorpusIndex invariant: every TaskClass variant indexed at construction",
        )
    }
}
