---
okf_version: "0.2"
type: Function
title: apply_engine_undo
description: "Undo one step of the **active** engine's own history."
resource: crates/oxide-app/src/app/mutation_gateway.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-13T09:33:25Z"
concept_id: crates/oxide-app/src/app/mutation_gateway/apply_engine_undo_1
language: rust
---

# apply_engine_undo

Undo one step of the **active** engine's own history.

## Signature

```rust
pub(crate) fn apply_engine_undo(&mut self, update_selection_info: bool) -> bool
```

## Visibility

- `pub(crate)`

## Docstring

Undo one step of the **active** engine's own history.

One `Engine::undo` per invocation: a batch recorded through
`apply_engine_commands` is already a single entry, so grouping needs
no second stack to count it. This used to be driven by an app-side
marker stack, which was one global stack shared by every open
document's engine and was never cleared on tab switch — so its
counts drifted out of step with the history they were counting and
an edit could be reverted with no invalidation and no dirty flag
(#533). Reading the history straight off the engine that owns it
also makes the `Engine::can_undo` the Edit menu reads
(`app/view/mod.rs`) agree with what Undo will actually do.

## Source
Lines 142–158 in `crates/oxide-app/src/app/mutation_gateway.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [mutation_gateway](/crates/oxide-app/src/app/mutation_gateway.md) |
| calls | [log_error](/crates/oxide-app/src/diagnostics/log_error.md) |
