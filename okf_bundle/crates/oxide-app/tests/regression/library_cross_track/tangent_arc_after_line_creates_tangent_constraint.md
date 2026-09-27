---
okf_version: "0.2"
type: Function
title: tangent_arc_after_line_creates_tangent_constraint
description: "Phase-5 #3 — Drive a Line gesture to completion, then switch to"
resource: crates/oxide-app/tests/regression/library_cross_track.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/tests/regression/library_cross_track/tangent_arc_after_line_creates_tangent_constraint
language: rust
---

# tangent_arc_after_line_creates_tangent_constraint

Phase-5 #3 — Drive a Line gesture to completion, then switch to

## Signature

```rust
fn tangent_arc_after_line_creates_tangent_constraint()
```

## Decorators

- `test`

## Docstring

Phase-5 #3 — Drive a Line gesture to completion, then switch to
the TangentArc tool and chain off the line's end. The dispatcher's
TangentArc handler must auto-emit a `TangentLineArc` constraint
linking the freshly minted Arc to the trailing Line.

Pure dispatcher routing — no `tool_pending` seeding. Mirrors the
real user flow (draw line, switch tool, click endpoint, click off
to commit).
[test]

## Source
Lines 1139–1266 in `crates/oxide-app/tests/regression/library_cross_track.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [library_cross_track](/crates/oxide-app/tests/regression/library_cross_track.md) |
| calls | [fixture_empty_footprint_editor](/crates/oxide-app/tests/regression/library_cross_track/fixture_empty_footprint_editor.md) |
| calls | [find](/crates/oxide-app/src/library/editor/symbol/context_menu/rows/find.md) |
