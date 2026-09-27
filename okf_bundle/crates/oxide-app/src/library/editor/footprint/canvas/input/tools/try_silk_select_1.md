---
okf_version: "0.2"
type: Function
title: try_silk_select
description: "v0.18.18/v0.21 — Silk-front graphic hit, filter-gated per kind."
resource: crates/oxide-app/src/library/editor/footprint/canvas/input/tools.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:48Z"
concept_id: crates/oxide-app/src/library/editor/footprint/canvas/input/tools/try_silk_select_1
language: rust
---

# try_silk_select

v0.18.18/v0.21 — Silk-front graphic hit, filter-gated per kind.

## Signature

```rust
pub(in crate::library::editor::footprint::canvas) fn try_silk_select(
        &self,
        cstate: &FootprintCanvasState,
        world: (f64, f64),
    ) -> Option<canvas::Action<LibraryMessage>>
```

## Visibility

- `pub(in crate::library::editor::footprint::canvas)`

## Docstring

v0.18.18/v0.21 — Silk-front graphic hit, filter-gated per kind.
Maps each FpGraphicKind to its matching `selection_filter.*`
bit so the user can disable Tracks / Arcs / Texts / Regions /
Fills independently.

## Source
Lines 494–542 in `crates/oxide-app/src/library/editor/footprint/canvas/input/tools.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [tools](/crates/oxide-app/src/library/editor/footprint/canvas/input/tools.md) |
| calls | [silk_f_hit_at](/crates/oxide-app/src/library/editor/footprint/canvas/mod/silk_f_hit_at.md) |
