---
okf_version: "0.2"
type: Function
title: with_selected_pad
description: v0.20 — Selected-pad editing handlers. Each one mutates a slice
resource: crates/oxide-app/src/app/handlers/dock/sch_library/footprint/pad.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/app/handlers/dock/sch_library/footprint/pad/with_selected_pad
language: rust
---

# with_selected_pad

v0.20 — Selected-pad editing handlers. Each one mutates a slice

## Signature

```rust
impl Oxide { pub(in crate::app::handlers::dock::sch_library) fn with_selected_pad(
        &mut self,
        idx: usize,
        f: F,
    ) -> bool }
```

## Type Parameters

- `F`

## Visibility

- `pub(in crate::app::handlers::dock::sch_library)`

## Docstring

v0.20 — Selected-pad editing handlers. Each one mutates a slice
of `state.pads[idx]` and dirty-marks the editor + clears the
canvas cache so the new value renders. The `with_parts` block
syncs the pad list back onto the underlying primitive so the
saved file picks up the change.

## Source
Lines 321–371 in `crates/oxide-app/src/app/handlers/dock/sch_library/footprint/pad.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [pad](/crates/oxide-app/src/app/handlers/dock/sch_library/footprint/pad.md) |
| calls | [remint_pad_geometry](/crates/oxide-app/src/library/editor/footprint/pad_to_sketch/mod/remint_pad_geometry.md) |
| calls | [warn_profile_pad_untransformed](/crates/oxide-app/src/library/editor/footprint/pad_to_sketch/mod/warn_profile_pad_untransformed.md) |
| calls | [mirror_move_pad_in_sketch](/crates/oxide-app/src/library/editor/footprint/pad_to_sketch/mod/mirror_move_pad_in_sketch.md) |
