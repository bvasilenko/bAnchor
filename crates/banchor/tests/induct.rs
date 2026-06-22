mod common;

use assert_cmd::Command;
use banchor::TaskClass;
use bsuite_core::ExitCode;

fn banchor() -> Command {
    Command::cargo_bin("banchor").unwrap()
}

fn induct_args_for(task: &str, task_class: TaskClass) -> Vec<String> {
    let mut args = vec!["induct".to_owned()];
    if task.starts_with('-') {
        args.push("--".to_owned());
    }
    args.push(task.to_owned());
    args.push("--task-class".to_owned());
    args.push(task_class.as_str().to_owned());
    args
}

#[test]
fn every_task_class_emits_a_non_empty_corpus_directive_on_stdout() {
    for task_class in TaskClass::ALL {
        let output = banchor()
            .args(induct_args_for("ship clean change", task_class))
            .assert()
            .code(ExitCode::Finding.as_i32())
            .get_output()
            .stdout
            .clone();

        let text = String::from_utf8(output).unwrap();
        common::assert_non_empty_corpus_directive(&text, task_class.as_str());
    }
}

#[test]
fn all_task_classes_emit_distinct_directives() {
    let directives: Vec<String> = TaskClass::ALL
        .iter()
        .map(|tc| {
            let output = banchor()
                .args(induct_args_for("ship clean change", *tc))
                .assert()
                .code(ExitCode::Finding.as_i32())
                .get_output()
                .stdout
                .clone();
            String::from_utf8(output).unwrap().trim().to_owned()
        })
        .collect();

    let unique: std::collections::BTreeSet<_> = directives.iter().collect();
    assert_eq!(
        unique.len(),
        TaskClass::ALL.len(),
        "every task class must produce a distinct directive"
    );
}

#[test]
fn empty_task_description_exits_usage() {
    banchor()
        .args([
            "induct",
            "",
            "--task-class",
            "refactor",
            "--mission",
            "default",
        ])
        .assert()
        .code(ExitCode::Usage.as_i32());
}

#[test]
fn missing_task_class_flag_exits_with_error() {
    banchor()
        .args(["induct", "ship clean change"])
        .assert()
        .failure();
}

#[test]
fn unknown_task_class_value_exits_with_error() {
    banchor()
        .args(["induct", "ship clean change", "--task-class", "unknown"])
        .assert()
        .failure();
}

#[test]
fn task_description_starting_with_hyphen_is_accepted_via_separator() {
    banchor()
        .args(["induct", "--task-class", "spike", "--", "-hyphen-task"])
        .assert()
        .code(ExitCode::Finding.as_i32());
}

/// Flag handling is task-class-agnostic; a single canonical class covers all flag paths.
#[test]
fn optional_flags_do_not_suppress_stdout_directive() {
    let tc = TaskClass::ALL[0].as_str();

    let cases: &[&[&str]] = &[
        &["induct", "task", "--task-class", tc, "--quiet"],
        &["induct", "task", "--task-class", tc, "--mission", "default"],
        &[
            "induct",
            "task",
            "--task-class",
            tc,
            "--evidence",
            "k=v",
            "--reason",
            "proof",
        ],
    ];

    for args in cases {
        let output = banchor()
            .args(*args)
            .assert()
            .code(ExitCode::Finding.as_i32())
            .get_output()
            .stdout
            .clone();
        assert!(!output.is_empty(), "stdout must carry the directive");
    }
}
