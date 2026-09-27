---
okf_version: "0.2"
type: Function
title: fixture_two_symbols
description: "A loaded, active schematic holding two plain symbols, so a test can"
resource: crates/oxide-app/tests/regression/undo_marker_divergence.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/tests/regression/undo_marker_divergence/fixture_two_symbols
language: rust
---

# fixture_two_symbols

A loaded, active schematic holding two plain symbols, so a test can

## Signature

```rust
fn fixture_two_symbols() -> (Oxide, uuid::Uuid, uuid::Uuid)
```

## Docstring

A loaded, active schematic holding two plain symbols, so a test can
spend one on a gateway edit and one on a gateway-bypassing edit.

## Source
Lines 144–161 in `crates/oxide-app/tests/regression/undo_marker_divergence.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [undo_marker_divergence](/crates/oxide-app/tests/regression/undo_marker_divergence.md) |
| calls | [add_tab](/crates/oxide-app/tests/regression/undo_marker_divergence/add_tab.md) |
| calls | [sheet_with](/crates/oxide-app/tests/regression/undo_marker_divergence/sheet_with.md) |
| calls | [activate](/crates/oxide-app/tests/regression/undo_marker_divergence/activate.md) |
| called_by | [one_gateway_edit_then_one_bypassing_edit](/crates/oxide-app/tests/regression/undo_marker_divergence/one_gateway_edit_then_one_bypassing_edit.md) |
