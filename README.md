# banchor

For agents that drift mid-task. banchor reads the agent's working session against the mission rail (goals, values, anchors declared at session start) and emits UNANCHORED when the next step would drift away from the stated mission. The task taxonomy is closed (7 classes at v0.1); the prompt library that powers anchoring evolves continuously via empirical-lift evaluation, so the same `banchor induct` invocation gets stricter at catching mission drift as the corpus matures.


Prompt lookup tool. Agent names a task class from a fixed list; banchor returns the prompt for that task class. The prompt tells the agent how to check the task against a stated mission.

Anchors a task description against a named or filesystem-rooted mission, classifies the task into a typed `TaskClass`, writes the verdict on stdout, exits with a discriminating code so the calling agent can branch.

```
banchor induct <task>             classify a task + emit next-step directive; exit 0 / 1 / 2 / 64
banchor task-classes              list the 11 supported task-class identifiers
banchor init                      scaffold a mission alias file in the current directory
banchor update                    self-update to the latest published version
banchor tail                      stream recent next-step transcripts
banchor explain                   print taxonomy + exit-code reference
```

Exit code contract: `0` task anchored to mission, `1` task NOT anchored, `2` internal error, `64` malformed input.

## Install

```sh
cargo install --git https://github.com/bvasilenko/bAnchor
```

## Use

```sh
banchor induct "rename HostContext::L2a to HostContext::CliL2a" \
    --mission ./missions/v0.1-stable.toml
# stdout: ANCHORED to mission "v0.1-stable"; task-class refactor
# exit: 0

banchor induct "rewrite onboarding copy in punchier tone" \
    --mission brand-voice
# stdout: ANCHORED to alias mission "brand-voice"; task-class rewrite
# exit: 0
```

The `--mission <path-or-name>` flag resolves a filesystem path (starts with `/`, `./`, or `../`) or a named alias. Supply evidence with repeatable `--evidence <id>=<value>` flags. Optional flags: `--manifest <path>`, `--json`, `--quiet`, `--reason <text>`.

## Task taxonomy

Closed 11-variant `TaskClass` enum. The taxonomy is fixed at this version; widening lands in a later version.

| Category | Variants |
|---|---|
| Engineering | `refactor`, `migration`, `feature`, `bug-fix`, `spike`, `research`, `scaffolding` |
| Editorial | `draft`, `rewrite`, `localize`, `brand-conform` |

`banchor task-classes` prints the full list.

## License

MIT.
