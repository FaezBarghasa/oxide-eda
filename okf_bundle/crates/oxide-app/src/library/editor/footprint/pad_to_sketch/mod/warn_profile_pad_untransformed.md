---
okf_version: "0.2"
type: Function
title: warn_profile_pad_untransformed
description: "A sketch-profile pad's copper is a traced loop, not a parametric"
resource: crates/oxide-app/src/library/editor/footprint/pad_to_sketch/mod.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:48Z"
concept_id: crates/oxide-app/src/library/editor/footprint/pad_to_sketch/mod/warn_profile_pad_untransformed
language: rust
---

# warn_profile_pad_untransformed

A sketch-profile pad's copper is a traced loop, not a parametric

## Signature

```rust
pub fn warn_profile_pad_untransformed(op: &str, pad_number: &str)
```

## Visibility

- `pub`

## Docstring

A sketch-profile pad's copper is a traced loop, not a parametric
shape — there is nothing on the pad for a frame transform to ride
on, and the loop stays exactly as drawn. This is the warning
[`remint_pad_geometry`]'s `false` return obliges its caller to
emit, kept next to the function that owes it so every caller says
the same thing.

## Source
Lines 411–416 in `crates/oxide-app/src/library/editor/footprint/pad_to_sketch/mod.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [pad_to_sketch](/crates/oxide-app/src/library/editor/footprint/pad_to_sketch/mod.md) |
| calls | [log_warning](/crates/oxide-app/src/diagnostics/log_warning.md) |
| called_by | [fp_editor_set_selected_pad_rotation](/crates/oxide-app/src/app/handlers/dock/sch_library/footprint/pad/fp_editor_set_selected_pad_rotation.md) |
| called_by | [with_selected_pad](/crates/oxide-app/src/app/handlers/dock/sch_library/footprint/pad/with_selected_pad.md) |
| called_by | [active_bar_flip_selection](/crates/oxide-app/src/library/editor/footprint/updates/active_bar/active_bar_flip_selection.md) |
| called_by | [active_bar_rotate_selection](/crates/oxide-app/src/library/editor/footprint/updates/active_bar/active_bar_rotate_selection.md) |
| called_by | [remint_dragged_pad](/crates/oxide-app/src/library/editor/footprint/updates/sketch/entities/remint_dragged_pad.md) |
