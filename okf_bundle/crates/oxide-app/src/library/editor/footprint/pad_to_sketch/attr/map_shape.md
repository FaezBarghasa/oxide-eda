---
okf_version: "0.2"
type: Function
title: map_shape
resource: crates/oxide-app/src/library/editor/footprint/pad_to_sketch/attr.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:48Z"
concept_id: crates/oxide-app/src/library/editor/footprint/pad_to_sketch/attr/map_shape
language: rust
---

# map_shape

## Signature

```rust
pub(super) fn map_shape(s: &LibPadShape) -> SkPadShape
```

## Visibility

- `pub(super)`

## Source
Lines 191–221 in `crates/oxide-app/src/library/editor/footprint/pad_to_sketch/attr.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [attr](/crates/oxide-app/src/library/editor/footprint/pad_to_sketch/attr.md) |
| calls | [format_f64](/crates/oxide-app/src/library/editor/footprint/pad_to_sketch/attr/format_f64.md) |
| called_by | [mirror_shape](/crates/oxide-app/src/library/editor/footprint/pad_to_sketch/attr/mirror_shape.md) |
| called_by | [pad_attr_from_editor_pad](/crates/oxide-app/src/library/editor/footprint/pad_to_sketch/attr/pad_attr_from_editor_pad.md) |
