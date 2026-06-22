mod common;

use assert_cmd::Command;
use banchor::TaskClass;
use bsuite_core::ExitCode;

fn banchor() -> Command {
    Command::cargo_bin("banchor").unwrap()
}

#[test]
fn malformed_induct_routes_error_to_stderr_and_nothing_to_stdout() {
    let cases: &[(&str, &[&str])] = &[
        ("missing-task-class-flag", &["induct", "ship clean change"]),
        (
            "unknown-task-class-value",
            &["induct", "ship clean change", "--task-class", "not-a-class"],
        ),
        (
            "blank-task-description",
            &["induct", "", "--task-class", "refactor"],
        ),
    ];

    for (label, args) in cases {
        let output = banchor()
            .args(*args)
            .assert()
            .failure()
            .get_output()
            .clone();

        assert!(
            output.stdout.is_empty(),
            "[{label}]: malformed input must produce nothing on stdout, got: {:?}",
            String::from_utf8_lossy(&output.stdout)
        );
        assert!(
            !output.stderr.is_empty(),
            "[{label}]: malformed input must describe the rejection on stderr"
        );
    }
}

#[test]
fn induct_routes_directive_to_stdout_and_nothing_to_stderr_for_all_task_classes() {
    for task_class in TaskClass::ALL {
        let output = banchor()
            .args([
                "induct",
                "ship clean change",
                "--task-class",
                task_class.as_str(),
            ])
            .assert()
            .code(ExitCode::Finding.as_i32())
            .get_output()
            .clone();

        let text = String::from_utf8(output.stdout.clone()).unwrap();
        common::assert_non_empty_corpus_directive(&text, &format!("induct {task_class}"));
        assert!(
            output.stderr.is_empty(),
            "induct {task_class}: must not write to stderr on success, got: {:?}",
            String::from_utf8_lossy(&output.stderr)
        );
    }
}

#[test]
fn task_classes_writes_to_stdout_not_stderr() {
    let output = banchor()
        .arg("task-classes")
        .assert()
        .success()
        .get_output()
        .clone();

    assert!(
        !output.stdout.is_empty(),
        "task-classes must write to stdout"
    );
    assert!(
        output.stderr.is_empty(),
        "task-classes must not write to stderr"
    );
}

#[test]
fn deferred_commands_produce_no_stdout_output() {
    for cmd in ["init", "tail", "explain"] {
        let output = banchor()
            .arg(cmd)
            .assert()
            .success()
            .get_output()
            .stdout
            .clone();
        assert!(
            output.is_empty(),
            "{cmd}: deferred command must produce no stdout output"
        );
    }
}
