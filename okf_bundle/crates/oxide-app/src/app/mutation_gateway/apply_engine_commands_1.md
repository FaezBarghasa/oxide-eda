---
okf_version: "0.2"
type: Function
title: apply_engine_commands
resource: crates/oxide-app/src/app/mutation_gateway.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-13T09:33:25Z"
concept_id: crates/oxide-app/src/app/mutation_gateway/apply_engine_commands_1
language: rust
---

# apply_engine_commands

## Signature

```rust
pub(crate) fn apply_engine_commands(
        &mut self,
        commands: Vec<oxide_engine::Command>,
        clear_overlay_cache: bool,
        update_selection_info: bool,
    ) -> bool
```

## Visibility

- `pub(crate)`

## Source
Lines 54–84 in `crates/oxide-app/src/app/mutation_gateway.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [mutation_gateway](/crates/oxide-app/src/app/mutation_gateway.md) |
| calls | [log_error](/crates/oxide-app/src/diagnostics/log_error.md) |
