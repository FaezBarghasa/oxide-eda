---
okf_version: "0.2"
type: Function
title: field_value_ci_prefers_exact_case_on_collision
description: "A symbol carrying both `Fitted` and `fitted` used to resolve by"
resource: crates/oxide-output/src/bom/mod.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-output"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-19T03:55:51Z"
concept_id: crates/oxide-output/src/bom/mod/field_value_ci_prefers_exact_case_on_collision
language: rust
---

# field_value_ci_prefers_exact_case_on_collision

A symbol carrying both `Fitted` and `fitted` used to resolve by

## Signature

```rust
fn field_value_ci_prefers_exact_case_on_collision()
```

## Decorators

- `test`

## Docstring

A symbol carrying both `Fitted` and `fitted` used to resolve by
whichever key the `HashMap` handed back first. The exact-case match
must win, deterministically. Rebuilt per iteration because
`RandomState` re-seeds per map, not per process.
[test]

## Source
Lines 607–614 in `crates/oxide-output/src/bom/mod.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [bom](/crates/oxide-output/src/bom/mod.md) |
