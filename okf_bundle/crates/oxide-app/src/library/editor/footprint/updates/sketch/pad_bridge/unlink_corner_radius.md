---
okf_version: "0.2"
type: Function
title: unlink_corner_radius
description: "v0.24 Phase 3 (Track A3) — split a RoundRect pad's shared"
resource: crates/oxide-app/src/library/editor/footprint/updates/sketch/pad_bridge.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/library/editor/footprint/updates/sketch/pad_bridge/unlink_corner_radius
language: rust
---

# unlink_corner_radius

v0.24 Phase 3 (Track A3) — split a RoundRect pad's shared

## Signature

```rust
fn unlink_corner_radius(
    editor: &mut crate::app::FootprintEditorState,
    arc_entity_id: oxide_sketch::id::SketchEntityId,
)
```

## Docstring

v0.24 Phase 3 (Track A3) — split a RoundRect pad's shared
`corner_r_<slug>` parameter into a per-corner override for the
right-clicked Arc.

Lookup chain:
1. Walk every EditorPad to find the one whose `shape_params`
contains a `corner_r_<corner>_arc` key whose value (UUID slug)
matches `arc_entity_id`.
2. From that match, derive the corner key (`corner_r_ne` / `_se` /
`_sw` / `_nw`).
3. Mint a fresh parameter `<shared_name>_<corner>`, copy the current
shared expression as its value, and bind the corner key on
`pad.shape_params`.
4. Trigger a `ForceRebuild` so the solver re-runs and the bake
reflects the new parametric link.

Defensive: arc not part of any pad → tracing::warn + no-op. Pad has no
shared `corner_r` binding (e.g. legacy data) → tracing::warn + no-op.

## Source
Lines 408–508 in `crates/oxide-app/src/library/editor/footprint/updates/sketch/pad_bridge.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [pad_bridge](/crates/oxide-app/src/library/editor/footprint/updates/sketch/pad_bridge.md) |
| calls | [apply_sketch_edit_with_warnings](/crates/oxide-app/src/library/editor/footprint/sketch_dispatch/apply_sketch_edit_with_warnings.md) |
| called_by | [apply](/crates/oxide-app/src/library/editor/footprint/updates/sketch/pad_bridge/apply.md) |
