---
okf_version: "0.2"
type: Function
title: placement_input_does_not_corrupt_history_on_undo
description: "Phase-5 #9 — `placement_input` is transient UI state and must NOT"
resource: crates/oxide-app/tests/regression/library_cross_track.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/tests/regression/library_cross_track/placement_input_does_not_corrupt_history_on_undo
language: rust
---

# placement_input_does_not_corrupt_history_on_undo

Phase-5 #9 — `placement_input` is transient UI state and must NOT

## Signature

```rust
fn placement_input_does_not_corrupt_history_on_undo()
```

## Decorators

- `test`

## Docstring

Phase-5 #9 — `placement_input` is transient UI state and must NOT
be restored by undo. The snapshot taken in `push_history` captures
only persisted footprint / sketch state (`file`, `pads`,
`selected_*`); `state.placement_input` is intentionally absent.
This pins that contract: after placing a Line via the
placement_input flow + undo, the line is gone AND
`state.placement_input` is `None` (not `Some("5")`).
[test]

## Source
Lines 375–481 in `crates/oxide-app/tests/regression/library_cross_track.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [library_cross_track](/crates/oxide-app/tests/regression/library_cross_track.md) |
| calls | [fixture_empty_footprint_editor](/crates/oxide-app/tests/regression/library_cross_track/fixture_empty_footprint_editor.md) |
| calls | [editor_state_proj](/crates/oxide-app/tests/regression/library_cross_track/editor_state_proj.md) |
