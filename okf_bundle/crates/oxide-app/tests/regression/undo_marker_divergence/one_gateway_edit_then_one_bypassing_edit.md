---
okf_version: "0.2"
type: Function
title: one_gateway_edit_then_one_bypassing_edit
description: "Delete one symbol through the gateway, then move the other through"
resource: crates/oxide-app/tests/regression/undo_marker_divergence.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/tests/regression/undo_marker_divergence/one_gateway_edit_then_one_bypassing_edit
language: rust
---

# one_gateway_edit_then_one_bypassing_edit

Delete one symbol through the gateway, then move the other through

## Signature

```rust
fn one_gateway_edit_then_one_bypassing_edit() -> (Oxide, uuid::Uuid)
```

## Docstring

Delete one symbol through the gateway, then move the other through
the Move Selection dialog, which calls `engine.execute` directly.

## Source
Lines 165–209 in `crates/oxide-app/tests/regression/undo_marker_divergence.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [undo_marker_divergence](/crates/oxide-app/tests/regression/undo_marker_divergence.md) |
| calls | [fixture_two_symbols](/crates/oxide-app/tests/regression/undo_marker_divergence/fixture_two_symbols.md) |
| calls | [find](/crates/oxide-app/src/library/editor/symbol/context_menu/rows/find.md) |
| called_by | [two_edits_then_two_undos_restores_both](/crates/oxide-app/tests/regression/undo_marker_divergence/two_edits_then_two_undos_restores_both.md) |
| called_by | [undo_availability_agrees_with_what_undo_can_actually_do](/crates/oxide-app/tests/regression/undo_marker_divergence/undo_availability_agrees_with_what_undo_can_actually_do.md) |
