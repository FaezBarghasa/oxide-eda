---
okf_version: "0.2"
type: Class
title: SilkKindGeometry
description: "v0.21 — per-`FpGraphicKind` editable geometry. We surface"
resource: crates/oxide-app/src/panels/footprint_context.rs
tags:
  - "lang:rust"
  - "type:Class"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:48Z"
concept_id: crates/oxide-app/src/panels/footprint_context/SilkKindGeometry
language: rust
---

# SilkKindGeometry

v0.21 — per-`FpGraphicKind` editable geometry. We surface

## Signature

```rust
pub enum SilkKindGeometry
```

## Decorators

- `derive(Debug, Clone)`

## Visibility

- `pub`

## Docstring

v0.21 — per-`FpGraphicKind` editable geometry. We surface
dedicated editable forms only for Line (Track) and Text (String);
Arc / Rectangle / Circle / Polygon collapse to `Other` and get a
minimal banner pointing the user at sketch mode for parametric
editing.
[derive(Debug, Clone)]

## Methods

- `from_mm`
- `to_mm`
- `position_mm`
- `content`
- `size_mm`

## Source
Lines 598–612 in `crates/oxide-app/src/panels/footprint_context.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [footprint_context](/crates/oxide-app/src/panels/footprint_context.md) |
