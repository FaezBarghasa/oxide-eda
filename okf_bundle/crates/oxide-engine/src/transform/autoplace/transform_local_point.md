---
okf_version: "0.2"
type: Function
title: transform_local_point
description: "Apply a symbol instance's position, rotation, and mirror to a"
resource: crates/oxide-engine/src/transform/autoplace.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-engine"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-13T09:33:26Z"
concept_id: crates/oxide-engine/src/transform/autoplace/transform_local_point
language: rust
---

# transform_local_point

Apply a symbol instance's position, rotation, and mirror to a

## Signature

```rust
fn transform_local_point(sym: &oxide_types::schematic::Symbol, lx: f64, ly: f64) -> (f64, f64)
```

## Docstring

Apply a symbol instance's position, rotation, and mirror to a
library-space point, returning world-space coordinates.

HI-19: thin wrapper over the shared `SymbolTransform::apply` so the
math lives in exactly one place (`oxide-types::schematic`). Kept
as a free function so existing call sites that pass `(lx, ly)`
don't need to reshape into `Point`.

## Source
Lines 248–252 in `crates/oxide-engine/src/transform/autoplace.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [autoplace](/crates/oxide-engine/src/transform/autoplace.md) |
| called_by | [autoplace_fields](/crates/oxide-engine/src/transform/autoplace/autoplace_fields.md) |
