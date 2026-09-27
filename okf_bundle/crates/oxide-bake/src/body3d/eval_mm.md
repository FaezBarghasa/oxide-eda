---
okf_version: "0.2"
type: Function
title: eval_mm
resource: crates/oxide-bake/src/body3d.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-bake"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:49Z"
concept_id: crates/oxide-bake/src/body3d/eval_mm
language: rust
---

# eval_mm

## Signature

```rust
fn eval_mm(expr: &str, ctx: &EvalContext) -> Result<f64, String>
```

## Source
Lines 146–152 in `crates/oxide-bake/src/body3d.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [body3d](/crates/oxide-bake/src/body3d.md) |
| called_by | [bake_body3d](/crates/oxide-bake/src/body3d/bake_body3d.md) |
