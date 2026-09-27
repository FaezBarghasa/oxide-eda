---
okf_version: "0.2"
type: Module
title: to_lib_symbol
description: "Pure conversion: a library-authored [`Symbol`] (`.snxsym`) into a"
resource: crates/oxide-library/src/primitive/symbol/to_lib_symbol.rs
tags:
  - "lang:rust"
  - "type:Module"
  - "module:crates"
  - "domain:oxide-library"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:49Z"
concept_id: crates/oxide-library/src/primitive/symbol/to_lib_symbol
language: rust
---

# to_lib_symbol

Pure conversion: a library-authored [`Symbol`] (`.snxsym`) into a

## Docstring

Pure conversion: a library-authored [`Symbol`] (`.snxsym`) into a
schematic [`LibSymbol`] (`oxide_types::schematic`).

This is **part 1 of 2** of issue #365 — no I/O, no app dependency, and
nothing here wires the result into the place flow or `PlaceSymbol`
(that is part 2). `LibSymbol::id` is a caller-supplied argument; this
module does not invent an id scheme.

## Coordinates — no y-flip here

`.snxsym` positions are y-up mm (`crates/oxide-app/src/library/editor/
symbol/canvas/geometry.rs`), and `LibPin.pin.position` / `LibGraphic`
points are already y-up library space too — the flip to y-down
schematic space happens later, only inside
[`oxide_types::schematic::SymbolTransform::apply`]. Every position
here carries over unchanged.

## Pin rotation convention (both sides agree — identity mapping)

`.snxsym`'s [`PinOrientation`] is the tip→body direction as a CCW angle
from +x in that same y-up space: `Right = 0°`, `Up = +90°`,
`Left = 180°`, `Down = -90°` — see `PinRenderGeometry::compute` in
`crates/oxide-app/src/library/editor/symbol/canvas/pins.rs`, where
`tip = pin.position` and `body_end = tip + unit(angle_rad) * length`.

`LibPin.pin.rotation` is read by every real consumer as the SAME
y-up, CCW-from-+x, tip→body angle, with the single y-flip applied
later — never here — by
[`oxide_types::schematic::SymbolTransform::apply`] (`y = -local.y`).
`crates/oxide-engine/src/transform/autoplace.rs` derives a pin's far
end as `(sx + length*cos(rotation), sy + length*sin(rotation))` in
this same pre-flip local space before handing the point to
`transform_local_point`, and the SVG/PDF exporter's `pin_direction`
(`crates/oxide-output/src/svg/symbols.rs`) maps `90° => (0.0, 1.0)`,
`270° => (0.0, -1.0)` — both agree `90°` means "toward +y" in
pre-flip space, matching the source convention exactly. Since
positions are not flipped either (previous section), the faithful
conversion is the identity on angle: `Right -> 0°`, `Up -> 90°`,
`Left -> 180°`, `Down -> 270°`. [`to_lib_symbol_tests`] pins all four.

## Pin direction — total, no panic

[`PinDirection`] (10 variants, symbol-editor-curated) and
`oxide_types::schematic::PinDirection` (14 variants, independently
curated) are two separate sets with no exact 1:1 twin for several
source variants. [`pin_direction`] documents every non-obvious choice
inline; every source variant is pinned by a test.

## Pin glyphs — lossy by construction

`.snxsym` carries four independent glyph slots per pin
(`inside_symbol`, `inside_edge_symbol`, `outside_edge_symbol`,
`outside_symbol`), each a 15-variant [`PinSymbolKind`]. The schematic
side has one flat 7-variant `PinShapeStyle` per pin. `outside_edge_symbol`
is chosen as the authoritative slot — see [`pin_shape_style`].

## Fill — colour is dropped

`SymbolGraphic::fill: Option<[u8; 4]>` becomes `FillType`, which
carries no colour: `None -> FillType::None`, `Some(_) ->
FillType::Background`. The RGBA value itself has nowhere to land.

## Relationships

| Type | Target |
|------|--------|
| related | [to_lib_symbol](/crates/oxide-library/src/primitive/symbol/to_lib_symbol/to_lib_symbol.md) |
| related | [to_lib_symbol](/crates/oxide-library/src/primitive/symbol/to_lib_symbol/to_lib_symbol.md) |
| related | [lib_pin_from](/crates/oxide-library/src/primitive/symbol/to_lib_symbol/lib_pin_from.md) |
| related | [pin_rotation_deg](/crates/oxide-library/src/primitive/symbol/to_lib_symbol/pin_rotation_deg.md) |
| related | [pin_direction](/crates/oxide-library/src/primitive/symbol/to_lib_symbol/pin_direction.md) |
| related | [pin_shape_style](/crates/oxide-library/src/primitive/symbol/to_lib_symbol/pin_shape_style.md) |
| related | [lib_graphic_from](/crates/oxide-library/src/primitive/symbol/to_lib_symbol/lib_graphic_from.md) |
| related | [fill_type](/crates/oxide-library/src/primitive/symbol/to_lib_symbol/fill_type.md) |
| related | [point_at_deg](/crates/oxide-library/src/primitive/symbol/to_lib_symbol/point_at_deg.md) |
| related | [arc_points](/crates/oxide-library/src/primitive/symbol/to_lib_symbol/arc_points.md) |
| related | [to_graphic](/crates/oxide-library/src/primitive/symbol/to_lib_symbol/to_graphic.md) |
