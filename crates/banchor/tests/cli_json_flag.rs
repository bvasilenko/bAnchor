use assert_cmd::Command;
use bsuite_core::ExitCode;
use serde_json::Value;

fn banchor() -> Command {
    Command::cargo_bin("banchor").unwrap()
}

#[test]
fn json_flag_emits_valid_json_envelope_on_stdout() {
    let raw = banchor()
        .args([
            "induct",
            "ship clean change",
            "--task-class",
            "refactor",
            "--json",
        ])
        .assert()
        .code(ExitCode::Finding.as_i32())
        .get_output()
        .stdout
        .clone();

    let output = String::from_utf8(raw).expect("stdout is UTF-8");
    let envelope: Value =
        serde_json::from_str(output.trim()).expect("--json output must be valid JSON");

    assert_eq!(
        envelope["schema_version"].as_u64(),
        Some(1),
        "schema_version"
    );
    assert_eq!(
        envelope["outcome"].as_str(),
        Some("finding"),
        "outcome must be 'finding'"
    );
    let directive = envelope["directive"]
        .as_str()
        .expect("directive field present");
    assert!(!directive.is_empty(), "directive must be non-empty");
}

#[test]
fn json_flag_emits_nothing_on_stderr_for_successful_induct() {
    let output = banchor()
        .args(["induct", "task", "--task-class", "feature", "--json"])
        .assert()
        .code(ExitCode::Finding.as_i32())
        .get_output()
        .clone();

    assert!(
        output.stderr.is_empty(),
        "stderr must be empty on success with --json, got: {}",
        String::from_utf8_lossy(&output.stderr)
    );
}

#[test]
fn json_flag_scope_is_findings_only_errors_remain_on_stderr() {
    let output = banchor()
        .args(["induct", "", "--task-class", "refactor", "--json"])
        .assert()
        .code(ExitCode::Usage.as_i32())
        .get_output()
        .clone();

    assert!(
        output.stdout.is_empty(),
        "usage error must produce no stdout output even with --json"
    );
    assert!(
        !output.stderr.is_empty(),
        "usage error must describe the failure on stderr"
    );
    let stderr_text = String::from_utf8_lossy(&output.stderr);
    assert!(
        serde_json::from_str::<Value>(stderr_text.trim()).is_err(),
        "stderr error output must be plain text, not JSON"
    );
}

#[test]
fn without_json_flag_plain_text_directive_appears_on_stdout() {
    let output = banchor()
        .args(["induct", "task", "--task-class", "bug-fix"])
        .assert()
        .code(ExitCode::Finding.as_i32())
        .get_output()
        .stdout
        .clone();

    let text = String::from_utf8(output).unwrap();
    assert!(
        !text.trim().is_empty(),
        "plain text directive must be non-empty"
    );
    assert!(
        serde_json::from_str::<Value>(text.trim()).is_err(),
        "plain text output must not be JSON"
    );
}
