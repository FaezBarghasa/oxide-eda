---
okf_version: "0.2"
type: Function
title: field_value_ci
description: "Case-insensitive field lookup, deterministic on a case collision."
resource: crates/oxide-output/src/bom/mod.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-output"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-19T03:55:51Z"
concept_id: crates/oxide-output/src/bom/mod/field_value_ci
language: rust
---

# field_value_ci

Case-insensitive field lookup, deterministic on a case collision.

## Signature

```rust
fn field_value_ci(
    fields: &'a std::collections::HashMap<String, String>,
    key: &str,
) -> Option<&'a str>
```

## Type Parameters

- `'a`

## Docstring

Case-insensitive field lookup, deterministic on a case collision.

`fields` is a `HashMap`, so naively taking the first case-insensitive
match via `.iter().find()` depends on hash-iteration order — a symbol
carrying both `Fitted` and `fitted` with contradictory values could
resolve either way, run to run. Prefer the exact-case match when one
exists (the unambiguous, intended lookup); otherwise fall back to the
case-insensitive match with the lexicographically smallest key, which is
a pure function of the map's contents rather than of the allocator's
`RandomState` seed.

## Source
Lines 349–361 in `crates/oxide-output/src/bom/mod.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [bom](/crates/oxide-output/src/bom/mod.md) |
| called_by | [resolve_base_variant_fitted](/crates/oxide-output/src/bom/mod/resolve_base_variant_fitted.md) |
