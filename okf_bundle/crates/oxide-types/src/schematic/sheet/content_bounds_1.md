---
okf_version: "0.2"
type: Function
title: content_bounds
description: Compute the bounding box of all elements in the sheet.
resource: crates/oxide-types/src/schematic/sheet.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-types"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:49Z"
concept_id: crates/oxide-types/src/schematic/sheet/content_bounds_1
language: rust
---

# content_bounds

Compute the bounding box of all elements in the sheet.

## Signature

```rust
pub fn content_bounds(&self) -> Option<Aabb>
```

## Visibility

- `pub`

## Docstring

Compute the bounding box of all elements in the sheet.

## Source
Lines 354–399 in `crates/oxide-types/src/schematic/sheet.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [sheet](/crates/oxide-types/src/schematic/sheet.md) |
