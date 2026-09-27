---
okf_version: "0.2"
type: Function
title: resolvable_command_ids
description: "Pull every quoted command-id literal out of `src`'s match-arm"
resource: crates/oxide-app/src/app/command/bridge.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:49Z"
concept_id: crates/oxide-app/src/app/command/bridge/resolvable_command_ids
language: rust
---

# resolvable_command_ids

Pull every quoted command-id literal out of `src`'s match-arm

## Signature

```rust
fn resolvable_command_ids(src: &str) -> HashSet<String>
```

## Docstring

Pull every quoted command-id literal out of `src`'s match-arm
lines (each starts, once trimmed, with `"` or — on a wrapped
multi-id arm — with `|`). Deliberately tiny, no regex dependency
— mirrors `keymap::menu_command_tests`'s `ids_from_call`.
Every id `core_to_message` can resolve: the match arms it names
literally, plus the Active Bar table its fallthrough consults.
A source scan alone would miss the table and report all 59 Active
Bar commands as dead.

## Source
Lines 230–234 in `crates/oxide-app/src/app/command/bridge.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [bridge](/crates/oxide-app/src/app/command/bridge.md) |
| calls | [bridged_command_ids](/crates/oxide-app/src/app/command/bridge/bridged_command_ids.md) |
| calls | [command_ids](/crates/oxide-app/src/app/command/active_bar/command_ids.md) |
| called_by | [unmapped_command_ids_only_shrink](/crates/oxide-app/src/app/command/bridge/unmapped_command_ids_only_shrink.md) |
