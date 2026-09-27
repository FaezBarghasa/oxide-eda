---
okf_version: "0.2"
type: Function
title: read_parenthesized
resource: crates/oxide-types/src/markup.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-types"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-types/src/markup/read_parenthesized
language: rust
---

# read_parenthesized

## Signature

```rust
fn read_parenthesized(input: &str, start_index: usize) -> Option<(&str, usize)>
```

## Source
Lines 473–491 in `crates/oxide-types/src/markup.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [markup](/crates/oxide-types/src/markup.md) |
| called_by | [evaluate_expressions](/crates/oxide-types/src/markup/evaluate_expressions.md) |
