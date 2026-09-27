---
okf_version: "0.2"
type: Function
title: opt_eval_mm
resource: crates/oxide-bake/src/cutout.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-bake"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:49Z"
concept_id: crates/oxide-bake/src/cutout/opt_eval_mm
language: rust
---

# opt_eval_mm

## Signature

```rust
fn opt_eval_mm(expr: &Option<String>, ctx: &EvalContext) -> Result<Option<f64>, String>
```

## Source
Lines 107–117 in `crates/oxide-bake/src/cutout.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [cutout](/crates/oxide-bake/src/cutout.md) |
| called_by | [bake_cutouts](/crates/oxide-bake/src/cutout/bake_cutouts.md) |
