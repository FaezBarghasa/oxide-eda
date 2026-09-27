---
okf_version: "0.2"
type: Class
title: SymbolTransform
description: World-space placement of a parent symbol — used when folding a
resource: crates/oxide-types/src/schematic/mod.rs
tags:
  - "lang:rust"
  - "type:Class"
  - "module:crates"
  - "domain:oxide-types"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-14T10:15:42Z"
concept_id: crates/oxide-types/src/schematic/mod/SymbolTransform
language: rust
---

# SymbolTransform

World-space placement of a parent symbol — used when folding a

## Signature

```rust
pub struct SymbolTransform
```

## Decorators

- `derive(Debug, Clone, Copy, PartialEq)`

## Visibility

- `pub`

## Docstring

World-space placement of a parent symbol — used when folding a
child element's library-space rotation/mirror with the parent's
transform.

The transform is `Y-up library` → `Y-down schematic`: a library-space
pin at `(0, +pin_length)` lands at world position `(0, -pin_length)`
relative to the parent body when the parent has no rotation or mirror.
[derive(Debug, Clone, Copy, PartialEq)]

## Methods

- `origin`
- `rotation_deg`
- `mirror_x`
- `mirror_y`

## Source
Lines 244–249 in `crates/oxide-types/src/schematic/mod.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [schematic](/crates/oxide-types/src/schematic/mod.md) |
