---
okf_version: "0.2"
type: Function
title: graphic_geometry_center
resource: crates/oxide-app/src/library/editor/symbol/state/rotation.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:04:17Z"
concept_id: crates/oxide-app/src/library/editor/symbol/state/rotation/graphic_geometry_center
language: rust
---

# graphic_geometry_center

## Signature

```rust
fn graphic_geometry_center(kind: &SymbolGraphicKind) -> [f64; 2]
```

## Source
Lines 155–164 in `crates/oxide-app/src/library/editor/symbol/state/rotation.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [rotation](/crates/oxide-app/src/library/editor/symbol/state/rotation.md) |
| calls | [polygon_centroid](/crates/oxide-app/src/library/editor/symbol/state/mod/polygon_centroid.md) |
| called_by | [rotate_graphic_90](/crates/oxide-app/src/library/editor/symbol/state/rotation/rotate_graphic_90.md) |
