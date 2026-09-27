---
okf_version: "0.2"
type: Function
title: rotation_deg
resource: crates/oxide-bake/src/pad.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-bake"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-bake/src/pad/rotation_deg
language: rust
---

# rotation_deg

## Signature

```rust
fn rotation_deg(expr: &Option<String>, ctx: &EvalContext) -> Result<f64, SketchError>
```

## Source
Lines 302–318 in `crates/oxide-bake/src/pad.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [pad](/crates/oxide-bake/src/pad.md) |
| calls | [strip_eq_prefix](/crates/oxide-bake/src/pad/strip_eq_prefix.md) |
| called_by | [bake_one_pad](/crates/oxide-bake/src/pad/bake_one_pad.md) |
