use assert_cmd::Command;
use banchor::TaskClass;
use predicates::prelude::*;

fn banchor() -> Command {
    Command::cargo_bin("banchor").unwrap()
}

fn public_task_class_names() -> Vec<String> {
    TaskClass::ALL
        .into_iter()
        .map(|task_class| task_class.to_string())
        .collect()
}

fn assert_induct_evidence_is_accepted(evidence: &str) {
    banchor()
        .args([
            "induct",
            "ship clean change",
            "--mission",
            "default",
            "--evidence",
            evidence,
            "--reason",
            "bounded skeleton",
        ])
        .assert()
        .code(1)
        .stdout(predicate::str::contains("placeholder directive"));
}

fn assert_induct_evidence_exits_usage(evidence: &str, message: &str) {
    banchor()
        .args(["induct", "task", "--evidence", evidence])
        .assert()
        .code(64)
        .stderr(predicate::str::contains(message));
}

#[test]
fn help_exits_successfully() {
    banchor()
        .arg("--help")
        .assert()
        .success()
        .stdout(predicate::str::contains("banchor"));
}

#[test]
fn missing_subcommand_exits_usage() {
    banchor()
        .assert()
        .code(64)
        .stderr(predicate::str::contains("Usage:"));
}

#[test]
fn unknown_subcommand_exits_usage() {
    banchor()
        .arg("unknown")
        .assert()
        .code(64)
        .stderr(predicate::str::contains("unrecognized subcommand"));
}

#[test]
fn task_classes_lists_every_public_name_once_in_stable_order() {
    let assert = banchor().arg("task-classes").assert().success();
    let output = String::from_utf8(assert.get_output().stdout.clone()).unwrap();
    let lines = output.lines().map(str::to_owned).collect::<Vec<_>>();

    assert_eq!(lines, public_task_class_names());
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
        assert_induct_evidence_exits_usage(evidence, message);
    }
}

#[test]
fn empty_task_description_exits_usage() {
    banchor()
        .args(["induct", "", "--mission", "default"])
        .assert()
        .code(64)
        .stderr(predicate::str::contains("task must not be empty"));
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
        assert_induct_evidence_is_accepted(evidence);
    }
}

#[test]
fn deferred_commands_expose_one_consistent_placeholder_contract() {
    for command in ["update", "init", "tail", "explain"] {
        banchor()
            .arg(command)
            .assert()
            .success()
            .stdout(predicate::str::contains(format!(
                "not yet implemented: {command}"
            )));
    }
}
