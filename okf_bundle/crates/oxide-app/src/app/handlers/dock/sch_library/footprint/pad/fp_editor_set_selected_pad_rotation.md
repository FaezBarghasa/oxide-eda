---
okf_version: "0.2"
type: Function
title: fp_editor_set_selected_pad_rotation
resource: crates/oxide-app/src/app/handlers/dock/sch_library/footprint/pad.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/app/handlers/dock/sch_library/footprint/pad/fp_editor_set_selected_pad_rotation
language: rust
---

# fp_editor_set_selected_pad_rotation

## Signature

```rust
impl Oxide { pub(crate) fn fp_editor_set_selected_pad_rotation(
        &mut self,
        idx: usize,
        value: String,
    ) -> bool }
```

## Visibility

- `pub(crate)`

## Source
Lines 98–132 in `crates/oxide-app/src/app/handlers/dock/sch_library/footprint/pad.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [pad](/crates/oxide-app/src/app/handlers/dock/sch_library/footprint/pad.md) |
| calls | [remint_pad_geometry](/crates/oxide-app/src/library/editor/footprint/pad_to_sketch/mod/remint_pad_geometry.md) |
| calls | [warn_profile_pad_untransformed](/crates/oxide-app/src/library/editor/footprint/pad_to_sketch/mod/warn_profile_pad_untransformed.md) |
