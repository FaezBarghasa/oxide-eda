---
okf_version: "0.2"
type: Function
title: ctrl_z_during_tangent_arc_undoes_last_segment
description: "Phase-5 #8 — Drive a TangentArc gesture end-to-end via the"
resource: crates/oxide-app/tests/regression/library_cross_track.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/tests/regression/library_cross_track/ctrl_z_during_tangent_arc_undoes_last_segment
language: rust
---

# ctrl_z_during_tangent_arc_undoes_last_segment

Phase-5 #8 — Drive a TangentArc gesture end-to-end via the

## Signature

```rust
fn ctrl_z_during_tangent_arc_undoes_last_segment()
```

## Decorators

- `test`

## Docstring

Phase-5 #8 — Drive a TangentArc gesture end-to-end via the
dispatcher (no manual `tool_pending` seeding); then issue
`Message::Edit(EditMsg::Undo)`. Both clicks should roll back: the Arc, its
auto-generated `TangentLineArc` constraint, and any anchor / centre
Points the dispatcher minted on the way. The seed Line stays.

The dispatcher's TangentArc handler emits two separate
`mutates_footprint_state` messages (one per click), so the second
click's `push_history` snapshot covers exactly the click-2 work
(mint arc + tangent constraint). A single `Message::Edit(EditMsg::Undo)` rolls
back that one snapshot — the click-1 mint stays. We test that
behaviour here: a single Undo must remove the arc + constraint
without disturbing the seed Line.
[test]

## Source
Lines 213–365 in `crates/oxide-app/tests/regression/library_cross_track.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [library_cross_track](/crates/oxide-app/tests/regression/library_cross_track.md) |
| calls | [editor_state_proj](/crates/oxide-app/tests/regression/library_cross_track/editor_state_proj.md) |
