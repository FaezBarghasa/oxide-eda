---
okf_version: "0.2"
type: Function
title: build_ctx
resource: crates/oxide-bake/src/cutout.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-bake"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:49Z"
concept_id: crates/oxide-bake/src/cutout/build_ctx
language: rust
---

# build_ctx

## Signature

```rust
fn build_ctx(params_canonical: &HashMap<String, f64>) -> EvalContext
```

## Source
Lines 96–105 in `crates/oxide-bake/src/cutout.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [cutout](/crates/oxide-bake/src/cutout.md) |
| called_by | [bake_cutouts](/crates/oxide-bake/src/cutout/bake_cutouts.md) |
