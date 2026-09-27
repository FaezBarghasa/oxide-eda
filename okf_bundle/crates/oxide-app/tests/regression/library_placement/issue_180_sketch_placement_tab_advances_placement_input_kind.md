---
okf_version: "0.2"
type: Function
title: issue_180_sketch_placement_tab_advances_placement_input_kind
description: "Issue #180 — complementary user-visible coverage for the standalone"
resource: crates/oxide-app/tests/regression/library_placement.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/tests/regression/library_placement/issue_180_sketch_placement_tab_advances_placement_input_kind
language: rust
---

# issue_180_sketch_placement_tab_advances_placement_input_kind

Issue #180 — complementary user-visible coverage for the standalone

## Signature

```rust
fn issue_180_sketch_placement_tab_advances_placement_input_kind()
```

## Decorators

- `test`

## Docstring

Issue #180 — complementary user-visible coverage for the standalone
canvas-to-dispatcher routing fix (`translate_footprint_canvas_msg`
in `standalone/footprint.rs`). The in-crate mapping test living
alongside that function is what actually catches the routing bug —
it proves the bridge no longer discards `SketchPlacementInputTab`
into a no-op `Save`. This test complements it (not replaces it) by
proving what a user sees once the message correctly reaches the
dispatcher: Tab advances the focused placement-input field instead
of silently doing nothing.
[test]

## Source
Lines 1461–1517 in `crates/oxide-app/tests/regression/library_placement.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [library_placement](/crates/oxide-app/tests/regression/library_placement.md) |
