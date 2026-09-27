---
okf_version: "0.2"
type: Function
title: field_value_ci_falls_back_to_smallest_key_without_exact_match
description: "With no exact-case match, the fallback must still be a pure function"
resource: crates/oxide-output/src/bom/mod.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-output"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-19T03:55:51Z"
concept_id: crates/oxide-output/src/bom/mod/field_value_ci_falls_back_to_smallest_key_without_exact_match
language: rust
---

# field_value_ci_falls_back_to_smallest_key_without_exact_match

With no exact-case match, the fallback must still be a pure function

## Signature

```rust
fn field_value_ci_falls_back_to_smallest_key_without_exact_match()
```

## Decorators

- `test`

## Docstring

With no exact-case match, the fallback must still be a pure function
of the map's contents (lexicographically smallest key), not of hash
order.
[test]

## Source
Lines 620–627 in `crates/oxide-output/src/bom/mod.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [bom](/crates/oxide-output/src/bom/mod.md) |
