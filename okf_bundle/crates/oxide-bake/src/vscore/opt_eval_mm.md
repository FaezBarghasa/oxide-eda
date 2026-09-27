---
okf_version: "0.2"
type: Function
title: opt_eval_mm
resource: crates/oxide-bake/src/vscore.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-bake"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-13T09:33:26Z"
concept_id: crates/oxide-bake/src/vscore/opt_eval_mm
language: rust
---

# opt_eval_mm

## Signature

```rust
fn opt_eval_mm(expr: &Option<String>, ctx: &EvalContext) -> Result<Option<f64>, String>
```

## Source
Lines 127–137 in `crates/oxide-bake/src/vscore.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [vscore](/crates/oxide-bake/src/vscore.md) |
| called_by | [bake_v_scores](/crates/oxide-bake/src/vscore/bake_v_scores.md) |
