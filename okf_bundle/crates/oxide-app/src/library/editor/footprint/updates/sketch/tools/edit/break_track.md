---
okf_version: "0.2"
type: Function
title: break_track
description: "#372 — Break Track. Single click on a sketch Line: hit-test the click"
resource: crates/oxide-app/src/library/editor/footprint/updates/sketch/tools/edit.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:48Z"
concept_id: crates/oxide-app/src/library/editor/footprint/updates/sketch/tools/edit/break_track
language: rust
---

# break_track

#372 — Break Track. Single click on a sketch Line: hit-test the click

## Signature

```rust
fn break_track(editor: &mut crate::app::FootprintEditorState, ctx: &ToolClickCtx)
```

## Docstring

#372 — Break Track. Single click on a sketch Line: hit-test the click
against every Line (0.30 mm tolerance, nearest stroke wins — the same
pick shape as the Trim arm above), project the click onto that Line to
a parameter `t`, and hand off to the `split_line` primitive (#360),
which divides the Line into two Lines meeting at a new mid Point.

History — ONE undo step. `SketchToolClick` is classified as a
footprint mutation, so the router captured exactly one pre-mutation
snapshot before dispatching here (see
`updates/mod.rs::mutates_footprint_state` + its blanket pre-push).
This fn must NOT push again — mirroring the Fillet / Trim fns — so a
split stays a single undo step.

Errors — graceful, no mutation. `split_line` leaves the sketch
byte-for-byte unchanged on every failure (a degenerate line, a `t` too
close to an endpoint, …). On ANY `Err` — and on a click that misses
every Line — we surface a warning the same shape as the Trim miss,
leave the tool armed, and mutate nothing.

## Source
Lines 661–772 in `crates/oxide-app/src/library/editor/footprint/updates/sketch/tools/edit.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [edit](/crates/oxide-app/src/library/editor/footprint/updates/sketch/tools/edit.md) |
| calls | [find](/crates/oxide-app/src/library/editor/symbol/context_menu/rows/find.md) |
| calls | [pick_line_and_param](/crates/oxide-app/src/library/editor/footprint/updates/sketch/tools/edit/pick_line_and_param.md) |
| calls | [split_line](/crates/oxide-sketch/src/split/mod/split_line.md) |
| calls | [apply_sketch_edit_with_warnings](/crates/oxide-app/src/library/editor/footprint/sketch_dispatch/apply_sketch_edit_with_warnings.md) |
| called_by | [apply](/crates/oxide-app/src/library/editor/footprint/updates/sketch/tools/edit/apply.md) |
