---
okf_version: "0.2"
type: Function
title: apply_engine_redo
description: "Redo one step of the **active** engine's own history. Mirror of"
resource: crates/oxide-app/src/app/mutation_gateway.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-13T09:33:25Z"
concept_id: crates/oxide-app/src/app/mutation_gateway/apply_engine_redo_1
language: rust
---

# apply_engine_redo

Redo one step of the **active** engine's own history. Mirror of

## Signature

```rust
pub(crate) fn apply_engine_redo(&mut self, update_selection_info: bool) -> bool
```

## Visibility

- `pub(crate)`

## Docstring

Redo one step of the **active** engine's own history. Mirror of
[`Oxide::apply_engine_undo`].

## Source
Lines 162–178 in `crates/oxide-app/src/app/mutation_gateway.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [mutation_gateway](/crates/oxide-app/src/app/mutation_gateway.md) |
| calls | [log_error](/crates/oxide-app/src/diagnostics/log_error.md) |
