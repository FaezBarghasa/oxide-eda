---
okf_version: "0.2"
type: Function
title: with_selected_sketch_pad
description: "v0.21 — sketch-mode counterpart of `with_selected_pad`. Looks"
resource: crates/oxide-app/src/app/handlers/dock/sch_library/footprint/pad.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/app/handlers/dock/sch_library/footprint/pad/with_selected_sketch_pad_1
language: rust
---

# with_selected_sketch_pad

v0.21 — sketch-mode counterpart of `with_selected_pad`. Looks

## Signature

```rust
pub(in crate::app::handlers::dock::sch_library) fn with_selected_sketch_pad(
        &mut self,
        id: oxide_sketch::id::SketchEntityId,
        f: F,
    ) -> bool
```

## Type Parameters

- `F`

## Visibility

- `pub(in crate::app::handlers::dock::sch_library)`

## Docstring

v0.21 — sketch-mode counterpart of `with_selected_pad`. Looks
up the sketch entity by id, runs the closure on its `PadAttr`
(creating one only if it already exists; non-pad entities are
silently skipped), then dirty-marks the editor + clears the
canvas cache. Solve+bake is queued on the next mutation cycle.

## Source
Lines 378–399 in `crates/oxide-app/src/app/handlers/dock/sch_library/footprint/pad.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [pad](/crates/oxide-app/src/app/handlers/dock/sch_library/footprint/pad.md) |
| calls | [find](/crates/oxide-app/src/library/editor/symbol/context_menu/rows/find.md) |
