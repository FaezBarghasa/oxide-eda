---
okf_version: "0.2"
type: Function
title: a_pasted_batch_is_one_undo_step
description: "Batch atomicity through real messages: a paste that places three"
resource: crates/oxide-app/tests/regression/undo_marker_divergence.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/tests/regression/undo_marker_divergence/a_pasted_batch_is_one_undo_step
language: rust
---

# a_pasted_batch_is_one_undo_step

Batch atomicity through real messages: a paste that places three

## Signature

```rust
fn a_pasted_batch_is_one_undo_step()
```

## Decorators

- `test`

## Docstring

Batch atomicity through real messages: a paste that places three
objects is one user action, so one Undo must take all three back.
`EditMsg::Paste` routes to `apply_engine_commands`, which is now
`Engine::execute_batch`.

This one is a guard, not a reproduction — the marker stack got this
case right, by recording `steps: 3` and undoing three times. What it
pins is that moving the grouping into the engine did not lose it.
[test]

## Source
Lines 296–334 in `crates/oxide-app/tests/regression/undo_marker_divergence.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [undo_marker_divergence](/crates/oxide-app/tests/regression/undo_marker_divergence.md) |
| calls | [add_tab](/crates/oxide-app/tests/regression/undo_marker_divergence/add_tab.md) |
| calls | [sheet_with](/crates/oxide-app/tests/regression/undo_marker_divergence/sheet_with.md) |
| calls | [activate](/crates/oxide-app/tests/regression/undo_marker_divergence/activate.md) |
