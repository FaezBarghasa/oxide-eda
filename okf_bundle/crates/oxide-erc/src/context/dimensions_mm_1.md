---
okf_version: "0.2"
type: Function
title: dimensions_mm
description: "Returns `(width_mm, height_mm)` for **landscape** orientation"
resource: crates/oxide-erc/src/context.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-erc"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:49Z"
concept_id: crates/oxide-erc/src/context/dimensions_mm_1
language: rust
---

# dimensions_mm

Returns `(width_mm, height_mm)` for **landscape** orientation

## Signature

```rust
pub fn dimensions_mm(self) -> (f64, f64)
```

## Visibility

- `pub`

## Docstring

Returns `(width_mm, height_mm)` for **landscape** orientation
(long side first). LO-5: schematic ERC currently treats every
sheet as landscape; if the schema ever grows an explicit
orientation flag (e.g. `A4_L` vs `A4`), the consumer should
swap the tuple instead of expecting this method to do it.

## Source
Lines 31–39 in `crates/oxide-erc/src/context.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [context](/crates/oxide-erc/src/context.md) |
