use assert_cmd::Command;
use bsuite_core::ExitCode;
use httpmock::MockServer;
use serde_json::Value;
use std::path::{Path, PathBuf};
use tempfile::TempDir;

struct TranscriptCase {
    name: &'static str,
    args: &'static [&'static str],
    expected_exit_code: ExitCode,
    directive_emitted: bool,
}

const TRANSCRIPT_CASES: &[TranscriptCase] = &[
    TranscriptCase {
        name: "induct-finding",
        args: &["induct", "ship clean change", "--task-class", "refactor"],
        expected_exit_code: ExitCode::Finding,
        directive_emitted: true,
    },
    TranscriptCase {
        name: "induct-empty-task",
        args: &["induct", "", "--task-class", "refactor"],
        expected_exit_code: ExitCode::Usage,
        directive_emitted: false,
    },
    TranscriptCase {
        name: "task-classes",
        args: &["task-classes"],
        expected_exit_code: ExitCode::Success,
        directive_emitted: false,
    },
    TranscriptCase {
        name: "init",
        args: &["init"],
        expected_exit_code: ExitCode::Success,
        directive_emitted: false,
    },
    TranscriptCase {
        name: "tail",
        args: &["tail"],
        expected_exit_code: ExitCode::Success,
        directive_emitted: false,
    },
    TranscriptCase {
        name: "explain",
        args: &["explain"],
        expected_exit_code: ExitCode::Success,
        directive_emitted: false,
    },
];

fn banchor_in_dir(args: &[&str], dir: &TempDir) -> assert_cmd::assert::Assert {
    let mut cmd = Command::cargo_bin("banchor").unwrap();
    cmd.env("BSUITE_TRANSCRIPT_DIR", dir.path());
    for arg in args {
        cmd.arg(arg);
    }
    cmd.assert()
}

fn collect_jsonl_files(dir: &TempDir) -> Vec<PathBuf> {
    let sub = dir.path().join("banchor");
    if !sub.exists() {
        return vec![];
    }
    std::fs::read_dir(&sub)
        .into_iter()
        .flatten()
        .filter_map(|e| e.ok())
        .map(|e| e.path())
        .filter(|p| p.extension().and_then(|e| e.to_str()) == Some("jsonl"))
        .collect()
}

fn read_record(path: &Path) -> Value {
    let content = std::fs::read_to_string(path).expect("transcript is readable UTF-8");
    serde_json::from_str(content.trim()).expect("transcript is valid JSON")
}

fn read_single_record(dir: &TempDir) -> Value {
    let files = collect_jsonl_files(dir);
    assert_eq!(
        files.len(),
        1,
        "expected exactly one transcript file, found {files:?}"
    );
    read_record(&files[0])
}

#[test]
fn every_subcommand_writes_exactly_one_transcript_record() {
    for case in TRANSCRIPT_CASES {
        let dir = TempDir::new().unwrap();
        banchor_in_dir(case.args, &dir).code(case.expected_exit_code.as_i32());

        let files = collect_jsonl_files(&dir);
        assert_eq!(
            files.len(),
            1,
            "[{}] expected one transcript file, found {files:?}",
            case.name
        );
    }
}

#[test]
fn transcript_records_carry_correct_exit_code_and_directive_flag() {
    for case in TRANSCRIPT_CASES {
        let dir = TempDir::new().unwrap();
        banchor_in_dir(case.args, &dir).code(case.expected_exit_code.as_i32());

        let record = read_single_record(&dir);

        assert_eq!(
            record["exit_code"].as_u64(),
            Some(case.expected_exit_code.as_i32() as u64),
            "[{}] exit_code",
            case.name
        );
        assert_eq!(
            record["directive_emitted"].as_bool(),
            Some(case.directive_emitted),
            "[{}] directive_emitted",
            case.name
        );
    }
}

#[test]
fn transcript_schema_carries_all_required_fields() {
    let dir = TempDir::new().unwrap();
    banchor_in_dir(
        &["induct", "ship clean change", "--task-class", "refactor"],
        &dir,
    )
    .code(ExitCode::Finding.as_i32());

    let record = read_single_record(&dir);

    assert_eq!(record["schema_version"].as_u64(), Some(1));
    assert_eq!(record["binary_name"].as_str(), Some("banchor"));
    assert!(!record["binary_version"].as_str().unwrap_or("").is_empty());
    assert!(!record["invocation_id"].as_str().unwrap_or("").is_empty());
    assert!(!record["timestamp"].as_str().unwrap_or("").is_empty());
    assert_eq!(record["routing_key"].as_str(), Some("banchor"));
    assert!(record["host_context"].as_str().is_some());
    assert!(record["exit_code"].as_u64().is_some());
    assert!(record["directive_emitted"].as_bool().is_some());
    assert!(record["elapsed_ms"].as_u64().is_some());
    assert_eq!(record["corpus_version"].as_u64(), Some(1));
    assert!(record["additional_fields"].as_object().is_some());
}

#[test]
fn consecutive_invocations_produce_distinct_invocation_ids() {
    let dir = TempDir::new().unwrap();

    for _ in 0..3 {
        banchor_in_dir(&["task-classes"], &dir).success();
    }

    let files = collect_jsonl_files(&dir);
    assert_eq!(files.len(), 3, "three invocations must produce three files");

    let ids: Vec<String> = files
        .iter()
        .map(|p| read_record(p)["invocation_id"].as_str().unwrap().to_owned())
        .collect();

    let unique: std::collections::BTreeSet<_> = ids.iter().collect();
    assert_eq!(
        unique.len(),
        3,
        "invocation_ids must be distinct; got: {ids:?}"
    );
}

#[test]
fn update_command_writes_transcript_record_regardless_of_server_response() {
    let server = MockServer::start();
    server.mock(|when, then| {
        when.path("/manifest.json");
        then.status(200)
            .header("content-type", "application/json")
            .body(r#"{"schema_version":1,"binary_name":"banchor","version":"0.1.0","release_at":"2025-01-01T00:00:00Z","platforms":{},"corpus_version":1,"obfuscation_tier":"none","signing_key_id":"fixture"}"#);
    });

    let dir = TempDir::new().unwrap();
    Command::cargo_bin("banchor")
        .unwrap()
        .env("BSUITE_TRANSCRIPT_DIR", dir.path())
        .env("BSUITE_UPDATE_BASE_URL", server.base_url())
        .arg("update")
        .assert()
        .failure();

    let record = read_single_record(&dir);
    let exit_code = record["exit_code"].as_u64().expect("exit_code present");
    assert_ne!(exit_code, 0, "failed update must record non-zero exit_code");
}
