---
okf_version: "0.2"
type: Function
title: circular_pattern
description: v0.22 Phase B4 — Circular Pattern. Click 1 picks the source entity.
resource: crates/oxide-app/src/library/editor/footprint/updates/sketch/tools/transform.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:48Z"
concept_id: crates/oxide-app/src/library/editor/footprint/updates/sketch/tools/transform/circular_pattern
language: rust
---

# circular_pattern

v0.22 Phase B4 — Circular Pattern. Click 1 picks the source entity.

## Signature

```rust
fn circular_pattern(editor: &mut crate::app::FootprintEditorState, ctx: &ToolClickCtx)
```

## Docstring

v0.22 Phase B4 — Circular Pattern. Click 1 picks the source entity.
The polar array requires a centre Point — mint a fresh one 5 mm to the
right of the click position so the array doesn't all stack on the
source. Default count 4, sweep 360°.

## Source
Lines 725–759 in `crates/oxide-app/src/library/editor/footprint/updates/sketch/tools/transform.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [transform](/crates/oxide-app/src/library/editor/footprint/updates/sketch/tools/transform.md) |
| calls | [apply_sketch_edit_with_warnings](/crates/oxide-app/src/library/editor/footprint/sketch_dispatch/apply_sketch_edit_with_warnings.md) |
| called_by | [apply](/crates/oxide-app/src/library/editor/footprint/updates/sketch/tools/transform/apply.md) |
