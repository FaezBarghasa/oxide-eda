---
okf_version: "0.2"
type: Function
title: undo_availability_agrees_with_what_undo_can_actually_do
description: "The sharper statement of the same defect: after the marker stack was"
resource: crates/oxide-app/tests/regression/undo_marker_divergence.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/tests/regression/undo_marker_divergence/undo_availability_agrees_with_what_undo_can_actually_do
language: rust
---

# undo_availability_agrees_with_what_undo_can_actually_do

The sharper statement of the same defect: after the marker stack was

## Signature

```rust
fn undo_availability_agrees_with_what_undo_can_actually_do()
```

## Decorators

- `test`

## Docstring

The sharper statement of the same defect: after the marker stack was
exhausted the menu still offered Undo, because `can_undo` is read from
the engine while `apply_engine_undo` was driven by the marker stack.
An enabled menu item that silently did nothing.
[test]

## Source
Lines 266–285 in `crates/oxide-app/tests/regression/undo_marker_divergence.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [undo_marker_divergence](/crates/oxide-app/tests/regression/undo_marker_divergence.md) |
| calls | [one_gateway_edit_then_one_bypassing_edit](/crates/oxide-app/tests/regression/undo_marker_divergence/one_gateway_edit_then_one_bypassing_edit.md) |
