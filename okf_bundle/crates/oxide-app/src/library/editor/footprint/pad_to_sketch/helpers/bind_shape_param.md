---
okf_version: "0.2"
type: Function
title: bind_shape_param
description: "Bind a canonical shape-parameter key (e.g. `\"corner_r\"`,"
resource: crates/oxide-app/src/library/editor/footprint/pad_to_sketch/helpers.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:48Z"
concept_id: crates/oxide-app/src/library/editor/footprint/pad_to_sketch/helpers/bind_shape_param
language: rust
---

# bind_shape_param

Bind a canonical shape-parameter key (e.g. `"corner_r"`,

## Signature

```rust
pub(super) fn bind_shape_param(
    sketch: &mut SketchData,
    pad: &mut EditorPad,
    key: &str,
    centre_id: SketchEntityId,
    value_mm: f64,
) -> String
```

## Visibility

- `pub(super)`

## Docstring

Bind a canonical shape-parameter key (e.g. `"corner_r"`,
`"diameter"`) to a freshly-named sketch parameter at value `expr`,
recording the binding on `pad.shape_params`. Returns the generated
parameter name (`"<key>_<centre-uuid-slug>"`).

## Source
Lines 130–145 in `crates/oxide-app/src/library/editor/footprint/pad_to_sketch/helpers.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [helpers](/crates/oxide-app/src/library/editor/footprint/pad_to_sketch/helpers.md) |
| calls | [id_slug](/crates/oxide-app/src/library/editor/footprint/pad_to_sketch/attr/id_slug.md) |
| called_by | [mint_chamfered_pad_geometry](/crates/oxide-app/src/library/editor/footprint/pad_to_sketch/mint/mint_chamfered_pad_geometry.md) |
| called_by | [mint_oval_pad_geometry](/crates/oxide-app/src/library/editor/footprint/pad_to_sketch/mint/mint_oval_pad_geometry.md) |
| called_by | [mint_round_pad_geometry](/crates/oxide-app/src/library/editor/footprint/pad_to_sketch/mint/mint_round_pad_geometry.md) |
| called_by | [mint_round_rect_pad_geometry](/crates/oxide-app/src/library/editor/footprint/pad_to_sketch/mint/mint_round_rect_pad_geometry.md) |
