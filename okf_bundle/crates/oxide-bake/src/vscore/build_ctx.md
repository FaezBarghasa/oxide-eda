---
okf_version: "0.2"
type: Function
title: build_ctx
resource: crates/oxide-bake/src/vscore.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-bake"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-13T09:33:26Z"
concept_id: crates/oxide-bake/src/vscore/build_ctx
language: rust
---

# build_ctx

## Signature

```rust
fn build_ctx(params_canonical: &HashMap<String, f64>) -> EvalContext
```

## Source
Lines 139–148 in `crates/oxide-bake/src/vscore.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [vscore](/crates/oxide-bake/src/vscore.md) |
| called_by | [bake_v_scores](/crates/oxide-bake/src/vscore/bake_v_scores.md) |
