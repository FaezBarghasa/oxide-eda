---
okf_version: "0.2"
type: Module
title: pins
description: "Pin-rendering helpers — the sheet-derived colour palette, the pin"
resource: crates/oxide-app/src/library/editor/symbol/canvas/pins.rs
tags:
  - "lang:rust"
  - "type:Module"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:48Z"
concept_id: crates/oxide-app/src/library/editor/symbol/canvas/pins
language: rust
---

# pins

Pin-rendering helpers — the sheet-derived colour palette, the pin

## Docstring

Pin-rendering helpers — the sheet-derived colour palette, the pin
text layout constants, and the per-pin render geometry. Pure code
motion out of `mod.rs`; consumed by `build_symbol_renderer_snapshot`
and `SymbolCanvas::new` (both still in the parent `canvas` module),
so the types carry `pub(super)` visibility.

## Relationships

| Type | Target |
|------|--------|
| related | [SymbolPalette](/crates/oxide-app/src/library/editor/symbol/canvas/pins/SymbolPalette.md) |
| related | [for_sheet](/crates/oxide-app/src/library/editor/symbol/canvas/pins/for_sheet.md) |
| related | [for_sheet](/crates/oxide-app/src/library/editor/symbol/canvas/pins/for_sheet.md) |
| related | [PinTextLayout](/crates/oxide-app/src/library/editor/symbol/canvas/pins/PinTextLayout.md) |
| related | [Aabb](/crates/oxide-app/src/library/editor/symbol/canvas/pins/Aabb.md) |
| related | [contains](/crates/oxide-app/src/library/editor/symbol/canvas/pins/contains.md) |
| related | [empty](/crates/oxide-app/src/library/editor/symbol/canvas/pins/empty.md) |
| related | [contains](/crates/oxide-app/src/library/editor/symbol/canvas/pins/contains.md) |
| related | [empty](/crates/oxide-app/src/library/editor/symbol/canvas/pins/empty.md) |
| related | [PinRenderGeometry](/crates/oxide-app/src/library/editor/symbol/canvas/pins/PinRenderGeometry.md) |
| related | [compute](/crates/oxide-app/src/library/editor/symbol/canvas/pins/compute.md) |
| related | [label_hit_boxes](/crates/oxide-app/src/library/editor/symbol/canvas/pins/label_hit_boxes.md) |
| related | [text_box](/crates/oxide-app/src/library/editor/symbol/canvas/pins/text_box.md) |
| related | [compute](/crates/oxide-app/src/library/editor/symbol/canvas/pins/compute.md) |
| related | [label_hit_boxes](/crates/oxide-app/src/library/editor/symbol/canvas/pins/label_hit_boxes.md) |
| related | [text_box](/crates/oxide-app/src/library/editor/symbol/canvas/pins/text_box.md) |
| related | [label_boxes_grab_their_anchor_and_reject_far_points](/crates/oxide-app/src/library/editor/symbol/canvas/pins/label_boxes_grab_their_anchor_and_reject_far_points.md) |
| related | [empty_label_box_never_hits](/crates/oxide-app/src/library/editor/symbol/canvas/pins/empty_label_box_never_hits.md) |
