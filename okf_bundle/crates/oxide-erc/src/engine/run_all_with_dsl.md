---
okf_version: "0.2"
type: Function
title: run_all_with_dsl
description: "Run built-in rules **and** any DSL-compiled rules in a single pass."
resource: crates/oxide-erc/src/engine.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-erc"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:04:17Z"
concept_id: crates/oxide-erc/src/engine/run_all_with_dsl
language: rust
---

# run_all_with_dsl

Run built-in rules **and** any DSL-compiled rules in a single pass.

## Signature

```rust
pub fn run_all_with_dsl(ctx: &ErcContext, dsl_rules: &[EvalFn]) -> Vec<Diagnostic>
```

## Visibility

- `pub`

## Docstring

Run built-in rules **and** any DSL-compiled rules in a single pass.

## Source
Lines 36–42 in `crates/oxide-erc/src/engine.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [engine](/crates/oxide-erc/src/engine.md) |
| calls | [run_all](/crates/oxide-erc/src/engine/run_all.md) |
| called_by | [run_with_dsl](/crates/oxide-erc/src/lib/run_with_dsl.md) |
| called_by | [run_with_project_and_dsl](/crates/oxide-erc/src/lib/run_with_project_and_dsl.md) |
