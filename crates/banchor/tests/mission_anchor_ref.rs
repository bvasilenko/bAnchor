use std::str::FromStr;

use banchor::{EvidenceMap, MissionAnchorRef};
use proptest::prelude::*;

#[test]
fn mission_anchor_resolution_uses_only_documented_path_prefixes() {
    let path_cases = ["/tmp/mission.md", "./mission.md", "../mission.md"];
    let alias_cases = ["default", "brand", "team-alpha", ".hidden", "nested/file"];

    for value in path_cases {
        assert!(matches!(
            MissionAnchorRef::from_str(value).unwrap(),
            MissionAnchorRef::Path(_)
        ));
    }

    for value in alias_cases {
        assert_eq!(
            MissionAnchorRef::from_str(value).unwrap(),
            MissionAnchorRef::Alias(value.to_owned())
        );
    }
}

#[test]
fn mission_anchor_display_round_trips_original_text() {
    for value in [
        "/tmp/mission.md",
        "./mission.md",
        "../mission.md",
        "default",
    ] {
        assert_eq!(
            MissionAnchorRef::from_str(value).unwrap().to_string(),
            value
        );
    }
}

#[test]
fn empty_mission_anchor_is_rejected() {
    assert!(MissionAnchorRef::from_str("").is_err());
}

#[test]
fn evidence_map_rejects_empty_ids() {
    for id in ["", " ", "\t", " \t "] {
        assert!(EvidenceMap::from_pairs([(id.to_owned(), "value".to_owned())]).is_err());
    }
}

#[test]
fn evidence_map_preserves_sorted_keys_and_values() {
    let map = EvidenceMap::from_pairs([
        ("b".to_owned(), "two".to_owned()),
        ("empty".to_owned(), String::new()),
        ("a".to_owned(), "one=still-one-value".to_owned()),
    ])
    .unwrap();

    let keys = map.as_map().keys().cloned().collect::<Vec<_>>();

    assert_eq!(keys, ["a", "b", "empty"]);
    assert_eq!(map.as_map()["a"], "one=still-one-value");
    assert_eq!(map.as_map()["empty"], "");
}

#[test]
fn repeated_evidence_ids_use_the_last_value() {
    let map = EvidenceMap::from_pairs([
        ("same".to_owned(), "first".to_owned()),
        ("same".to_owned(), "second".to_owned()),
    ])
    .unwrap();

    assert_eq!(map.as_map()["same"], "second");
}

proptest! {
    #[test]
    fn mission_anchor_alias_round_trips_arbitrary_non_path_strings(
        s in "[a-zA-Z0-9._-]{1,64}"
    ) {
        prop_assume!(!s.starts_with('/'));
        prop_assume!(!s.starts_with("./"));
        prop_assume!(!s.starts_with("../"));

        let anchor = MissionAnchorRef::from_str(&s).unwrap();
        prop_assert!(matches!(anchor, MissionAnchorRef::Alias(_)));
        prop_assert_eq!(anchor.to_string(), s);
    }

    #[test]
    fn mission_anchor_absolute_path_round_trips(suffix in "[a-zA-Z0-9/_.-]{0,32}") {
        let s = format!("/{suffix}");
        let anchor = MissionAnchorRef::from_str(&s).unwrap();
        prop_assert!(matches!(anchor, MissionAnchorRef::Path(_)));
        prop_assert_eq!(anchor.to_string(), s);
    }
}
