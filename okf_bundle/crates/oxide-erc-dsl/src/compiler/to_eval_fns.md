---
okf_version: "0.2"
type: Function
title: to_eval_fns
description: Convert compiled rules into engine evaluator closures.
resource: crates/oxide-erc-dsl/src/compiler.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-erc-dsl"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-13T09:33:26Z"
concept_id: crates/oxide-erc-dsl/src/compiler/to_eval_fns
language: rust
---

# to_eval_fns

Convert compiled rules into engine evaluator closures.

## Signature

```rust
pub fn to_eval_fns(rules: &[CompiledRule]) -> Vec<EvalFn>
```

## Visibility

- `pub`

## Docstring

Convert compiled rules into engine evaluator closures.

## Source
Lines 80–82 in `crates/oxide-erc-dsl/src/compiler.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [compiler](/crates/oxide-erc-dsl/src/compiler.md) |
| called_by | [parse_validate_compile_to_eval_fns](/crates/oxide-erc-dsl/src/lib/parse_validate_compile_to_eval_fns.md) |
