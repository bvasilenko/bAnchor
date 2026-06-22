use assert_cmd::Command;
use banchor::TaskClass;
use bsuite_core::ExitCode;
use predicates::prelude::*;

fn banchor() -> Command {
    Command::cargo_bin("banchor").unwrap()
}

fn assert_induct_evidence_accepted(evidence: &str) {
    banchor()
        .args([
            "induct",
            "ship clean change",
            "--task-class",
            "refactor",
            "--evidence",
            evidence,
        ])
        .assert()
        .code(ExitCode::Finding.as_i32());
}

fn assert_induct_evidence_clap_error(evidence: &str, message: &str) {
    banchor()
        .args([
            "induct",
            "task",
            "--task-class",
            "refactor",
            "--evidence",
            evidence,
        ])
        .assert()
        .failure()
        .stderr(predicate::str::contains(message));
}

#[test]
fn help_exits_successfully_and_names_all_subcommands() {
    let output = banchor()
        .arg("--help")
        .assert()
        .success()
        .get_output()
        .stdout
        .clone();
    let text = String::from_utf8(output).unwrap();

    for token in [
        "induct",
        "task-classes",
        "update",
        "init",
        "tail",
        "explain",
    ] {
        assert!(text.contains(token), "help omitted subcommand: {token}");
    }
}

#[test]
fn missing_subcommand_exits_with_error() {
    banchor()
        .assert()
        .failure()
        .stderr(predicate::str::contains("Usage:"));
}

#[test]
fn unknown_subcommand_exits_with_error() {
    banchor()
        .arg("unknown")
        .assert()
        .failure()
        .stderr(predicate::str::contains("unrecognized subcommand"));
}

#[test]
fn task_classes_lists_every_name_exactly_once_in_stable_order() {
    let assert = banchor().arg("task-classes").assert().success();
    let output = String::from_utf8(assert.get_output().stdout.clone()).unwrap();
    let lines: Vec<&str> = output.lines().collect();

    let expected: Vec<&str> = TaskClass::ALL.iter().map(|tc| tc.as_str()).collect();
    assert_eq!(lines, expected);
}

#[test]
fn evidence_arguments_reject_unusable_ids_and_shapes() {
    let cases = [
        ("missing-separator", "expected evidence in id=value form"),
        ("", "expected evidence in id=value form"),
        ("=value", "evidence id must not be empty"),
        (" \t =value", "evidence id must not be empty"),
    ];

    for (evidence, message) in cases {
        assert_induct_evidence_clap_error(evidence, message);
    }
}

#[test]
fn evidence_arguments_accept_any_non_empty_id_and_raw_value() {
    for evidence in [
        "empty=",
        "trace=a=b",
        "kebab-id=value",
        "id=value with spaces",
        "id= value with leading space",
    ] {
        assert_induct_evidence_accepted(evidence);
    }
}
