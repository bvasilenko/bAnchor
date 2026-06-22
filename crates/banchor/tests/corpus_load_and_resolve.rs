use banchor::{TaskClass, corpus_index::TaskClassCorpusIndex};
use base64::Engine;
use bsuite_core::{BsuiteCoreError, CorpusEntry, CorpusFile, ProvenanceRecord, RoutingKey};
use ed25519_dalek::{Signer, SigningKey, VerifyingKey};
use serde::Serialize;

const CORPUS_TOML: &str = include_str!("../corpus/banchor-v0.toml");
const PUBKEY_BYTES: &[u8] = include_bytes!("../corpus/banchor-v0-pubkey.bin");
const SIGNKEY_BYTES: &[u8] = include_bytes!("../corpus/banchor-v0-signkey.bin");

fn load_verifying_key() -> VerifyingKey {
    let bytes: [u8; 32] = PUBKEY_BYTES.try_into().expect("pubkey is 32 bytes");
    VerifyingKey::from_bytes(&bytes).expect("pubkey is valid")
}

fn load_signing_key() -> SigningKey {
    let bytes: [u8; 32] = SIGNKEY_BYTES.try_into().expect("signkey is 32 bytes");
    SigningKey::from_bytes(&bytes)
}

fn load_corpus() -> TaskClassCorpusIndex {
    TaskClassCorpusIndex::from_toml_signed(CORPUS_TOML, &load_verifying_key())
        .expect("fixture corpus loads cleanly")
}

#[test]
fn corpus_signature_is_valid_against_embedded_pubkey() {
    let _corpus = load_corpus();
}

#[test]
fn signing_key_and_verifying_key_form_a_consistent_pair() {
    use ed25519_dalek::Signer;
    let signing_key = load_signing_key();
    assert_eq!(signing_key.verifying_key(), load_verifying_key());
    let sig = signing_key.sign(b"test");
    signing_key
        .verifying_key()
        .verify_strict(b"test", &sig)
        .unwrap();
}

#[test]
fn all_eleven_task_classes_are_indexed() {
    let corpus = load_corpus();

    for task_class in TaskClass::ALL {
        let directive = corpus.resolve(task_class);
        assert!(
            !directive.as_str().is_empty(),
            "empty directive for task_class: {task_class}"
        );
    }
}

#[test]
fn every_task_class_directive_is_distinct() {
    let corpus = load_corpus();

    let directives: Vec<&str> = TaskClass::ALL
        .iter()
        .map(|tc| corpus.resolve(*tc).as_str())
        .collect();

    let unique: std::collections::BTreeSet<_> = directives.iter().copied().collect();
    assert_eq!(
        unique.len(),
        TaskClass::ALL.len(),
        "every task class must have a distinct directive"
    );
}

#[test]
fn tampered_signature_is_rejected() {
    let tampered = CORPUS_TOML.replacen(
        "ed25519:",
        "ed25519:AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA",
        1,
    );
    let result = TaskClassCorpusIndex::from_toml_signed(&tampered, &load_verifying_key());
    assert!(
        matches!(
            result,
            Err(banchor::BanchorError::Core(
                BsuiteCoreError::CorpusSignatureInvalid
                    | BsuiteCoreError::CorpusDeserializationFailed(_)
            ))
        ),
        "tampered signature must be rejected, got: {result:?}"
    );
}

#[test]
fn corpus_missing_a_task_class_variant_is_rejected() {
    #[derive(Serialize)]
    struct ExtEntry {
        routing_key: &'static str,
        task_class: &'static str,
        directive: String,
        provenance: ExtProvenance,
    }

    #[derive(Serialize)]
    struct ExtProvenance {
        run_id: &'static str,
        iteration: u32,
        observation_source: &'static str,
        pre_compliance: f64,
        post_compliance: f64,
    }

    #[derive(Serialize)]
    struct ExtCorpusFile {
        schema_version: u32,
        signature: String,
        canonical_key_id: &'static str,
        entries: Vec<ExtEntry>,
    }

    let signing_key = load_signing_key();
    let verifying_key = load_verifying_key();

    // All variants except the first (Refactor): one missing entry triggers the gap check.
    let included: Vec<TaskClass> = TaskClass::ALL
        .iter()
        .copied()
        .filter(|&tc| tc != TaskClass::Refactor)
        .collect();

    let core_entries: Vec<CorpusEntry> = included
        .iter()
        .map(|tc| CorpusEntry {
            routing_key: RoutingKey::BAnchor,
            directive: format!("test directive for {}", tc.as_str()),
            provenance: ProvenanceRecord {
                run_id: "test".to_owned(),
                iteration: 0,
                observation_source: "test".to_owned(),
                pre_compliance: 0.0,
                post_compliance: 0.0,
            },
        })
        .collect();

    let mut core_corpus = CorpusFile {
        schema_version: 1,
        signature: String::new(),
        canonical_key_id: "banchor-fixture-v0".to_owned(),
        entries: core_entries,
    };

    let payload = bsuite_core::corpus::canonical_payload_bytes(&core_corpus)
        .expect("10-entry corpus payload");
    let sig = signing_key.sign(&payload);
    let sig_b64 = base64::engine::general_purpose::STANDARD.encode(sig.to_bytes());
    core_corpus.signature = format!("ed25519:{sig_b64}");

    let ext_entries: Vec<ExtEntry> = included
        .iter()
        .map(|tc| ExtEntry {
            routing_key: "banchor",
            task_class: tc.as_str(),
            directive: format!("test directive for {}", tc.as_str()),
            provenance: ExtProvenance {
                run_id: "test",
                iteration: 0,
                observation_source: "test",
                pre_compliance: 0.0,
                post_compliance: 0.0,
            },
        })
        .collect();

    let ext_file = ExtCorpusFile {
        schema_version: 1,
        signature: core_corpus.signature.clone(),
        canonical_key_id: "banchor-fixture-v0",
        entries: ext_entries,
    };

    let corpus_toml = toml::to_string_pretty(&ext_file).expect("serialises cleanly");

    let result = TaskClassCorpusIndex::from_toml_signed(&corpus_toml, &verifying_key);
    assert!(
        matches!(result, Err(banchor::BanchorError::CorpusLoad(_))),
        "corpus missing a task class variant must be rejected with CorpusLoad, got: {result:?}"
    );
}

#[test]
fn corpus_with_duplicate_task_class_is_rejected() {
    let extra_entry = r#"
[[entries]]
routing_key = "banchor"
task_class = "refactor"
directive = "duplicate"

[entries.provenance]
run_id = "x"
iteration = 0
observation_source = "x"
pre_compliance = 0.0
post_compliance = 0.0
"#;

    let fabricated = format!("{CORPUS_TOML}{extra_entry}");
    let result = TaskClassCorpusIndex::from_toml_signed(&fabricated, &load_verifying_key());
    assert!(
        result.is_err(),
        "duplicate task_class must be rejected (signature or index validation)"
    );
}
