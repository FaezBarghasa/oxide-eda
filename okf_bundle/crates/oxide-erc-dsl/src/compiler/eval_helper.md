---
okf_version: "0.2"
type: Function
title: eval_helper
resource: crates/oxide-erc-dsl/src/compiler.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-erc-dsl"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-13T09:33:26Z"
concept_id: crates/oxide-erc-dsl/src/compiler/eval_helper
language: rust
---

# eval_helper

## Signature

```rust
fn eval_helper(helper: &CompiledHelper, subject: Subject<'_>) -> bool
```

## Source
Lines 284–309 in `crates/oxide-erc-dsl/src/compiler.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [compiler](/crates/oxide-erc-dsl/src/compiler.md) |
| calls | [parse_pin_type](/crates/oxide-erc-dsl/src/compiler/parse_pin_type.md) |
| calls | [normalize](/crates/oxide-erc-dsl/src/compiler/normalize.md) |
| calls | [is_driving_pin](/crates/oxide-erc-dsl/src/compiler/is_driving_pin.md) |
| called_by | [eval_expr](/crates/oxide-erc-dsl/src/compiler/eval_expr.md) |
