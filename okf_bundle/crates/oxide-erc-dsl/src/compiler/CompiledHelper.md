---
okf_version: "0.2"
type: Class
title: CompiledHelper
description: "A compiled helper call. For regex-capable helpers, regexes are compiled once."
resource: crates/oxide-erc-dsl/src/compiler.rs
tags:
  - "lang:rust"
  - "type:Class"
  - "module:crates"
  - "domain:oxide-erc-dsl"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-13T09:33:26Z"
concept_id: crates/oxide-erc-dsl/src/compiler/CompiledHelper
language: rust
---

# CompiledHelper

A compiled helper call. For regex-capable helpers, regexes are compiled once.

## Signature

```rust
pub struct CompiledHelper
```

## Decorators

- `derive(Debug, Clone)`

## Visibility

- `pub`

## Docstring

A compiled helper call. For regex-capable helpers, regexes are compiled once.
[derive(Debug, Clone)]

## Methods

- `name`
- `args`
- `regex_arg`

## Source
Lines 18–22 in `crates/oxide-erc-dsl/src/compiler.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [compiler](/crates/oxide-erc-dsl/src/compiler.md) |
