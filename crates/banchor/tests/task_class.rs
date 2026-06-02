use std::{collections::BTreeSet, str::FromStr};

use banchor::{BanchorError, TaskClass};
use proptest::prelude::*;

const NAMES: [&str; 11] = [
    "refactor",
    "migration",
    "feature",
    "bug-fix",
    "spike",
    "research",
    "scaffolding",
    "draft",
    "rewrite",
    "localize",
    "brand-conform",
];

proptest! {
    #[test]
    fn task_class_round_trips(index in 0usize..TaskClass::ALL.len()) {
        let task_class = TaskClass::ALL[index];
        let name = task_class.to_string();

        prop_assert_eq!(TaskClass::from_str(&name).unwrap(), task_class);
        prop_assert!(NAMES.contains(&name.as_str()));
    }

    #[test]
    fn unknown_task_class_is_rejected(name in "[a-z0-9-]{1,32}") {
        prop_assume!(!NAMES.contains(&name.as_str()));

        prop_assert!(matches!(
            TaskClass::from_str(&name),
            Err(BanchorError::UnknownTaskClass(_))
        ));
    }
}

#[test]
fn task_class_names_are_unique() {
    let names = TaskClass::ALL
        .into_iter()
        .map(|task_class| task_class.to_string())
        .collect::<BTreeSet<_>>();

    assert_eq!(names.len(), TaskClass::ALL.len());
}
