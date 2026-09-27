---
okf_version: "0.2"
type: Function
title: rect_pattern
description: v0.22 Phase B3 — Rectangular Pattern. Click 1 picks the source entity
resource: crates/oxide-app/src/library/editor/footprint/updates/sketch/tools/transform.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:48Z"
concept_id: crates/oxide-app/src/library/editor/footprint/updates/sketch/tools/transform/rect_pattern
language: rust
---

# rect_pattern

v0.22 Phase B3 — Rectangular Pattern. Click 1 picks the source entity

## Signature

```rust
fn rect_pattern(editor: &mut crate::app::FootprintEditorState, ctx: &ToolClickCtx)
```

## Docstring

v0.22 Phase B3 — Rectangular Pattern. Click 1 picks the source entity
(whatever was clicked, including a freshly-minted Point if the click
missed everything). Mints a default 2×2 grid with 5 mm spacing,
sequential numbering. User edits via JSON until a Properties sub-form
lands.

## Source
Lines 696–719 in `crates/oxide-app/src/library/editor/footprint/updates/sketch/tools/transform.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [transform](/crates/oxide-app/src/library/editor/footprint/updates/sketch/tools/transform.md) |
| calls | [apply_sketch_edit_with_warnings](/crates/oxide-app/src/library/editor/footprint/sketch_dispatch/apply_sketch_edit_with_warnings.md) |
| called_by | [apply](/crates/oxide-app/src/library/editor/footprint/updates/sketch/tools/transform/apply.md) |
