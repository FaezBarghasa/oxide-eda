---
okf_version: "0.2"
type: Function
title: fp_parse_optional_mm
description: "v0.20 — parse a Properties-panel mm input as `Option<f64>`."
resource: crates/oxide-app/src/app/handlers/dock/sch_library/mod.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/app/handlers/dock/sch_library/mod/fp_parse_optional_mm
language: rust
---

# fp_parse_optional_mm

v0.20 — parse a Properties-panel mm input as `Option<f64>`.

## Signature

```rust
pub(super) fn fp_parse_optional_mm(value: &str) -> Option<f64>
```

## Visibility

- `pub(super)`

## Docstring

v0.20 — parse a Properties-panel mm input as `Option<f64>`.
Empty / whitespace = `None` (means "use rule"); non-numeric =
`None` (the form re-displays the previous value, so the user
can keep typing). Used by every per-side mask / paste row.

## Source
Lines 771–777 in `crates/oxide-app/src/app/handlers/dock/sch_library/mod.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [sch_library](/crates/oxide-app/src/app/handlers/dock/sch_library/mod.md) |
| called_by | [fp_editor_set_next_pad_drill_diameter](/crates/oxide-app/src/app/handlers/dock/sch_library/footprint/pad/fp_editor_set_next_pad_drill_diameter.md) |
| called_by | [fp_editor_set_next_pad_drill_slot_length](/crates/oxide-app/src/app/handlers/dock/sch_library/footprint/pad/fp_editor_set_next_pad_drill_slot_length.md) |
| called_by | [fp_editor_set_next_pad_mask_margin_bottom](/crates/oxide-app/src/app/handlers/dock/sch_library/footprint/pad/fp_editor_set_next_pad_mask_margin_bottom.md) |
| called_by | [fp_editor_set_next_pad_mask_margin_top](/crates/oxide-app/src/app/handlers/dock/sch_library/footprint/pad/fp_editor_set_next_pad_mask_margin_top.md) |
| called_by | [fp_editor_set_next_pad_paste_margin_bottom](/crates/oxide-app/src/app/handlers/dock/sch_library/footprint/pad/fp_editor_set_next_pad_paste_margin_bottom.md) |
| called_by | [fp_editor_set_next_pad_paste_margin_top](/crates/oxide-app/src/app/handlers/dock/sch_library/footprint/pad/fp_editor_set_next_pad_paste_margin_top.md) |
| called_by | [fp_editor_set_selected_pad_drill_diameter](/crates/oxide-app/src/app/handlers/dock/sch_library/footprint/pad/fp_editor_set_selected_pad_drill_diameter.md) |
| called_by | [fp_editor_set_selected_pad_mask_margin_bottom](/crates/oxide-app/src/app/handlers/dock/sch_library/footprint/pad/fp_editor_set_selected_pad_mask_margin_bottom.md) |
| called_by | [fp_editor_set_selected_pad_mask_margin_top](/crates/oxide-app/src/app/handlers/dock/sch_library/footprint/pad/fp_editor_set_selected_pad_mask_margin_top.md) |
| called_by | [fp_editor_set_selected_pad_paste_margin_bottom](/crates/oxide-app/src/app/handlers/dock/sch_library/footprint/pad/fp_editor_set_selected_pad_paste_margin_bottom.md) |
| called_by | [fp_editor_set_selected_pad_paste_margin_top](/crates/oxide-app/src/app/handlers/dock/sch_library/footprint/pad/fp_editor_set_selected_pad_paste_margin_top.md) |
| called_by | [handle_fp_editor_set_next_pad_copper_offset_x](/crates/oxide-app/src/app/handlers/dock/sch_library/footprint/pad/handle_fp_editor_set_next_pad_copper_offset_x.md) |
| called_by | [handle_fp_editor_set_next_pad_copper_offset_y](/crates/oxide-app/src/app/handlers/dock/sch_library/footprint/pad/handle_fp_editor_set_next_pad_copper_offset_y.md) |
| called_by | [handle_fp_editor_set_next_pad_hole_tolerance_minus](/crates/oxide-app/src/app/handlers/dock/sch_library/footprint/pad/handle_fp_editor_set_next_pad_hole_tolerance_minus.md) |
| called_by | [handle_fp_editor_set_next_pad_hole_tolerance_plus](/crates/oxide-app/src/app/handlers/dock/sch_library/footprint/pad/handle_fp_editor_set_next_pad_hole_tolerance_plus.md) |
| called_by | [handle_fp_editor_set_footprint_height](/crates/oxide-app/src/app/handlers/dock/sch_library/footprint/props/handle_fp_editor_set_footprint_height.md) |
| called_by | [handle_dock_sch_library_message](/crates/oxide-app/src/app/handlers/dock/sch_library/mod/handle_dock_sch_library_message.md) |
