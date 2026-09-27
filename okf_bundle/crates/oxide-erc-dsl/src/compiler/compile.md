---
okf_version: "0.2"
type: Function
title: compile
description: Compile all rules. Continues compiling independent rules and returns all
resource: crates/oxide-erc-dsl/src/compiler.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-erc-dsl"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-13T09:33:26Z"
concept_id: crates/oxide-erc-dsl/src/compiler/compile
language: rust
---

# compile

Compile all rules. Continues compiling independent rules and returns all

## Signature

```rust
pub fn compile(rules: &[RuleAst]) -> Result<Vec<CompiledRule>, Vec<DslError>>
```

## Visibility

- `pub`

## Docstring

Compile all rules. Continues compiling independent rules and returns all
compile-time errors (e.g. invalid regexes) together.

## Source
Lines 61–77 in `crates/oxide-erc-dsl/src/compiler.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [compiler](/crates/oxide-erc-dsl/src/compiler.md) |
| calls | [compile_rule](/crates/oxide-erc-dsl/src/compiler/compile_rule.md) |
