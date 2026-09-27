---
okf_version: "0.2"
type: Function
title: bake_chamfered_rejects_united_ratio
description: "A chamfer ratio carrying a physical unit is an error, not a silent"
resource: crates/oxide-bake/tests/bake_pads.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-bake"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:49Z"
concept_id: crates/oxide-bake/tests/bake_pads/bake_chamfered_rejects_united_ratio
language: rust
---

# bake_chamfered_rejects_united_ratio

A chamfer ratio carrying a physical unit is an error, not a silent

## Signature

```rust
fn bake_chamfered_rejects_united_ratio()
```

## Decorators

- `test`

## Docstring

A chamfer ratio carrying a physical unit is an error, not a silent
clamp: `"20um"` used to bake `chamfer_ratio == 0.5` with no warning.
[test]

## Source
Lines 382–399 in `crates/oxide-bake/tests/bake_pads.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [bake_pads](/crates/oxide-bake/tests/bake_pads.md) |
| calls | [smd_rect_pad](/crates/oxide-bake/tests/bake_pads/smd_rect_pad.md) |
| calls | [solve](/crates/oxide-bake/tests/bake_pads/solve.md) |
| calls | [bake_pads](/crates/oxide-bake/src/pad/bake_pads.md) |
