---
okf_version: "0.2"
type: Class
title: FootprintFile
description: "Multi-footprint container for `.snxfpt` files — Altium PCB"
resource: crates/oxide-library/src/primitive/footprint/mod.rs
tags:
  - "lang:rust"
  - "type:Class"
  - "module:crates"
  - "domain:oxide-library"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-library/src/primitive/footprint/mod/FootprintFile
language: rust
---

# FootprintFile

Multi-footprint container for `.snxfpt` files — Altium PCB

## Signature

```rust
pub struct FootprintFile
```

## Decorators

- `derive(Clone, Debug, PartialEq, Serialize, Deserialize)`

## Visibility

- `pub`

## Docstring

Multi-footprint container for `.snxfpt` files — Altium PCB
Library parity. One file holds many footprints; each footprint
still has its own UUID for `PrimitiveRef` resolution.

Wire format (v0.18.4): TOML manifest header + one `[[footprints]]`
array entry per Footprint. Each entry's bulk pad list is embedded
as a TSV literal multi-line string (`pads_tsv = '''…'''`) — line-
diffable in git, editable in any spreadsheet. Graphics
(silk/fab/courtyard), 3D body, sketch, pours, keepouts, cutouts,
v-scores, mask openings/excludes, and paste apertures stay as
inline TOML since they're variant-shaped or sparse.
[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]

## Methods

- `format`
- `file_uuid`
- `display_name`
- `footprints`
- `created`
- `updated`

## Source
Lines 473–489 in `crates/oxide-library/src/primitive/footprint/mod.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [footprint](/crates/oxide-library/src/primitive/footprint/mod.md) |
