---
okf_version: "0.2"
type: Function
title: align_pads
description: "v0.14 — apply an [`AlignOp`] to the pads at `indices` in `state`,"
resource: crates/oxide-app/src/library/editor/footprint/updates/mod.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:48Z"
concept_id: crates/oxide-app/src/library/editor/footprint/updates/mod/align_pads
language: rust
---

# align_pads

v0.14 — apply an [`AlignOp`] to the pads at `indices` in `state`,

## Signature

```rust
fn align_pads(
    state: &mut crate::library::editor::footprint::state::FootprintEditorState,
    indices: &[usize],
    op: crate::library::editor::footprint::state::AlignOp,
    step: f64,
)
```

## Docstring

v0.14 — apply an [`AlignOp`] to the pads at `indices` in `state`,
in place. `step` is the spacing increment (mm) for the
Increase/Decrease ops — pass the active grid step. Centre-based
throughout: a pad's "position" is its centre, so aligning edges and
aligning centres coincide once sizes are equal; for mixed sizes we
follow Altium's pad-centre convention (the Properties X/Y is the
centre). Callers guarantee `indices` is deduped, in range, and long
enough (≥2 for align, ≥3 for distribute).

[`AlignOp`]: crate::library::editor::footprint::state::AlignOp

## Source
Lines 33–47 in `crates/oxide-app/src/library/editor/footprint/updates/mod.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [updates](/crates/oxide-app/src/library/editor/footprint/updates/mod.md) |
| calls | [apply_align](/crates/oxide-app/src/library/editor/footprint/updates/mod/apply_align.md) |
| called_by | [align_confirm](/crates/oxide-app/src/library/editor/footprint/updates/active_bar/align_confirm.md) |
| called_by | [align_pads_action](/crates/oxide-app/src/library/editor/footprint/updates/view/align_pads_action.md) |
