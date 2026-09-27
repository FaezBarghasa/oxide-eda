---
okf_version: "0.2"
type: Function
title: opt_eval_mm
resource: crates/oxide-bake/src/pour.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-bake"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:49Z"
concept_id: crates/oxide-bake/src/pour/opt_eval_mm
language: rust
---

# opt_eval_mm

## Signature

```rust
fn opt_eval_mm(expr: &Option<String>, ctx: &EvalContext) -> Result<Option<f64>, String>
```

## Source
Lines 127–137 in `crates/oxide-bake/src/pour.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [pour](/crates/oxide-bake/src/pour.md) |
| called_by | [bake_pours](/crates/oxide-bake/src/pour/bake_pours.md) |
