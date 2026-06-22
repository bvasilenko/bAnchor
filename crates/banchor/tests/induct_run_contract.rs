use banchor::{
    BanchorError, InductArgs, InductionState, TaskClass, corpus_index::TaskClassCorpusIndex,
};
use bsuite_core::HostContext;
use ed25519_dalek::VerifyingKey;

const CORPUS_TOML: &str = include_str!("../corpus/banchor-v0.toml");
const PUBKEY_BYTES: &[u8] = include_bytes!("../corpus/banchor-v0-pubkey.bin");

fn fixture_corpus() -> TaskClassCorpusIndex {
    TaskClassCorpusIndex::from_toml_signed(CORPUS_TOML, &fixture_verifying_key())
        .expect("fixture corpus loads cleanly")
}

fn fixture_verifying_key() -> VerifyingKey {
    let bytes: [u8; 32] = PUBKEY_BYTES.try_into().expect("pubkey is 32 bytes");
    VerifyingKey::from_bytes(&bytes).expect("pubkey is valid")
}

fn minimal_args(task: &str, task_class: TaskClass) -> InductArgs {
    InductArgs {
        task: task.to_owned(),
        task_class,
        mission: None,
        evidence: vec![],
        manifest: None,
        json: false,
        quiet: false,
        reason: None,
    }
}

#[test]
fn verdict_is_unanchored_for_all_task_classes() {
    let corpus = fixture_corpus();
    for task_class in TaskClass::ALL {
        let (_, state) = banchor::induct::run(
            &minimal_args("ship clean change", task_class),
            &corpus,
            HostContext::L2a,
        )
        .expect("valid input must not error");
        assert_eq!(
            state,
            InductionState::Unanchored,
            "expected Unanchored for task_class {task_class}"
        );
    }
}

#[test]
fn directive_forwarded_from_corpus_for_all_task_classes() {
    let corpus = fixture_corpus();
    for task_class in TaskClass::ALL {
        let (directive, _) = banchor::induct::run(
            &minimal_args("any task description", task_class),
            &corpus,
            HostContext::L2a,
        )
        .expect("valid input must not error");
        assert_eq!(
            directive.as_str(),
            corpus.resolve(task_class).as_str(),
            "directive must equal corpus resolution for task_class {task_class}"
        );
    }
}

#[test]
fn verdict_is_independent_of_task_description_content() {
    let corpus = fixture_corpus();
    let descriptions = [
        "a",
        "ship clean change",
        "a longer task description with multiple words and punctuation.",
        "unicode content: cafe with accent",
        "  leading and trailing spaces  ",
        "task-with-dashes_and_underscores",
        "123 numeric prefix",
    ];
    for task in descriptions {
        let (_, state) = banchor::induct::run(
            &minimal_args(task, TaskClass::Refactor),
            &corpus,
            HostContext::L2a,
        )
        .expect("non-blank task must not error");
        assert_eq!(
            state,
            InductionState::Unanchored,
            "verdict must be Unanchored regardless of task description content: {task:?}"
        );
    }
}

#[test]
fn verdict_is_independent_of_host_context() {
    let corpus = fixture_corpus();
    for host_context in HostContext::ALL {
        let (_, state) = banchor::induct::run(
            &minimal_args("any task", TaskClass::Feature),
            &corpus,
            host_context,
        )
        .expect("valid input must not error");
        assert_eq!(
            state,
            InductionState::Unanchored,
            "verdict must be Unanchored for host_context {host_context}"
        );
    }
}

#[test]
fn blank_task_description_is_malformed() {
    let corpus = fixture_corpus();
    let blank_cases = ["", " ", "\t", "\n", "   ", "\t\n\r\n"];
    for task in blank_cases {
        let err = banchor::induct::run(
            &minimal_args(task, TaskClass::Refactor),
            &corpus,
            HostContext::L2a,
        )
        .expect_err("blank task must be rejected");
        assert!(
            matches!(err, BanchorError::MalformedTaskDescription(_)),
            "blank task {task:?} must produce MalformedTaskDescription, got: {err:?}"
        );
        assert!(
            err.is_malformed_input(),
            "blank task {task:?}: is_malformed_input() must be true"
        );
    }
}
