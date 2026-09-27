---
okf_version: "0.2"
type: Function
title: eval_dimensionless
resource: crates/oxide-bake/src/vscore.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-bake"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-13T09:33:26Z"
concept_id: crates/oxide-bake/src/vscore/eval_dimensionless
language: rust
---

# eval_dimensionless

## Signature

```rust
fn eval_dimensionless(expr: &str, ctx: &EvalContext) -> Result<f64, String>
```

## Source
Lines 150–156 in `crates/oxide-bake/src/vscore.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [vscore](/crates/oxide-bake/src/vscore.md) |
| called_by | [bake_v_scores](/crates/oxide-bake/src/vscore/bake_v_scores.md) |
