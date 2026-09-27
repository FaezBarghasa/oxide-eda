---
okf_version: "0.2"
type: Function
title: apply
description: Apply transform to a library-space point.
resource: crates/oxide-types/src/schematic/mod.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-types"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-14T10:15:42Z"
concept_id: crates/oxide-types/src/schematic/mod/apply
language: rust
---

# apply

Apply transform to a library-space point.

## Signature

```rust
impl SymbolTransform { pub fn apply(&self, local: Point) -> Point }
```

## Visibility

- `pub`

## Docstring

Apply transform to a library-space point.
[must_use]

## Source
Lines 265–280 in `crates/oxide-types/src/schematic/mod.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [schematic](/crates/oxide-types/src/schematic/mod.md) |
