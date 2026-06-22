use assert_cmd::Command;
use httpmock::MockServer;

fn banchor() -> Command {
    Command::cargo_bin("banchor").unwrap()
}

#[test]
fn update_subcommand_fails_when_manifest_signature_verification_fails() {
    let server = MockServer::start();

    let manifest_json = serde_json::json!({
        "schema_version": 1,
        "binary_name": "banchor",
        "version": "0.1.0",
        "release_at": "2025-01-01T00:00:00Z",
        "platforms": {
            "linux-x86_64": { "archive_url": "https://example.com/banchor.tar", "sha256": "abc" }
        },
        "corpus_version": 1,
        "obfuscation_tier": "none",
        "signing_key_id": "fixture-key"
    });

    server.mock(|when, then| {
        when.path("/manifest.json");
        then.status(200)
            .header("content-type", "application/json")
            .body(serde_json::to_string(&manifest_json).unwrap());
    });
    server.mock(|when, then| {
        when.path("/manifest.json.sig");
        then.status(200).body("ed25519:AAAA");
    });

    banchor()
        .arg("update")
        .env("BSUITE_UPDATE_BASE_URL", server.base_url())
        .assert()
        .failure();
}

#[test]
fn update_subcommand_writes_to_stderr_not_stdout_on_all_outcomes() {
    let server = MockServer::start();

    server.mock(|when, then| {
        when.path("/manifest.json");
        then.status(404);
    });

    let output = banchor()
        .arg("update")
        .env("BSUITE_UPDATE_BASE_URL", server.base_url())
        .assert()
        .failure()
        .get_output()
        .clone();

    assert!(
        output.stdout.is_empty(),
        "update must not write to stdout; got: {}",
        String::from_utf8_lossy(&output.stdout)
    );
    assert!(
        !output.stderr.is_empty(),
        "update must describe the failure on stderr"
    );
}
