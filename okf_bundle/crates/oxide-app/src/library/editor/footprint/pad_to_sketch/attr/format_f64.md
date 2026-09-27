---
okf_version: "0.2"
type: Function
title: format_f64
description: "Format a float with up to 4 fractional digits, trimming trailing"
resource: crates/oxide-app/src/library/editor/footprint/pad_to_sketch/attr.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:48Z"
concept_id: crates/oxide-app/src/library/editor/footprint/pad_to_sketch/attr/format_f64
language: rust
---

# format_f64

Format a float with up to 4 fractional digits, trimming trailing

## Signature

```rust
pub(super) fn format_f64(v: f64) -> String
```

## Visibility

- `pub(super)`

## Docstring

Format a float with up to 4 fractional digits, trimming trailing
zeros. Keeps the generated expression strings readable
(e.g. `1.5` rather than `1.5000000000000`).

## Source
Lines 226–234 in `crates/oxide-app/src/library/editor/footprint/pad_to_sketch/attr.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [attr](/crates/oxide-app/src/library/editor/footprint/pad_to_sketch/attr.md) |
| called_by | [map_shape](/crates/oxide-app/src/library/editor/footprint/pad_to_sketch/attr/map_shape.md) |
