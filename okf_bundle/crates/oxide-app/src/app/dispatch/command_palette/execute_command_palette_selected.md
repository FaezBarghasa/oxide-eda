---
okf_version: "0.2"
type: Function
title: execute_command_palette_selected
resource: crates/oxide-app/src/app/dispatch/command_palette.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/app/dispatch/command_palette/execute_command_palette_selected
language: rust
---

# execute_command_palette_selected

## Signature

```rust
impl Oxide { fn execute_command_palette_selected(&mut self) -> Task<Message> }
```

## Source
Lines 71–112 in `crates/oxide-app/src/app/dispatch/command_palette.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [command_palette](/crates/oxide-app/src/app/dispatch/command_palette.md) |
| calls | [build_catalog](/crates/oxide-app/src/app/command_palette/build_catalog.md) |
| calls | [rank_results](/crates/oxide-app/src/app/command_palette/rank_results.md) |
| calls | [core_to_message](/crates/oxide-app/src/app/command/bridge/core_to_message.md) |
