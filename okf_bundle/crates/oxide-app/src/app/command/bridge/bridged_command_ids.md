---
okf_version: "0.2"
type: Function
title: bridged_command_ids
resource: crates/oxide-app/src/app/command/bridge.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:49Z"
concept_id: crates/oxide-app/src/app/command/bridge/bridged_command_ids
language: rust
---

# bridged_command_ids

## Signature

```rust
fn bridged_command_ids(src: &str) -> Vec<String>
```

## Source
Lines 236–267 in `crates/oxide-app/src/app/command/bridge.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [bridge](/crates/oxide-app/src/app/command/bridge.md) |
| calls | [match_block](/crates/oxide-app/src/app/command/bridge/match_block.md) |
| calls | [find](/crates/oxide-app/src/library/editor/symbol/context_menu/rows/find.md) |
| called_by | [every_bridged_command_id_resolves_in_the_catalog](/crates/oxide-app/src/app/command/bridge/every_bridged_command_id_resolves_in_the_catalog.md) |
| called_by | [resolvable_command_ids](/crates/oxide-app/src/app/command/bridge/resolvable_command_ids.md) |
