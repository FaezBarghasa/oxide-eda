---
okf_version: "0.2"
type: Function
title: type_5_during_line_draw_commits_at_5mm
description: "Phase-5 #2 — `placement_input` of \"5\" pinned with `LineLength`"
resource: crates/oxide-app/tests/regression/library_cross_track.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/tests/regression/library_cross_track/type_5_during_line_draw_commits_at_5mm
language: rust
---

# type_5_during_line_draw_commits_at_5mm

Phase-5 #2 — `placement_input` of "5" pinned with `LineLength`

## Signature

```rust
fn type_5_during_line_draw_commits_at_5mm()
```

## Decorators

- `test`

## Docstring

Phase-5 #2 — `placement_input` of "5" pinned with `LineLength`
kind, on a Line tool's second click — even though the cursor is
at (10, 0), the line's end Point must land at exactly (5, 0).
Drives the dispatcher end-to-end via `Message::Library(...)`.

This pins the cross-track interaction: typing a digit during a
Line gesture (Track D) must override the cursor distance for the
commit click, irrespective of what auto-Horizontal / auto-snap
machinery is wired in upstream phases.
[test]

## Source
Lines 1039–1128 in `crates/oxide-app/tests/regression/library_cross_track.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [library_cross_track](/crates/oxide-app/tests/regression/library_cross_track.md) |
| calls | [fixture_empty_footprint_editor](/crates/oxide-app/tests/regression/library_cross_track/fixture_empty_footprint_editor.md) |
| calls | [find](/crates/oxide-app/src/library/editor/symbol/context_menu/rows/find.md) |
