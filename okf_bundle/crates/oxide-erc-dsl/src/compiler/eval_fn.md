---
okf_version: "0.2"
type: Function
title: eval_fn
description: "Clones the evaluator closure for use with `engine::run_all_with_dsl`."
resource: crates/oxide-erc-dsl/src/compiler.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-erc-dsl"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-13T09:33:26Z"
concept_id: crates/oxide-erc-dsl/src/compiler/eval_fn
language: rust
---

# eval_fn

Clones the evaluator closure for use with `engine::run_all_with_dsl`.

## Signature

```rust
impl CompiledRule { pub fn eval_fn(&self) -> EvalFn }
```

## Visibility

- `pub`

## Docstring

Clones the evaluator closure for use with `engine::run_all_with_dsl`.

## Source
Lines 54–56 in `crates/oxide-erc-dsl/src/compiler.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [compiler](/crates/oxide-erc-dsl/src/compiler.md) |
