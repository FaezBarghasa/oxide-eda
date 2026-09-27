---
okf_version: "0.2"
type: Function
title: edge_arc
description: "#467 — Edge Arc: click start, click end, click a third point the arc"
resource: crates/oxide-app/src/library/editor/footprint/updates/sketch/tools/draw.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:48Z"
concept_id: crates/oxide-app/src/library/editor/footprint/updates/sketch/tools/draw/edge_arc
language: rust
---

# edge_arc

#467 — Edge Arc: click start, click end, click a third point the arc

## Signature

```rust
fn edge_arc(editor: &mut crate::app::FootprintEditorState, ctx: &ToolClickCtx)
```

## Docstring

#467 — Edge Arc: click start, click end, click a third point the arc
must pass through. Unlike `Arc` (centre-first), the centre is
*derived* — not clicked — from the circumcircle of the three picks,
reusing the shared solver (#461/#483) instead of re-deriving the
geometry here.

## Source
Lines 477–574 in `crates/oxide-app/src/library/editor/footprint/updates/sketch/tools/draw.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [draw](/crates/oxide-app/src/library/editor/footprint/updates/sketch/tools/draw.md) |
| calls | [find](/crates/oxide-app/src/library/editor/symbol/context_menu/rows/find.md) |
| calls | [circumcircle](/crates/oxide-types/src/schematic/mod/circumcircle.md) |
| calls | [apply_sketch_edit_with_warnings](/crates/oxide-app/src/library/editor/footprint/sketch_dispatch/apply_sketch_edit_with_warnings.md) |
| called_by | [apply](/crates/oxide-app/src/library/editor/footprint/updates/sketch/tools/draw/apply.md) |
