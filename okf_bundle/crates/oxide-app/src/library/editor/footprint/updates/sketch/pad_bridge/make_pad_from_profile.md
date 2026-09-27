---
okf_version: "0.2"
type: Function
title: make_pad_from_profile
description: "Reachable from the Sketch ▸ Modify dropdown as well as the bar itself,"
resource: crates/oxide-app/src/library/editor/footprint/updates/sketch/pad_bridge.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/library/editor/footprint/updates/sketch/pad_bridge/make_pad_from_profile
language: rust
---

# make_pad_from_profile

Reachable from the Sketch ▸ Modify dropdown as well as the bar itself,

## Signature

```rust
fn make_pad_from_profile(editor: &mut crate::app::FootprintEditorState)
```

## Docstring

Reachable from the Sketch ▸ Modify dropdown as well as the bar itself,
and it is a one-shot action rather than a tool arm — so it has to
dismiss the menu on its own. (Tool-arming rows go through
`ActiveBarSetSketchTool`, which already does.)

v0.22 Phase D4 — convert the closed-loop profile that includes the
currently-selected Line into a `PadShape::Custom(SketchProfile)` pad.

Walk: start from the selected Line, use
`oxide_bake::profile::trace_closed_profile` to chase the
unique-incident-edge cycle in the sketch. On success, compute the
centroid of the traced vertices, mint a centre `Point` there, and
attach a `PadAttr` whose `shape` is
`Custom(SketchProfile{source: vec![seed_line_id]})`. The bake re-walks
the loop on the next solve and emits a `LibPadShape::Custom` polygon.

Designator: `next_pad_num` from existing `PadAttr` entities, identical
pattern to `apply_sketch_role(.., RoleTag::Pad)` for ordering
consistency.

Fail modes (silent except for warning push):
- No Line selected → "select a Line first".
- Line is not part of a closed loop → "loop is open or branches".
- `last_solve` is None (no solve has run yet) → ask user to interact
briefly so a solve fires, then retry. (Auto-mint paths on entry to
Sketch mode already trigger a solve, so this is rare.)

## Source
Lines 151–388 in `crates/oxide-app/src/library/editor/footprint/updates/sketch/pad_bridge.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [pad_bridge](/crates/oxide-app/src/library/editor/footprint/updates/sketch/pad_bridge.md) |
| calls | [find](/crates/oxide-app/src/library/editor/symbol/context_menu/rows/find.md) |
| calls | [trace_closed_profile](/crates/oxide-bake/src/profile/trace_closed_profile.md) |
| calls | [apply_sketch_edit_with_warnings](/crates/oxide-app/src/library/editor/footprint/sketch_dispatch/apply_sketch_edit_with_warnings.md) |
| called_by | [apply](/crates/oxide-app/src/library/editor/footprint/updates/sketch/pad_bridge/apply.md) |
