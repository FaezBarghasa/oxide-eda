---
okf_version: "0.2"
type: Function
title: compile
description: Compile validated AST rules into executable evaluator closures.
resource: crates/oxide-erc-dsl/src/lib.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-erc-dsl"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:49Z"
concept_id: crates/oxide-erc-dsl/src/lib/compile
language: rust
---

# compile

Compile validated AST rules into executable evaluator closures.

## Signature

```rust
pub fn compile(rules: &[RuleAst]) -> Result<Vec<CompiledRule>, Vec<DslError>>
```

## Visibility

- `pub`

## Docstring

Compile validated AST rules into executable evaluator closures.

## Source
Lines 32–34 in `crates/oxide-erc-dsl/src/lib.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [lib](/crates/oxide-erc-dsl/src/lib.md) |
| called_by | [parse_validate_compile](/crates/oxide-erc-dsl/src/lib/parse_validate_compile.md) |
