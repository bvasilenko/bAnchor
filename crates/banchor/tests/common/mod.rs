#![allow(dead_code)]

const PROHIBITED_PLACEHOLDER_PHRASES: &[&str] = &["[banchor placeholder directive"];

pub fn assert_non_empty_corpus_directive(text: &str, label: &str) {
    assert!(
        !text.trim().is_empty(),
        "{label}: directive must be non-empty"
    );
    for phrase in PROHIBITED_PLACEHOLDER_PHRASES {
        assert!(
            !text.contains(phrase),
            "{label}: output must not contain placeholder phrase: {phrase:?}"
        );
    }
}
