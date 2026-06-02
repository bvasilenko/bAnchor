# banchor

CLI mission-rail anchor for agentic loops. Reads a task description plus an optional mission anchor; emits a `next-step` directive on stdout naming the typed task class and the anchored mission.

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
