---
okf_version: "0.2"
type: Function
title: from_sheet
resource: crates/oxide-output/src/svg/document.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-output"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:49Z"
concept_id: crates/oxide-output/src/svg/document/from_sheet
language: rust
---

# from_sheet

## Signature

```rust
impl SvgRenderContext { pub fn from_sheet(
        sheet: &SheetSnapshot,
        opts: &PdfOptions,
        page_w_mm: f64,
        page_h_mm: f64,
        units_per_mm: f64,
        eval_inputs: Option<&SvgEvaluatorInputs<'_>>,
    ) -> Self }
```

## Visibility

- `pub`

## Source
Lines 32–386 in `crates/oxide-output/src/svg/document.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [document](/crates/oxide-output/src/svg/document.md) |
| calls | [rect_path](/crates/oxide-output/src/svg/geometry/rect_path.md) |
| calls | [label_size_pt](/crates/oxide-output/src/svg/labels/label_size_pt.md) |
| calls | [label_spin_style](/crates/oxide-output/src/svg/labels/label_spin_style.md) |
| calls | [schematic_text_offset_net](/crates/oxide-output/src/svg/labels/schematic_text_offset_net.md) |
| calls | [schematic_text_offset_global](/crates/oxide-output/src/svg/labels/schematic_text_offset_global.md) |
| calls | [schematic_text_offset_hier](/crates/oxide-output/src/svg/labels/schematic_text_offset_hier.md) |
| calls | [spin_text_style](/crates/oxide-output/src/svg/labels/spin_text_style.md) |
| calls | [label_colour](/crates/oxide-output/src/svg/labels/label_colour.md) |
| calls | [halign_to_svg](/crates/oxide-output/src/svg/labels/halign_to_svg.md) |
| calls | [valign_to_svg](/crates/oxide-output/src/svg/labels/valign_to_svg.md) |
| calls | [push_sch_drawing_path](/crates/oxide-output/src/svg/drawings/push_sch_drawing_path.md) |
| calls | [symbol_eval_variables](/crates/oxide-output/src/svg/symbols/symbol_eval_variables.md) |
| calls | [push_symbol_lib_graphics](/crates/oxide-output/src/svg/symbols/push_symbol_lib_graphics.md) |
| calls | [push_symbol_pins](/crates/oxide-output/src/svg/symbols/push_symbol_pins.md) |
| calls | [field_effective_style](/crates/oxide-output/src/svg/symbols/field_effective_style.md) |
| calls | [normalize_standard_text_with_ctx](/crates/oxide-output/src/svg/mod/normalize_standard_text_with_ctx.md) |
| calls | [encode_svg_document](/crates/oxide-output/src/svg/document/encode_svg_document.md) |
