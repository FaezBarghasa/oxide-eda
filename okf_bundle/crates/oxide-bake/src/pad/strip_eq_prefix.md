---
okf_version: "0.2"
type: Function
title: strip_eq_prefix
description: "Strip the optional Altium-style leading `=` and surrounding"
resource: crates/oxide-bake/src/pad.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-bake"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-bake/src/pad/strip_eq_prefix
language: rust
---

# strip_eq_prefix

Strip the optional Altium-style leading `=` and surrounding

## Signature

```rust
fn strip_eq_prefix(src: &str) -> &str
```

## Docstring

Strip the optional Altium-style leading `=` and surrounding
whitespace so authored expressions like `= pad_w` parse cleanly.
Matches the convention used by [`oxide_sketch::parameter`] and
[`oxide_sketch::solver::residual::resolve_dim`].

## Source
Lines 283–286 in `crates/oxide-bake/src/pad.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [pad](/crates/oxide-bake/src/pad.md) |
| called_by | [bake_shape](/crates/oxide-bake/src/pad/bake_shape.md) |
| called_by | [eval_mm](/crates/oxide-bake/src/pad/eval_mm.md) |
| called_by | [rotation_deg](/crates/oxide-bake/src/pad/rotation_deg.md) |
