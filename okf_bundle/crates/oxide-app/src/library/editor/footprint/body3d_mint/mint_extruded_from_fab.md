---
okf_version: "0.2"
type: Function
title: mint_extruded_from_fab
description: "\"Extruded 3D Body\" — extrude with no explicit outline so preview3d /"
resource: crates/oxide-app/src/library/editor/footprint/body3d_mint.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:48Z"
concept_id: crates/oxide-app/src/library/editor/footprint/body3d_mint/mint_extruded_from_fab
language: rust
---

# mint_extruded_from_fab

"Extruded 3D Body" — extrude with no explicit outline so preview3d /

## Signature

```rust
pub fn mint_extruded_from_fab(fp: &mut Footprint)
```

## Visibility

- `pub`

## Docstring

"Extruded 3D Body" — extrude with no explicit outline so preview3d /
bake fall back to the fab outline (preview3d.rs:290 handles `None`).

## Source
Lines 21–27 in `crates/oxide-app/src/library/editor/footprint/body3d_mint.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [body3d_mint](/crates/oxide-app/src/library/editor/footprint/body3d_mint.md) |
| called_by | [mint_extruded_body3d](/crates/oxide-app/src/library/editor/footprint/updates/geometry/mint_extruded_body3d.md) |
