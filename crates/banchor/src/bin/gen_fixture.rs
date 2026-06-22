use base64::Engine;
use bsuite_core::{CorpusEntry, CorpusFile, ProvenanceRecord, RoutingKey};
use ed25519_dalek::{Signer, SigningKey};
use serde::Serialize;
use std::{fs, path::Path};

/// Fixed seed -- fixture-only material; never used for production signing.
const FIXTURE_SEED: [u8; 32] = [
    0x62, 0x61, 0x6e, 0x63, 0x68, 0x6f, 0x72, 0x2d, 0x66, 0x69, 0x78, 0x74, 0x75, 0x72, 0x65, 0x2d,
    0x76, 0x30, 0x2d, 0x73, 0x65, 0x65, 0x64, 0x2d, 0x30, 0x30, 0x30, 0x30, 0x30, 0x30, 0x30, 0x30,
];

const CANONICAL_KEY_ID: &str = "banchor-fixture-v0";
const OBSERVATION_SOURCE: &str = "hand-authored-fixture-v0";
const RUN_ID: &str = "hand-authored-fixture-v0";

struct EntrySpec {
    task_class: &'static str,
    directive: &'static str,
}

const ENTRIES: &[EntrySpec] = &[
    EntrySpec {
        task_class: "refactor",
        directive: "Before executing this refactor, verify the task is anchored to the stated \
mission. Confirm the mission document names the code area being changed and the goals section \
lists a measurable outcome the refactor serves. Walk the induction ladder: Mission names the \
product outcome; Goals names the engineering milestone; Anchors names the invariants the change \
must preserve; Values names the quality bar; Task names the specific transformation; \
Expected-result names the observable outcome after the change; Action-items names the ordered \
steps. If any rung is absent, name it and stop until the operator supplies it before proceeding.",
    },
    EntrySpec {
        task_class: "migration",
        directive: "Before executing this migration, confirm the mission names the system \
boundary being moved and the goals section states both the source and target states explicitly. \
Anchors must list every contract that must survive the migration unchanged. Walk the induction \
ladder: Mission; Goals (source state, target state); Anchors (preserved contracts); Values \
(correctness over speed); Task (the migration steps); Expected-result (all records in target \
state, source retired); Action-items (ordered, reversible steps with rollback criteria). Name \
any missing rung and stop until it is supplied.",
    },
    EntrySpec {
        task_class: "feature",
        directive: "Before implementing this feature, verify the mission document describes the \
user or system need the feature addresses and the goals section names the acceptance criterion. \
Walk the induction ladder: Mission names who benefits and why; Goals names the measurable \
acceptance criterion; Anchors names the existing contracts the feature must not break; Values \
names the trade-offs the operator accepts; Task names the specific new behaviour; \
Expected-result names what a passing acceptance test would observe; Action-items names the \
ordered implementation steps. Supply any missing rung before proceeding.",
    },
    EntrySpec {
        task_class: "bug-fix",
        directive: "Before fixing this bug, verify the mission names the system invariant the \
bug violates and the goals section names the observable correct behaviour. Walk the induction \
ladder: Mission names the invariant; Goals names the correct behaviour; Anchors names the tests \
or contracts that must remain green; Values names the regression-risk tolerance; Task names the \
specific defect being corrected; Expected-result names the test or observation that proves the \
fix is complete; Action-items names the ordered fix steps. Name any missing rung and stop until \
it is supplied.",
    },
    EntrySpec {
        task_class: "spike",
        directive: "Before executing this spike, confirm the mission names the uncertainty being \
resolved and the goals section states the question the spike must answer. Walk the induction \
ladder: Mission names the risk the spike de-risks; Goals names the specific question with a \
binary answer; Anchors names the time-box and the discard rule if the spike fails; Values names \
the outcome (learning, not shipping); Task names the exploration approach; Expected-result names \
the artefact that captures the answer; Action-items names the ordered exploration steps. Supply \
any missing rung before starting.",
    },
    EntrySpec {
        task_class: "research",
        directive: "Before starting this research task, confirm the mission names the decision \
the research informs and the goals section states the question the research must answer. Walk \
the induction ladder: Mission names the decision; Goals names the research question and the \
output format; Anchors names the sources considered authoritative; Values names the rigour \
standard; Task names the specific investigation; Expected-result names the document or data \
artefact delivered; Action-items names the ordered investigation steps. Name any missing rung \
and stop until it is supplied.",
    },
    EntrySpec {
        task_class: "scaffolding",
        directive: "Before generating this scaffold, verify the mission names the project or \
module the scaffold initialises and the goals section states what a correct scaffold must \
contain. Walk the induction ladder: Mission names the project and its purpose; Goals names the \
completeness criterion for the scaffold; Anchors names the conventions the scaffold must follow; \
Values names the maintainability standard; Task names the scaffold shape; Expected-result names \
the directory tree and files produced; Action-items names the ordered generation steps. Supply \
any missing rung before proceeding.",
    },
    EntrySpec {
        task_class: "draft",
        directive: "Before drafting this content, verify the mission names the audience and the \
communication goal and the goals section states the success criterion for the draft. Walk the \
induction ladder: Mission names the audience and communication goal; Goals names the measurable \
success criterion; Anchors names the brand-voice constraints and any regulated terms; Values \
names the tone and length standards; Task names the specific content to produce; Expected-result \
names the review gate that marks the draft accepted; Action-items names the ordered writing \
steps. Name any missing rung and stop until it is supplied.",
    },
    EntrySpec {
        task_class: "rewrite",
        directive: "Before rewriting this content, verify the mission names why the existing \
version fails and the goals section states what the rewritten version must achieve. Walk the \
induction ladder: Mission names the failure mode of the current version; Goals names the \
improvement criterion; Anchors names the facts and claims from the original that must be \
preserved; Values names the voice and accuracy standard; Task names the specific rewrite scope; \
Expected-result names the review gate that marks the rewrite accepted; Action-items names the \
ordered rewrite steps. Supply any missing rung before proceeding.",
    },
    EntrySpec {
        task_class: "localize",
        directive: "Before localising this content, verify the mission names the target locale \
and the audience and the goals section states the completeness criterion. Walk the induction \
ladder: Mission names the target locale and audience; Goals names the completeness criterion and \
the review gate; Anchors names the approved term glossary and any locale-specific regulatory \
requirements; Values names the translation accuracy and cultural-fit standard; Task names the \
specific content scope; Expected-result names the artefact delivered to the locale reviewer; \
Action-items names the ordered localisation steps. Name any missing rung before starting.",
    },
    EntrySpec {
        task_class: "brand-conform",
        directive: "Before conforming this content to the brand standard, verify the mission \
names the brand guideline version being applied and the goals section states the compliance \
criterion. Walk the induction ladder: Mission names the brand guideline and its authority; Goals \
names the specific compliance criterion; Anchors names the banned terms, required disclosures, \
and mandatory voice patterns; Values names the balance between brand rigour and message clarity; \
Task names the specific content being conformed; Expected-result names the review artefact that \
proves conformance; Action-items names the ordered conformance steps. Supply any missing rung \
before proceeding.",
    },
];

#[derive(Serialize)]
struct FixtureFile {
    schema_version: u32,
    signature: String,
    canonical_key_id: &'static str,
    entries: Vec<FixtureEntry>,
}

#[derive(Serialize)]
struct FixtureEntry {
    routing_key: RoutingKey,
    task_class: &'static str,
    directive: &'static str,
    provenance: FixtureProvenance,
}

#[derive(Serialize)]
struct FixtureProvenance {
    run_id: &'static str,
    iteration: u32,
    observation_source: &'static str,
    pre_compliance: f64,
    post_compliance: f64,
}

fn main() {
    let signing_key = SigningKey::from_bytes(&FIXTURE_SEED);
    let verifying_key = signing_key.verifying_key();

    let core_entries: Vec<CorpusEntry> = ENTRIES
        .iter()
        .map(|spec| CorpusEntry {
            routing_key: RoutingKey::BAnchor,
            directive: spec.directive.to_owned(),
            provenance: ProvenanceRecord {
                run_id: RUN_ID.to_owned(),
                iteration: 0,
                observation_source: OBSERVATION_SOURCE.to_owned(),
                pre_compliance: 0.0,
                post_compliance: 0.0,
            },
        })
        .collect();

    let mut corpus = CorpusFile {
        schema_version: 1,
        signature: String::new(),
        canonical_key_id: CANONICAL_KEY_ID.to_owned(),
        entries: core_entries,
    };

    let payload_bytes = bsuite_core::corpus::canonical_payload_bytes(&corpus)
        .expect("fixture provenance scores are finite; canonicalization must succeed");

    let signature = signing_key.sign(&payload_bytes);
    let sig_b64 = base64::engine::general_purpose::STANDARD.encode(signature.to_bytes());
    corpus.signature = format!("ed25519:{sig_b64}");

    let fixture_entries: Vec<FixtureEntry> = ENTRIES
        .iter()
        .map(|spec| FixtureEntry {
            routing_key: RoutingKey::BAnchor,
            task_class: spec.task_class,
            directive: spec.directive,
            provenance: FixtureProvenance {
                run_id: RUN_ID,
                iteration: 0,
                observation_source: OBSERVATION_SOURCE,
                pre_compliance: 0.0,
                post_compliance: 0.0,
            },
        })
        .collect();

    let fixture_file = FixtureFile {
        schema_version: 1,
        signature: corpus.signature.clone(),
        canonical_key_id: CANONICAL_KEY_ID,
        entries: fixture_entries,
    };

    let header = "# Fixture corpus. Hand-authored seed material until an evolved corpus ships at a later cycle. Not for production trust.\n\n";
    let body = toml::to_string_pretty(&fixture_file).expect("fixture file serialises cleanly");
    let toml_content = format!("{header}{body}");

    let out_dir = Path::new(env!("CARGO_MANIFEST_DIR")).join("corpus");
    fs::create_dir_all(&out_dir).expect("create corpus directory");

    fs::write(out_dir.join("banchor-v0.toml"), toml_content).expect("write corpus TOML");
    fs::write(
        out_dir.join("banchor-v0-pubkey.bin"),
        verifying_key.to_bytes(),
    )
    .expect("write verifying key");
    fs::write(
        out_dir.join("banchor-v0-signkey.bin"),
        signing_key.to_bytes(),
    )
    .expect("write signing key");

    println!("Corpus files written to {}", out_dir.display());
}
