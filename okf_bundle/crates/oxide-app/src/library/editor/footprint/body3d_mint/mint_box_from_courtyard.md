---
okf_version: "0.2"
type: Function
title: mint_box_from_courtyard
description: "\"3D Body\" — extrude the courtyard outline into a solid box. Height and"
resource: crates/oxide-app/src/library/editor/footprint/body3d_mint.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:48Z"
concept_id: crates/oxide-app/src/library/editor/footprint/body3d_mint/mint_box_from_courtyard
language: rust
---

# mint_box_from_courtyard

"3D Body" — extrude the courtyard outline into a solid box. Height and

## Signature

```rust
pub fn mint_box_from_courtyard(fp: &mut Footprint)
```

## Visibility

- `pub`

## Docstring

"3D Body" — extrude the courtyard outline into a solid box. Height and
colours come from `Body3D::default()` (no magic constants).

## Source
Lines 10–17 in `crates/oxide-app/src/library/editor/footprint/body3d_mint.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [body3d_mint](/crates/oxide-app/src/library/editor/footprint/body3d_mint.md) |
| called_by | [mint_body3d_extrudes_courtyard](/crates/oxide-app/src/library/editor/footprint/tests/mint_body3d_extrudes_courtyard.md) |
| called_by | [mint_body3d](/crates/oxide-app/src/library/editor/footprint/updates/geometry/mint_body3d.md) |
