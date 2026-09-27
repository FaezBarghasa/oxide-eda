---
okf_version: "0.2"
type: Function
title: run_with_dsl
description: Run built-in ERC rules plus caller-provided DSL evaluator functions.
resource: crates/oxide-erc/src/lib.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-erc"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-13T09:33:26Z"
concept_id: crates/oxide-erc/src/lib/run_with_dsl
language: rust
---

# run_with_dsl

Run built-in ERC rules plus caller-provided DSL evaluator functions.

## Signature

```rust
pub fn run_with_dsl(snapshot: &SchematicSheet, dsl_rules: &[engine::EvalFn]) -> Vec<Violation>
```

## Visibility

- `pub`

## Docstring

Run built-in ERC rules plus caller-provided DSL evaluator functions.

## Source
Lines 140–146 in `crates/oxide-erc/src/lib.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [lib](/crates/oxide-erc/src/lib.md) |
| calls | [run_all_with_dsl](/crates/oxide-erc/src/engine/run_all_with_dsl.md) |
