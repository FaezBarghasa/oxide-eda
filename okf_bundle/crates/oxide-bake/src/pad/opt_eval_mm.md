---
okf_version: "0.2"
type: Function
title: opt_eval_mm
resource: crates/oxide-bake/src/pad.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-bake"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-bake/src/pad/opt_eval_mm
language: rust
---

# opt_eval_mm

## Signature

```rust
fn opt_eval_mm(expr: &Option<String>, ctx: &EvalContext) -> Result<Option<f64>, SketchError>
```

## Source
Lines 295–300 in `crates/oxide-bake/src/pad.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [pad](/crates/oxide-bake/src/pad.md) |
| calls | [eval_mm](/crates/oxide-bake/src/pad/eval_mm.md) |
| called_by | [bake_one_pad](/crates/oxide-bake/src/pad/bake_one_pad.md) |
