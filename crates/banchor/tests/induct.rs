use assert_cmd::Command;
use predicates::prelude::*;
use proptest::prelude::*;

const DIRECTIVE_HEADER: &str = "[banchor placeholder directive - pre-corpus output]";
const PLACEHOLDER_ROUTE: &str = "Routing key: TaskClass::Draft placeholder route.";
const PLACEHOLDER_STATE: &str = "Induction-state: Unanchored.";
const ACTION_START: &str = "ACTION: This invocation reached banchor at the pre-corpus phase";
const EXIT_FOOTER: &str = "Exit code carries the verdict-class signal.";
const PARSED_TASK_PREFIX: &str = "Parsed task: ";
const ROUTE_SEPARATOR: &str = ". Routing key: ";
const MISSION_REF_PREFIX: &str = ". Mission ref: ";
const STATE_SEPARATOR: &str = ". Induction-state: ";

fn banchor() -> Command {
    Command::cargo_bin("banchor").unwrap()
}

fn directive_stdout(args: &[&str]) -> String {
    let assert = banchor()
        .args(args)
        .assert()
        .code(1)
        .stdout(predicate::str::contains(DIRECTIVE_HEADER))
        .stdout(predicate::str::contains(PLACEHOLDER_ROUTE))
        .stdout(predicate::str::contains(PLACEHOLDER_STATE))
        .stdout(predicate::str::contains(ACTION_START))
        .stdout(predicate::str::contains(EXIT_FOOTER));

    String::from_utf8(assert.get_output().stdout.clone()).unwrap()
}

fn assert_directive_fields(args: &[&str], task: &str, mission_ref: &str) {
    let stdout = directive_stdout(args);
    let fields = DirectiveFields::parse(&stdout);

    assert_eq!(fields.task, task);
    assert_eq!(fields.mission_ref, mission_ref);
    assert_eq!(fields.task_class, "TaskClass::Draft placeholder route");
    assert_eq!(fields.induction_state, "Unanchored");
}

#[derive(Debug, PartialEq, Eq)]
struct DirectiveFields {
    task: String,
    task_class: String,
    mission_ref: String,
    induction_state: String,
}

impl DirectiveFields {
    fn parse(stdout: &str) -> Self {
        let parsed_line = stdout
            .lines()
            .find(|line| line.starts_with(PARSED_TASK_PREFIX))
            .unwrap();

        let (task, after_task) = parsed_line
            .strip_prefix(PARSED_TASK_PREFIX)
            .unwrap()
            .split_once(ROUTE_SEPARATOR)
            .unwrap();
        let (task_class, after_route) = after_task.split_once(MISSION_REF_PREFIX).unwrap();
        let (mission_ref, induction_state) = after_route.split_once(STATE_SEPARATOR).unwrap();

        Self {
            task: decode_field_value(task),
            task_class: task_class.to_owned(),
            mission_ref: decode_field_value(mission_ref),
            induction_state: induction_state.trim_end_matches('.').to_owned(),
        }
    }
}

fn decode_field_value(value: &str) -> String {
    let mut decoded = String::new();
    let mut characters = value.chars().peekable();

    while let Some(character) = characters.next() {
        if character != '\\' {
            decoded.push(character);
            continue;
        }

        match characters.next() {
            Some('n') => decoded.push('\n'),
            Some('r') => decoded.push('\r'),
            Some('t') => decoded.push('\t'),
            Some('\\') => decoded.push('\\'),
            Some('u') => decoded.push(decode_unicode_escape(&mut characters)),
            Some(other) => {
                decoded.push('\\');
                decoded.push(other);
            }
            None => decoded.push('\\'),
        }
    }

    decoded
}

fn decode_unicode_escape(characters: &mut std::iter::Peekable<impl Iterator<Item = char>>) -> char {
    assert_eq!(characters.next(), Some('{'));

    let mut hex = String::new();
    for character in characters.by_ref() {
        if character == '}' {
            break;
        }

        hex.push(character);
    }

    let codepoint = u32::from_str_radix(&hex, 16).unwrap();
    char::from_u32(codepoint).unwrap()
}

fn cli_args_for_task(task: &str) -> Vec<&str> {
    let mut args = vec!["induct"];
    if task.starts_with('-') {
        args.push("--");
    }
    args.push(task);
    args
}

fn accepted_task_text() -> impl Strategy<Value = String> {
    non_blank_text("task must contain non-whitespace text")
}

fn accepted_mission_ref_text() -> impl Strategy<Value = String> {
    non_blank_text("mission ref must contain non-whitespace text")
}

fn non_blank_text(reason: &'static str) -> impl Strategy<Value = String> {
    any::<String>().prop_filter(reason, |value| !value.trim().is_empty())
}

#[test]
fn induct_directive_carries_the_general_placeholder_contract() {
    assert_directive_fields(
        &[
            "induct",
            "rename HostContext::L2a to HostContext::CliL2a",
            "--mission",
            "v0.1-stable",
        ],
        "rename HostContext::L2a to HostContext::CliL2a",
        "v0.1-stable",
    );
}

#[test]
fn induct_directive_reports_none_when_mission_is_absent() {
    assert_directive_fields(
        &["induct", "ship clean change"],
        "ship clean change",
        "none",
    );
}

#[test]
fn induct_directive_preserves_representative_field_boundaries_losslessly() {
    let cases = [
        (
            "  preserve punctuation: a=b, c::d, and spaces  ",
            "mission/ref with spaces",
        ),
        ("line1\nline2 --flag-like text", "mission\nalpha"),
        ("tabs\tand spaces remain payload", "mission\tbeta"),
        ("carriage\rreturn", "mission\rrelease"),
        (r"path\to\mission", r"anchor\path"),
        ("unicode α é 🙂 remains readable", "mission α é 🙂"),
    ];

    for (task, mission_ref) in cases {
        assert_directive_fields(
            &["induct", task, "--mission", mission_ref],
            task,
            mission_ref,
        );
    }
}

#[test]
fn induct_directive_accepts_task_values_starting_with_hyphens() {
    let cases = [
        "-single-hyphen-prefix",
        "--double-hyphen-prefix",
        "--mission",
        "--json",
    ];

    for task in cases {
        assert_directive_fields(&cli_args_for_task(task), task, "none");
    }
}

#[test]
fn induct_directive_accepts_mission_ref_values_starting_with_hyphens() {
    let cases = ["- ", "-short-ref", "--long-ref", "--json"];

    for mission_ref in cases {
        assert_directive_fields(
            &["induct", "ship clean change", "--mission", mission_ref],
            "ship clean change",
            mission_ref,
        );
    }
}

#[test]
fn induct_optional_inputs_do_not_suppress_the_stdout_prompt_contract() {
    let cases: &[(&[&str], &str)] = &[
        (&["induct", "ship clean change", "--quiet"], "none"),
        (&["induct", "ship clean change", "--json"], "none"),
        (
            &[
                "induct",
                "ship clean change",
                "--evidence",
                "trace=a=b",
                "--reason",
                "bounded proof",
            ],
            "none",
        ),
        (
            &[
                "induct",
                "ship clean change",
                "--manifest",
                "./manifest.json",
            ],
            "none",
        ),
    ];

    for (args, mission_ref) in cases {
        assert_directive_fields(args, "ship clean change", mission_ref);
    }
}

proptest! {
    #![proptest_config(ProptestConfig::with_cases(24))]

    #[test]
    fn induct_directive_preserves_any_accepted_task_losslessly(task in accepted_task_text()) {
        let args = cli_args_for_task(&task);
        assert_directive_fields(&args, &task, "none");
    }

    #[test]
    fn induct_directive_preserves_any_mission_ref_losslessly(mission_ref in accepted_mission_ref_text()) {
        assert_directive_fields(
            &["induct", "ship clean change", "--mission", &mission_ref],
            "ship clean change",
            &mission_ref,
        );
    }
}
